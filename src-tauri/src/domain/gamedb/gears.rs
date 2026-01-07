use serde::{Deserialize, Serialize};

use crate::domain::combat_context::action::EffectType;
use crate::domain::types::data::StatType;
use crate::domain::types::id::{GearID, GearSetID};
use crate::domain::types::trigger::TriggerCondition;

use super::skills::SkillEventMs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GearConfig {
    // 基础信息
    pub gear_id: GearID,     // 唯一标识id
    pub gear_type: GearType, // 装备类型
    pub rarity: u8,          // 星级
    pub level: u8,           // 等级
    pub artifice: bool,      // 可否精炼
    pub icon_path: String,   // 图标地址

    // 套装信息
    pub set_id: GearSetID, // 所属套装id

    // 基础数据（0~3精炼）
    pub attr_from_0_to_3: Vec<GearAttributes>, // 0~3精炼属性数据

    // 省略后端信息：name(装备名), region(区域), source(来源)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearSetConfig {
    // 基础信息
    pub set_id: GearSetID,          // 唯一标识id
    pub desc_params: Vec<f64>,      // 技能效果数值表
    pub effects: Vec<SkillEventMs>, // 触发效果

    // 可选信息
    pub trigger_conditions: Option<Vec<TriggerCondition>>, // 触发条件
    pub fixed_bonus: Option<EffectType>,                   // 固定加成

    // 省略后端信息：name(套装名), description(描述)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GearType {
    Armor,  // 衣服
    Gloves, // 手套
    Kit,    // 配件
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearAttributes {
    pub stat_type: StatType,
    pub value: [f64; 4], // 0~3数值
}
