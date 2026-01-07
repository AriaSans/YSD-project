pub struct AppSetting {
    pub ms_per_tick: i64,            // 每tick多少ms
    pub freezetime_combo_ms: i64,    // 连携技冻结时间
    pub freezetime_ultimate_ms: i64, // 终极技冻结时间
    pub sp_per_ms: f64,                // 每秒恢复多少sp
}

impl AppSetting {
    pub fn new() -> Self {
        Self {
            ms_per_tick: 50,
            freezetime_combo_ms: 300,
            freezetime_ultimate_ms: 500,
            sp_per_ms: 0.008,
        }
    }

    pub fn get_sp_per_tick(&self) -> f64 {
        self.sp_per_ms * (self.ms_per_tick as f64)
    }
}
