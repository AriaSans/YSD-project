use serde::{Deserialize, Serialize};

use super::data::InflictionType;
use super::id::BuffID;
use super::statusbuff::enemy::{StatusEnemy};

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerTag {
    FinalStrike,        // 重击
    Crush,              // 猛击
    Fracture,           // 碎甲

    // 增加敌人状态
    PhysicalInfliction, // 物理异常
    HeatInfliction,     // 灼热异常
    ElectricInfliction, // 电磁异常
    CryoInfliction,     // 寒冷异常
    NatureInfliction,   // 自然异常

    // 增加buff
    Amp,                // 增幅
    Protected,          // 庇护
    Susceptible,        // 脆弱
    Weakened,           // 虚弱
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerCondition {
    // 状态（敌人）
    HaveInflictionStacks {
        inflication_type: InflictionType,
        stacks_min: u8,
    }, // 持有异常层数(以上)
    HaveNoInfliction(InflictionType), // 未持有异常
    HaveStatusEnemy(StatusEnemy),     // 持有状态（敌人）
    ConsumeStatusEnemy(StatusEnemy),  // 消耗状态（敌人）
    GetStatusEnemy(StatusEnemy),      // 进入状态（敌人）

    // 状态（我方）
    CheckHp {
        compare_type: CompareOp,
        value: i64,
    }, // 检测血量less/more, 百分比
    GetActiveStatus(BuffID),     // 施加状态（友方）
    ConsumeActiveStatus(BuffID), // 消耗状态（友方）
    
    // 受击
    Injured,
    
    // tag标记条件
    HaveBuffWithTag(TriggerTag),// 拥有Buff（敌我）
    HitByTag(TriggerTag), // 触发点

    And(Vec<TriggerCondition>),
    Or(Vec<TriggerCondition>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CompareOp {
    Gt, // >
    Lt, // <
    Gte, // >=
    Lte, // <=
}