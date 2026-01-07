use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    // 环境初始设置
    pub init_sp: f64,
}

impl EnvironmentSetting {
    pub fn new()->Self {
        Self { 
            init_sp: 200.0
         }
    }
}