use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::gamedb::skills::SkillConfig;
use crate::domain::setting::AppSetting;
use crate::domain::types::data::{DynamicValue, InflictionType, TargetSelector};
use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::{BuffID, InstanceID, SkillID, SlotIndex};
use crate::domain::types::statusbuff::enemy::StatusEnemy;
use crate::domain::types::tick::{Tick, TimeMs};
use crate::domain::types::trigger::{TriggerCondition, TriggerTag};

use super::runtime_skill_config::RuntimeSkillConfig;

// 代表一个瞬间发生的具体影响（日志，前端看）
#[derive(Debug, Serialize, Deserialize)]
pub struct CombatEvent {
    pub tick: Tick,                    // 发生的绝对时间
    pub source_action_uid: InstanceID, // 来自哪个动作
    pub source_slot: SlotIndex,

    // 具体的 payload
    pub effect: EffectType,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub uid: InstanceID,                 // 唯一实例id
    pub from_skill_instance: InstanceID, // 来自技能实例id
    pub from_slot_index: SlotIndex,      // 来自干员位

    // 引用静态配置，获取duration, events, interrupt_windows
    pub config: Arc<RuntimeSkillConfig>,

    // === 动态状态(State) ===
    pub start_tick: Tick,   // 动作开始的绝对时间
    pub hitstop_debt: Tick, // 累计的卡肉时长, 出现卡肉事件是压入卡肉时间，每经过1tick减1

    // 进度标记：
    // 记录下一个触发第几个event
    pub next_event_index: usize,

    // 状态标记（用于打断判断, 依附角色）
    pub state_flags: ActionFlags,
    pub is_attached_to_model: bool,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct ActionFlags:u8 {
        const DAMAGE_DEALT = 1 << 0;
        const INTERRUPTED = 1 << 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EffectType<T = Tick, V = DynamicValue<Fixed>> {
    // === 攻击类 ===
    // 伤害
    Damage {
        value: V,
        stagger: Option<V>,
        hitstop: T,
    },
    // 重击
    FinalStrike,

    // === 敌人状态类 ===
    // 尝试消耗层数
    TryConsumeInflication {
        inflication_type: InflictionType,
        tag: TriggerTag,
    },
    // 增加异常层数
    AddInfliction {
        inflication_type: InflictionType,
        tag: TriggerTag,
    },
    // 增加敌人状态
    AddStatusEnemy {
        status: StatusEnemy,
        duration: T,
        tag: TriggerTag,
    },
    // 增加敌人debuff
    AddDeBuffEnemy {
        debuff_id: BuffID,
        duration: T,
        value: V,
        tag: TriggerTag,
    },

    // === 友方增益/资源类 ===
    // 增加我方buff
    AddActiveBuff {
        target: TargetSelector,
        buff_id: BuffID,
        duration: T,
        value: V,
        tag: TriggerTag,
    },
    // 回复SP
    GrantSp {
        target: TargetSelector,
        value: V,
    },
    // 回复Energy
    GrantEnergy {
        target: TargetSelector,
        value: V,
    },

    // === 机制/控制类 ===
    // 冻结时间
    WorldFreeze(T),
    // 切换主控
    SwitchTo(SlotIndex),

    // === 逻辑控制 ===
    // 分支
    Branch {
        condition: TriggerCondition,
        success: Vec<EffectType<T, V>>,
        failure: Option<Vec<EffectType<T, V>>>,
    },
    // 生成子技能
    SpawnChildSkill {
        skill_id: SkillID,
        delay: T,
        inherit_stats: bool,
    },
}

impl EffectType<TimeMs, DynamicValue<f64>> {
    pub fn convert(self, setting: &AppSetting) -> EffectType<Tick, DynamicValue<Fixed>> {
        match self {
            // 转tick, fixed
            EffectType::WorldFreeze(ms) => EffectType::WorldFreeze(ms.to_tick(setting)),
            EffectType::AddStatusEnemy {
                status,
                duration,
                tag,
            } => EffectType::AddStatusEnemy {
                status,
                duration: duration.to_tick(setting),
                tag,
            },

            EffectType::AddDeBuffEnemy {
                debuff_id,
                duration,
                value,
                tag,
            } => EffectType::AddDeBuffEnemy {
                debuff_id,
                duration: duration.to_tick(setting),
                value: value.into_fixed(),
                tag,
            },

            EffectType::AddActiveBuff {
                target,
                buff_id,
                duration,
                value,
                tag,
            } => EffectType::AddActiveBuff {
                target,
                buff_id,
                duration: duration.to_tick(setting),
                value: value.into_fixed(),
                tag,
            },
            EffectType::Damage {
                value,
                stagger,
                hitstop,
            } => EffectType::Damage {
                value: value.into_fixed(),
                stagger: stagger.map(|d| d.into_fixed()),
                hitstop: hitstop.to_tick(setting),
            },
            EffectType::GrantSp { target, value } => EffectType::GrantSp {
                target,
                value: value.into_fixed(),
            },
            EffectType::Branch {
                condition,
                success,
                failure,
            } => EffectType::Branch {
                condition,
                success: success.into_iter().map(|e| e.convert(setting)).collect(),
                failure: failure.map(|f| f.into_iter().map(|e| e.convert(setting)).collect()),
            },
            EffectType::SpawnChildSkill {
                skill_id,
                delay,
                inherit_stats,
            } => EffectType::SpawnChildSkill {
                skill_id,
                delay: delay.to_tick(setting),
                inherit_stats,
            },

            // 不转tick, fixed
            EffectType::SwitchTo(s) => EffectType::SwitchTo(s),
            EffectType::FinalStrike => EffectType::FinalStrike,
            EffectType::TryConsumeInflication {
                inflication_type,
                tag,
            } => EffectType::TryConsumeInflication {
                inflication_type,
                tag,
            },
            EffectType::AddInfliction {
                inflication_type,
                tag,
            } => EffectType::AddInfliction {
                inflication_type,
                tag,
            },
            EffectType::GrantEnergy { target, value } => EffectType::GrantEnergy {
                target,
                value: value.into_fixed(),
            },
        }
    }
}
