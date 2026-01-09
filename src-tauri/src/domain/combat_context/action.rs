use serde::{Deserialize, Serialize};

use crate::domain::setting::AppSetting;
use crate::domain::types::data::{DynamicValue, InflictionType};
use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::{BuffID, InstanceID, SlotIndex};
use crate::domain::types::statusbuff::enemy::StatusEnemy;
use crate::domain::types::tick::{Tick, TimeMs};
use crate::domain::types::trigger::TriggerTag;

pub struct Action {
    pub uid: InstanceID,                 // 唯一实例id
    pub from_skill_instance: InstanceID, // 来自技能实例id
    pub from_slot_index: SlotIndex,      // 来自干员位
    pub tick_stamp: Tick,                // 发送tick
    pub effect: EffectType,              // 效果

    pub applied: bool, // 是否已发生
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EffectType<T = Tick, V = DynamicValue<Fixed>> {
    // 伤害
    Damage(V),

    // 冻结时间
    WorldFreeze(T),
    // 切换主控
    SwitchTo(SlotIndex),

    // 重击
    FinalStrike,

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

    // 增加敌人状态/debuff
    AddStatusEnemy {
        status: StatusEnemy,
        duration: T,
        tag: TriggerTag,
    },
    AddDeBuffEnemy {
        debuff_id: BuffID,
        duration: T,
        value: V,
        tag: TriggerTag,
    },

    // 增加我方buff
    AddActiveBuff {
        buff_id: BuffID,
        duration: T,
        value: V,
        tag: TriggerTag,
    },

    // 回复资源
    GrantSp(V),
    GrantEnergy {
        value: V,
        slot_index: SlotIndex,
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
                buff_id,
                duration,
                value,
                tag,
            } => EffectType::AddActiveBuff {
                buff_id,
                duration: duration.to_tick(setting),
                value: value.into_fixed(),
                tag,
            },
            EffectType::Damage(v) => EffectType::Damage(v.into_fixed()),
            EffectType::GrantSp(a) => EffectType::GrantSp(a.into_fixed()),

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
            EffectType::GrantEnergy { value, slot_index } => {
                EffectType::GrantEnergy { value: value.into_fixed(), slot_index }
            }
        }
    }
}
