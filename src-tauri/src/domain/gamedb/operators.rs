use serde::{Deserialize, Serialize};

use crate::domain::types::data::{Element, StatType};
use crate::domain::types::id::{OperatorID, SkillID, TalentID};

use super::weapons::WeaponType;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperatorConfig {
    // 干员基础信息
    pub oper_id: OperatorID,    // 唯一id
    pub star: u8,               // 星级
    pub class: OperatorClass,   // 职业
    pub element: Element,       // 元素
    pub tags: Vec<OperatorTag>, // 干员标签
    pub wpn_type: WeaponType,   // 武器类型
    pub atk_combo_hits: u8,     // 攻击段数
    pub icon_path_oper: String, // 图标地址

    // 基础数值（99级）
    pub hp: i64,              // 生命值
    pub atk: i64,             // 攻击力
    pub str: i64,             // 力量
    pub agl: i64,             // 敏捷
    pub int: i64,             // 智识
    pub wil: i64,             // 意志
    pub stat_main: StatType,  // 主要属性
    pub stat_minor: StatType, // 副要属性
    // 终极技能量上限
    pub energy_max: i64,

    // 天赋
    pub talents: Vec<Talent>,
    // 对应技能id
    pub basic_attack: SkillID,
    pub battle_skill: SkillID,
    pub combo_skill: SkillID,
    pub ultimate: SkillID,

    // 特殊机制
    pub special_setting: Option<SpecialSetting>,

    // 省略: name
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Talent {
    pub tal_id: TalentID, // 天赋id
    pub level_max: u8,    // 等级上限
    pub icon_path: String, // 图标地址

    // 省略: details，name
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialSetting {
    Point {
        max: u8,
        name: String,
        icon_path: String,
    }, // 离散永存资源
    Gauge {
        max: f64,
        name: String,
        icon_path: String,
    }, // 连续永存资源
    TimeStacksMs {
        duration_ms: i64,
        name: String,
        icon_path: String,
    }, // 叠加限时资源
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OperatorClass {
    Guard,     // 近卫
    Vanguard,  // 先锋
    Caster,    // 术士
    Defender,  // 重装
    Striker,   // 突袭
    Supporter, // 辅助
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OperatorTag {
    // 类别
    DamageDealer, // 输出
    Protect,      // 保护
    HPTreatment,  // 治疗
    SPRecovery,   // 回费
    CrowdControl, // 群控

    // 异常
    HeatInfliction,     // 灼热异常
    ElectricInfliction, // 电磁异常
    CryoInfliction,     // 寒冷异常
    NatureInfliction,   // 自然异常

    // 物理效果
    KnockDown, // 击倒
    Lift,      // 击飞
    Crush,     // 击破
    Breach,    // 碎甲
    Stagger,   // 失衡
    Link,      // 连击

    // 法术效果
    Combust,   // 燃烧
    Electrify, // 导电
    Solidify,  // 冻结
    Corrode,   // 腐蚀

    // 强化
    Amp, // 增幅

    // 易伤
    Susceptibility,         // 脆弱
    ArtsSusceptibility,     // 法术脆弱
    PhysicalSusceptibility, // 物理脆弱

    // 削弱
    Weaken, // 虚弱

    // 其他
    Shield, // 护盾
    Crit,   // 暴击
}
