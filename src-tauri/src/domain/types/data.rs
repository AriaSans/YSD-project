use serde::{Deserialize, Serialize};

use crate::domain::gamedb::skills::SkillConfig;
use crate::domain::setting::AppSetting;

use super::fixed::Fixed;
use super::tick::{ExpireTick, Tick};

// sp技力,x1000存储(Fixed)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpState {
    pub base_value: Fixed,
    pub last_update_tick: Tick,
}

// 能量,x1000存储(Fixed)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Energy {
    pub value: Fixed,
    pub max: Fixed,
}

// 失衡值,x1000存储(Fixed)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stagger {
    pub value: Fixed,
    pub max: Fixed,
}

// 异常层数
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct InflictionStacks(pub u8);

// 特殊资源
#[derive(Clone, Copy, Debug)]
pub enum SpecialResource {
    None,
    Point(u8),                                     // 离散永存资源
    Gauge { current: f64, max: f64 },              // 连续永存资源(不自长)
    TimeStacks { stacks: u8, expire: ExpireTick }, // 叠加限时资源
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    Physical,
    Heat,
    Electirc,
    Cryo,
    Nature,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InflictionType {
    Physical,
    Heat,
    Electirc,
    Cryo,
    Nature,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffixTier {
    Small,  // -小
    Medium, // -中
    Large,  // -大
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicValue {
    Fix(f64),             // 固定数
    Ref { param: usize }, // Vec<>中的第几个参数
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StatType {
    Atk,              // 攻击力
    Hp,               // 血量
    Def,              // 防御力
    Str,              // 力量
    Agl,              // 敏捷
    Int,              // 智识
    Wil,              // 意志
    UltimateGainEff,  // 终结技能量获取效率
    ComboCDReduction, // 连携技冷却时间缩减
}

impl SpState {
    // 取当前tick的sp值，结果为x1000的Fixed
    pub fn current_value(&self, current_tick: Tick, setting: &AppSetting) -> Fixed {
        let rate = Fixed::from_float(setting.get_sp_per_tick());
        let passed = Fixed::from_int(current_tick.0 - self.last_update_tick.0);
        passed.mul(rate)
    }
}

impl InflictionStacks {
    pub fn add(&self) -> Self {
        if self.0 == 4 {
            Self(4)
        } else {
            Self(self.0 + 1)
        }
    }

    pub fn clear(&self) -> Self {
        Self(0)
    }
}

impl DynamicValue {
    pub fn resolve(&self, skill_level: u8, config: &SkillConfig) -> f64 {
        match self {
            DynamicValue::Fix(val) => *val,
            DynamicValue::Ref { param } => {
                // 防越界，等级-1为参数
                let level_index = (skill_level as usize).saturating_sub(1);

                config
                    .params
                    .get(level_index)
                    .and_then(|level_params| level_params.get(*param))
                    .copied() // 解引用
                    .unwrap_or(0.0)
            }
        }
    }
}
