use serde::{Serialize, Deserialize};
use crate::domain::types::data::*;
use crate::domain::types::id::{BuffID, SlotIndex};
use crate::domain::types::tick::{ExpireTick, Tick};

// 干员Buff, 不参与状态检测
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveBuff {
    pub buff_id: BuffID,
    pub from_slotindex: SlotIndex,
    pub expiretick: ExpireTick,
    pub payload: BuffPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum BuffPayload {
    Simple, // 简单状态（固定攻击力+10%）
    Stacked{ current: u8, max: u8}, // 层数状态（物理战技强化3层）
    Value(i64), // 数值状态（攻击，护盾，充能300）
    StatModifier {
        stat_type: StatType,
        value: f64,
        is_ratio: bool, // true=百分比, false=固定值
    }   // 属性修改
}

// 同样状态记录
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MechanismState {
    Cd(Tick),   // 冷却计数器
    Counter(i64),// 累计计数器
    Flag(bool)// 触发计数器
}