use serde::{Deserialize, Serialize};

use crate::domain::types::data::AffixTier;
use crate::domain::types::id::{SkillID, WeaponID};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponConfig {
    // 基础信息
    pub wpn_id: WeaponID,     // 唯一标识id
    pub wpn_type: WeaponType, // 武器类型
    pub rarity: u8,           // 星级
    pub icon_path: String,    // 图标地址

    // 词条(前二), 查表获得每级数据
    pub affixes_common: (WeaponAffixDef, WeaponAffixDef),

    // 词条(特殊)，每级数据保存于这
    pub affix_special_skillid: SkillID,
    // 省略:name, description, source
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponAffixDef {
    pub affix_type: WeaponAffixCommon, // 词条名
    pub tier: AffixTier,               // 大中小
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum WeaponType {
    Sword,      // 单手剑
    GreatSword, // 大剑
    Polearm,    // 枪
    Handcannon, // 手铳
    ArtsUnit,   // 法术单元
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeaponAffixCommon {
    // 基础属性增幅
    MainBoost, // 主能力提升
    StrBoost,  // 力量提升
    AglBoost,  // 敏捷提升
    IntBoost,  // 智识提升
    WilBoost,  // 意志提升

    // 附加属性增幅
    AtkBoost,        // 攻击提升(+百分比)
    CriticalRate,    // 暴击率提升(+百分比)
    ArtsIntensity,   // 源石技艺提升(+数值)
    UltimateGainEff, // 终结技效率提升(+百分比)
    HpBoost,         // 生命提升（最大生命值+百分比）
    TreatmentEff,    // 治疗效率提升(+百分比)

    // 伤害提升(百分比)
    ArtsDMGBosst,     // 法术伤害提升
    PhysicalDMGBosst, // 物理伤害提升
    HeatDMGBosst,     // 灼热伤害提升
    ElectricDMGBosst, // 电磁伤害提升
    CryoDMGBosst,     // 寒冷伤害提升
    NatureDMGBosst,   // 自然伤害提升
}
