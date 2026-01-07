use serde::{self, Deserialize, Serialize};

use crate::domain::types::id::BuffID;
use crate::domain::types::trigger::TriggerTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuffConfig {
    // 唯一标识符
    pub buff_id: BuffID,

    // 行为属性
    pub receiver: Receiver, // 敌人或友方
    pub buff_type: BuffType, // Buff, DeBuff, Hidden
    pub max_stacks: u8, // 最大层数（1不叠加）

    // 叠加策略
    pub stack_strategy: StackStrategy,

    // 标签（触发器检测）
    pub tags: Vec<TriggerTag>,

    // 图标
    pub icon_path: String,
    // 省略:name, description

}

#[derive(Debug,Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum BuffType {
    Buff,
    Debuff,
    Hidden, // 实现内部cd、计数器
}

#[derive(Debug,Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum Receiver {
    Enemy,
    Operator,
}

#[derive(Debug,Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum StackStrategy {
    Refresh,     // 刷新
    Extend,      // 延长
    StackCount,  // 叠层并刷新
    Independent, // 独立存在(多图标)
}
