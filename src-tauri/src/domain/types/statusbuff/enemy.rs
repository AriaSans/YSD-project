use crate::domain::types::data::*;
use crate::domain::types::id::{BuffID, SlotIndex};
use crate::domain::types::tick::ExpireTick;
use serde::{Deserialize, Serialize};

use super::operator::BuffPayload;

// 敌人状态(覆盖型)
#[derive(Debug,Clone, Copy,Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum StatusEnemy {
    // 物理异常
    Lift,      // 击飞
    KnockDown, // 倒地

    // 法术异常
    Combustion,      // 燃烧
    Electrification, // 导电
    Solidification,  // 冻结
    Corrosion,       // 腐蚀

    // 其他
    Crystal,   // 晶石化
    Staggered, // 失衡
    Charging,  // 蓄力红圈
    Focus,     // 聚焦(From: Abtal)
}

// 法术异常层
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum ArtsStacks {
    None,
    Heat(InflictionStacks),
    Electric(InflictionStacks),
    Cryo(InflictionStacks),
    Nature(InflictionStacks),
}

// 敌人Debuff，不参与依赖判断
#[derive(Debug, Serialize, Deserialize)]
pub struct DeBuffEnemy{
    pub debuff_id: BuffID,
    pub slotindex: SlotIndex,
    pub expiretick: ExpireTick,
    pub effect: BuffPayload,
}


