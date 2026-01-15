use std::collections::HashMap;

use action::{Action, CombatEvent};
use environment::EnvironmentSnapshot;
use error_instance::ErrorInstance;
use runtime_operator::RuntimeOperator;
use runtime_skill_config::RuntimeSkillConfig;
use trigger_listener::TriggerListener;

use crate::domain::types::trigger::TriggerTag;

use super::types::id::{InstanceID, SkillID};
use super::types::tick::Tick;

pub mod action;
pub mod environment;
pub mod error_instance;
pub mod runtime_operator;
pub mod runtime_skill_config;
pub mod trigger_listener;

pub struct CombatContext {
    pub env_snapshots: Vec<EnvironmentSnapshot>,

    pub event_log: Vec<CombatEvent>,                // 执行的event日志
    pub errors: HashMap<InstanceID, ErrorInstance>, // 错误列表<技能实例id， 错误信息>

    // === 静态引用 ===
    pub runtime_operator: Vec<RuntimeOperator>, // 初始人物属性表
    pub trigger_registry: HashMap<TriggerTag, Vec<TriggerListener>>, // 触发器注册表
    pub skill_config_registry: HashMap<SkillID, Vec<RuntimeSkillConfig>>, // 触发器注册表
}

impl CombatContext {
    pub fn new() -> Self {
        let mut env_snapshots = vec![];
        env_snapshots.push(EnvironmentSnapshot::init());

        Self {
            env_snapshots,
            runtime_operator: vec![],
            trigger_registry: HashMap::new(),
            skill_config_registry: HashMap::new(),

            event_log: vec![],
            errors: HashMap::new(),
        }
    }

    // 获取第一张环境
    pub fn first_snapshot(&self) -> &EnvironmentSnapshot {
        self.env_snapshots.first().expect("严重错误：快照列表空")
    }
            

    // 获取最后一张环境
    pub fn latest_snapshot(&self) -> &EnvironmentSnapshot {
        self.env_snapshots.last().expect("严重错误：快照列表空")
    }
}
