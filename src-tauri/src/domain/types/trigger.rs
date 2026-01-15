use serde::{Deserialize, Serialize};

use super::data::InflictionType;
use super::fixed::Fixed;
use super::id::BuffID;
use super::statusbuff::enemy::StatusEnemy;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TriggerTag {
    FinalStrike, // 重击
    Crush,       // 猛击
    Fracture,    // 碎甲

    // 增加敌人状态
    PhysicalInfliction, // 物理异常
    HeatInfliction,     // 灼热异常
    ElectricInfliction, // 电磁异常
    CryoInfliction,     // 寒冷异常
    NatureInfliction,   // 自然异常

    // 增加buff
    Amp,         // 增幅
    Protected,   // 庇护
    Susceptible, // 脆弱
    Weakened,    // 虚弱
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition<V = Fixed> {
    // === 状态（敌人）检测 ===
    HaveInflictionStacks {
        inflication_type: InflictionType,
        stacks_min: u8,
    }, // 持有异常层数(以上)
    HaveNoInfliction(InflictionType), // 未持有异常
    HaveStatusEnemy(StatusEnemy),     // 持有状态（敌人）
    ConsumeStatusEnemy(StatusEnemy),  // 消耗状态（敌人）
    GetStatusEnemy(StatusEnemy),      // 进入状态（敌人）

    // === 状态（我方）检测 ===
    CheckHp {
        compare_type: CompareOp,
        value: V,
    }, // 检测血量less/more, 百分比
    GetActiveStatus(BuffID),     // 施加状态（友方）
    ConsumeActiveStatus(BuffID), // 消耗状态（友方）

    // 受击检测
    Injured,

    // === tag标记条件 ===
    HaveBuffWithTag(TriggerTag), // 拥有Buff（敌我）
    HitByTag(TriggerTag),        // 触发点

    // === 概率 ===
    Probability(V),

    // === 逻辑 ===
    And(Vec<TriggerCondition<V>>),
    Or(Vec<TriggerCondition<V>>),
    Not(Box<TriggerCondition<V>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CompareOp {
    Gt,  // >
    Lt,  // <
    Gte, // >=
    Lte, // <=
}

impl TriggerCondition<f64> {
    pub fn convert(self) -> TriggerCondition<Fixed> {
        match self {
            TriggerCondition::ConsumeActiveStatus(buff_id) => {
                TriggerCondition::ConsumeActiveStatus(buff_id)
            }
            TriggerCondition::GetActiveStatus(buff_id) => {
                TriggerCondition::GetActiveStatus(buff_id)
            }
            TriggerCondition::HaveInflictionStacks {
                inflication_type,
                stacks_min,
            } => TriggerCondition::HaveInflictionStacks {
                inflication_type,
                stacks_min,
            },
            TriggerCondition::HaveNoInfliction(inflication_type) => {
                TriggerCondition::HaveNoInfliction(inflication_type)
            }
            TriggerCondition::HaveStatusEnemy(status_enemy) => {
                TriggerCondition::HaveStatusEnemy(status_enemy)
            }
            TriggerCondition::ConsumeStatusEnemy(status_enemy) => {
                TriggerCondition::ConsumeStatusEnemy(status_enemy)
            }
            TriggerCondition::GetStatusEnemy(status_enemy) => {
                TriggerCondition::GetStatusEnemy(status_enemy)
            }
            TriggerCondition::CheckHp {
                compare_type,
                value,
            } => TriggerCondition::CheckHp {
                compare_type,
                value: Fixed::from_float(value),
            },
            TriggerCondition::Injured => TriggerCondition::Injured,
            TriggerCondition::HaveBuffWithTag(tag) => TriggerCondition::HaveBuffWithTag(tag),
            TriggerCondition::HitByTag(tag) => TriggerCondition::HitByTag(tag),
            TriggerCondition::Probability(value) => {
                TriggerCondition::Probability(Fixed::from_float(value))
            }
            TriggerCondition::And(conds) => TriggerCondition::And(
                conds
                    .into_iter()
                    .map(|c| c.convert())
                    .collect(),
            ),
            TriggerCondition::Or(conds) => TriggerCondition::Or(
                conds
                    .into_iter()
                    .map(|c| c.convert())
                    .collect(),
            ),
            TriggerCondition::Not(cond) => TriggerCondition::Not(Box::new((*cond).convert())),
        }
    }
}
