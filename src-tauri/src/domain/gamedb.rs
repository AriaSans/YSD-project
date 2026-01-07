use std::collections::HashMap;
use std::sync::Arc;

use buffs::BuffConfig;
use gears::{GearConfig, GearSetConfig};
use operators::OperatorConfig;
use skills::SkillConfig;
use weapons::{WeaponAffixCommon, WeaponConfig};

use super::types::data::AffixTier;
use super::types::id::{BuffID, GearID, GearSetID, OperatorID, SkillID, WeaponID};

pub mod buffs;
pub mod gears;
pub mod operators;
pub mod skills;
pub mod weapons;

pub struct GameDB {
    pub operators: HashMap<OperatorID, Arc<OperatorConfig>>, // 干员表
    pub skills: HashMap<SkillID, SkillConfig>,               // 技能表
    pub weapons: HashMap<WeaponID, WeaponConfig>,            // 武器表
    pub gears: HashMap<GearID, GearConfig>,                  // 装备表
    pub gearsets: HashMap<GearSetID, GearSetConfig>,         // 装备套装表
    pub buffs: HashMap<BuffID, BuffConfig>,                  // buff表

    pub affixes: HashMap<(WeaponAffixCommon, AffixTier), [f64; 9]>, // 武器常驻词条表
}

impl GameDB {
    pub fn link() -> Self {
        Self {
            operators: HashMap::new(),
            skills: HashMap::new(),
            weapons: HashMap::new(),
            gears: HashMap::new(),
            gearsets: HashMap::new(),
            buffs: HashMap::new(),

            affixes: HashMap::new(),
        }
    }
}
