use std::collections::HashMap;

use action::Action;
use environment::EnvironmentSnapshot;
use runtime_operator::RuntimeOperator;
use trigger_listener::TriggerListener;

use crate::domain::types::trigger::TriggerTag;

pub mod action;
pub mod environment;
pub mod runtime_operator;
pub mod trigger_listener;

pub struct CombatContext {
    pub actions: Vec<Action>,
    pub env_sanpshots: Vec<EnvironmentSnapshot>,

    pub runtime_operator: Vec<RuntimeOperator>, // 初始人物属性表
    pub trigger_registry: HashMap<TriggerTag, Vec<TriggerListener>>, // 触发器注册表
}

impl CombatContext {
    pub fn new() -> Self {
        Self {
            actions: vec![],
            env_sanpshots: vec![],
            runtime_operator: vec![],
            trigger_registry: HashMap::new(),
        }
    }
}
