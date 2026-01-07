use environment_setting::EnvironmentSetting;
use operator_instance::OperatorInstance;
use serde::{Deserialize, Serialize};
use skill_instance::SkillInstance;

use super::types::id::InstanceID;
use super::types::tick::Tick;

pub mod dependency;
pub mod environment_setting;
pub mod operator_instance;
pub mod skill_instance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub current_tick: Tick,                      // 当前指针Tick
    pub active_operators: Vec<OperatorInstance>, // 选中的干员（最多4）
    pub skills_timeline: Vec<SkillInstance>,     // 技能时间轴
    pub env_setting: EnvironmentSetting,         // 环境初始信息

    pub next_uid: u64, // 下一个uid（全局分配id）
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            current_tick: Tick(0),
            active_operators: vec![],
            skills_timeline: vec![],
            env_setting: EnvironmentSetting::new(),

            next_uid: 0,
        }
    }

    pub fn alloc_uid(&mut self) -> InstanceID {
        let id = self.next_uid;
        self.next_uid += 1;
        InstanceID(id)
    }
}
