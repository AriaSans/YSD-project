use serde::{Deserialize, Serialize};

use crate::domain::setting::{AppSetting};
use crate::domain::types::data::{DynamicValue, InflictionType};
use crate::domain::types::id::{BuffID, InstanceID, SlotIndex};
use crate::domain::types::statusbuff::enemy::StatusEnemy;
use crate::domain::types::tick::{Tick, TimeMs};
use crate::domain::types::trigger::TriggerTag;

pub struct Action {
    pub uid: InstanceID,
    pub from_skill_instance: InstanceID,
    pub from_slot_index: SlotIndex,
    pub tick_stamp: Tick,
    pub effect: EffectType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EffectType<T = Tick> {
    // 伤害
    Damage(f64),

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
        value: DynamicValue,
        tag: TriggerTag,
    },

    // 增加我方buff
    AddActiveBuff {
        buff_id: BuffID,
        duration: T,
        value: DynamicValue,
        tag: TriggerTag,
    },

    // 回复资源
    GrantSp(DynamicValue),
    GrantEnergy {
        value: DynamicValue,
        slot_index: SlotIndex,
    },
}

impl EffectType<TimeMs> {
    pub fn into_tick(self, setting: &AppSetting) -> EffectType<Tick> {
        match self {
            // 转tick
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
                value,
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
                value,
                tag,
            },

            // 不转tick
            EffectType::Damage(s) => EffectType::Damage(s),
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
            EffectType::GrantSp(a) => EffectType::GrantSp(a),
            EffectType::GrantEnergy { value, slot_index } => {
                EffectType::GrantEnergy { value, slot_index }
            }
        }
    }
}
