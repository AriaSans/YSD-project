use im::{HashMap, Vector};

use crate::domain::types::data::*;
use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::BuffID;
use crate::domain::types::id::MechanismStateID;
use crate::domain::types::id::SkillID;
use crate::domain::types::id::SlotIndex;
use crate::domain::types::statusbuff::enemy::*;
use crate::domain::types::statusbuff::operator::*;
use crate::domain::types::tick::*;

use super::action::Action;

pub struct EnvironmentSnapshot {
    // === 1. 时间维 ===
    pub current_tick: Tick,

    // === 2. 动作维 ===
    pub active_actions: Vector<Action>,

    // === 3. 实体维 ===
    // 主控
    pub controlled_slotindex: SlotIndex,

    // 资源类
    pub sp: SpState,
    pub energys: Vector<Energy>,
    pub special_resources: Vector<SpecialResource>,

    // 连携技
    pub combo_cd: Vector<Tick>, // 连携技cd，减少至0触发
    pub qte_windows: HashMap<SkillID, ExpireTick>,

    // 敌人状态/debuff
    pub stagger_enemy: Stagger,
    pub status_enemy: HashMap<StatusEnemy, ExpireTick>, // 无层数，直接刷新
    pub active_debuffs_enemy: HashMap<BuffID, DeBuffEnemy>,
    pub physical_stack: InflictionStacks,
    pub arts_stack: ArtsStacks,

    // 我方状态/Buff
    pub active_buffs_operator: HashMap<(SlotIndex, BuffID), ActiveBuff>, // 我方buff

    // 通用的隐形状态(计时，累计，触发)
    pub mechanism_states: HashMap<MechanismStateID, MechanismState>,
}

impl EnvironmentSnapshot {
    pub fn init() -> Self {
        let active_actions = Vector::new();
        let energys = Vector::new();
        let special_resources = Vector::new();
        let combo_cd = Vector::new();
        let qte_windows = HashMap::new();
        let status_enemy = HashMap::new();
        let active_debuffs_enemy = HashMap::new();
        let active_buffs_operator = HashMap::new();
        let mechanism_states = HashMap::new();

        Self {
            current_tick: Tick(0),
            active_actions,
            controlled_slotindex: SlotIndex(0),
            sp: SpState { value: Fixed::from_int(200) },
            energys,
            special_resources,
            combo_cd,
            qte_windows,
            stagger_enemy: Stagger { value: Fixed::from_int(0) },
            status_enemy,
            active_debuffs_enemy,
            physical_stack: InflictionStacks(0),
            arts_stack: ArtsStacks::None,
            active_buffs_operator,
            mechanism_states,
        }
    }
}
