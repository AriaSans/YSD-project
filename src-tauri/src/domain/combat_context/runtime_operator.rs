use crate::domain::types::id::{OperatorID, SlotIndex};
use crate::domain::types::tick::Tick;

pub struct RuntimeOperator {
    // 配置信息
    pub config_id: OperatorID,
    pub slot_index: SlotIndex,

    // 预计算好的面板
    pub atk_base: f64,        // 攻击力
    pub hp_base: f64,         // 生命值
    pub def_base: f64,        // 防御力
    pub critical_ration: f64, // 暴击率
    pub arts_intensity: f64,  // 源石技艺提升

    pub str_base: f64, // 力量
    pub agl_base: f64, // 敏捷
    pub int_base: f64, // 智识
    pub wil_base: f64, // 意志

    pub atk_boost_ration: f64,       // 攻击加成（暂单乘区）
    pub hp_boost_ration: f64,        // 生命提升（百分比）
    pub treatment_boost_ration: f64, // 治疗效率提示（百分比）

    pub arts_boost_ration: f64,     // 法术伤害提升
    pub physical_boost_ration: f64, // 物理伤害提升
    pub heat_boost_ration: f64,     // 灼热伤害提升
    pub electric_boost_ration: f64, // 电磁伤害提升
    pub cyro_boost_ration: f64,     // 寒冷伤害提升
    pub nature_boost_ration: f64,   // 自然伤害提升

    pub ultimate_gain_efficiency: f64, // 终结技效率（百分比）
    pub combo_cd_redution_ration: f64, // 连携技cd减少ratio
    pub combo_cd_redution_tick: Tick,  // 连携技cd减少Tick
}

impl RuntimeOperator {
    pub fn new() -> Self {
        Self {
            config_id: OperatorID("empty42".into()),
            slot_index: SlotIndex(0),
            atk_base: 100.0,
            hp_base: 100.0,
            def_base: 100.0,
            critical_ration: 100.0,
            arts_intensity: 100.0,
            str_base: 100.0,
            agl_base: 100.0,
            int_base: 100.0,
            wil_base: 100.0,
            atk_boost_ration: 100.0,
            hp_boost_ration: 100.0,
            treatment_boost_ration: 100.0,
            arts_boost_ration: 100.0,
            physical_boost_ration: 100.0,
            heat_boost_ration: 100.0,
            electric_boost_ration: 100.0,
            cyro_boost_ration: 100.0,
            nature_boost_ration: 100.0,
            ultimate_gain_efficiency: 100.0,
            combo_cd_redution_ration: 100.0,
            combo_cd_redution_tick: Tick(10),
        }
    }
}
