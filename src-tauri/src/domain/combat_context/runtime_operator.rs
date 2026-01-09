use ustr::Ustr;

use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::{OperatorID, SlotIndex};
use crate::domain::types::tick::Tick;

pub struct RuntimeOperator {
    // 配置信息
    pub config_id: OperatorID,
    pub slot_index: SlotIndex,

    // 预计算好的面板
    pub atk_base: Fixed,        // 攻击力
    pub hp_base: Fixed,         // 生命值
    pub def_base: Fixed,        // 防御力
    pub critical_ration: Fixed, // 暴击率
    pub arts_intensity: Fixed,  // 源石技艺提升

    pub str_base: Fixed, // 力量
    pub agl_base: Fixed, // 敏捷
    pub int_base: Fixed, // 智识
    pub wil_base: Fixed, // 意志

    pub atk_boost_ration: Fixed,       // 攻击加成（暂单乘区）
    pub hp_boost_ration: Fixed,        // 生命提升（百分比）
    pub treatment_boost_ration: Fixed, // 治疗效率提示（百分比）

    pub arts_boost_ration: Fixed,     // 法术伤害提升
    pub physical_boost_ration: Fixed, // 物理伤害提升
    pub heat_boost_ration: Fixed,     // 灼热伤害提升
    pub electric_boost_ration: Fixed, // 电磁伤害提升
    pub cyro_boost_ration: Fixed,     // 寒冷伤害提升
    pub nature_boost_ration: Fixed,   // 自然伤害提升

    pub ultimate_gain_efficiency: Fixed, // 终结技效率（百分比）
    pub combo_cd_redution_ration: Fixed, // 连携技cd减少ratio
    pub combo_cd_redution_tick: Tick,  // 连携技cd减少Tick
}

impl RuntimeOperator {
    pub fn new() -> Self {
        Self {
            config_id: OperatorID(Ustr::from("oper_empty")),
            slot_index: SlotIndex(0),
            atk_base: Fixed::from_int(0),
            hp_base: Fixed::from_int(0),
            def_base: Fixed::from_int(0),
            critical_ration: Fixed::from_int(0),
            arts_intensity: Fixed::from_int(0),
            str_base: Fixed::from_int(0),
            agl_base: Fixed::from_int(0),
            int_base: Fixed::from_int(0),
            wil_base: Fixed::from_int(0),
            atk_boost_ration: Fixed::from_int(0),
            hp_boost_ration: Fixed::from_int(0),
            treatment_boost_ration: Fixed::from_int(0),
            arts_boost_ration: Fixed::from_int(0),
            physical_boost_ration: Fixed::from_int(0),
            heat_boost_ration: Fixed::from_int(0),
            electric_boost_ration: Fixed::from_int(0),
            cyro_boost_ration: Fixed::from_int(0),
            nature_boost_ration: Fixed::from_int(0),
            ultimate_gain_efficiency: Fixed::from_int(0),
            combo_cd_redution_ration: Fixed::from_int(0),
            combo_cd_redution_tick: Tick(10),
        }
    }

    pub fn from_instance() {}
}
