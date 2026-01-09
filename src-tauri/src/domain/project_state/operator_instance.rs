use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::gamedb::operators::OperatorConfig;
use crate::domain::types::id::{GearID, GearSetID, InstanceID, OperatorID, SlotIndex, WeaponID};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorInstance {
    // 基础信息
    pub uid: InstanceID,     // 唯一实例id
    pub oper_id: OperatorID, // 干员id：oper_surtr
    pub slot_index: SlotIndex,

    #[serde(skip)]
    pub config: Option<Arc<OperatorConfig>>, // 干员配置

    // 装备
    pub weapon: Option<WeaponInstance>,       // 武器
    pub gear_armor: Option<GearInstance>,     // 装备-衣服
    pub gear_gloves: Option<GearInstance>,    // 装备-手套
    pub gear_kits: [Option<GearInstance>; 2], // 装备-配件
    pub gear_set: Option<GearSetID>,
    // 自定义配置（角色潜能等）
    pub potential: u8,          // 角色潜能
    pub talents_level: [u8; 2], // 天赋等级
    pub skills_level: [u8; 4],  // 技能等级(1~12)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearInstance {
    pub gear_id: GearID,
    pub artifice: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponInstance {
    pub gear_id: WeaponID,
    pub level: u8,  // 武器等级
    pub affixes_rank:[u8; 3],   // 词条等级（三个）
}
