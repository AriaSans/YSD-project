use serde::{Deserialize, Serialize};

use crate::domain::setting::{AppSetting};

#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Tick(pub i64);

#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ExpireTick(pub i64);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DurationTick(pub i64);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TimeMs(pub i64);

impl Tick {
    pub fn from_ms(ms: i64 ,setting: &AppSetting) -> Self{
        Tick(ms/ setting.ms_per_tick)
    }
    pub fn to_ms(self, setting: &AppSetting) -> TimeMs {
        TimeMs(self.0 * setting.ms_per_tick)
    }
}

impl TimeMs {
    pub fn to_tick(self, setting: &AppSetting) -> Tick {
        Tick(self.0 / setting.ms_per_tick)
    }
}
