use serde::{Deserialize, Serialize};

const SCALE: i64 = 1000;
// 倍数存储计算
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixed(pub i64);

impl Fixed {
    pub fn from_int(val: i64) -> Self{
        Fixed(val * SCALE)
    }
    pub fn from_float(val: f64) -> Self{
        Fixed((val * (SCALE as f64)).round() as i64)
    }
    pub fn raw(self) -> i64 {
        self.0
    }
    pub fn to_int(self) -> i64{
        self.0 / SCALE
    }
    pub fn to_float(self) -> f64{
        (self.0 as f64) / (SCALE as f64)
    }
    pub fn mul(&self, rhs: Self) -> Self {
        Self(self.0 * rhs.0 / SCALE)
    }
    pub fn mul_f64(&self, rhs: f64) -> Self {
        let rate = Fixed((rhs * (SCALE as f64)).round() as i64);
        Self(self.0 * rate.0 / SCALE)
    }
}