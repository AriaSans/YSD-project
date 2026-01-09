use im::{HashMap, Vector};

use crate::domain::types::data::*;
use crate::domain::types::id::BuffID;
use crate::domain::types::id::MechanismStateID;
use crate::domain::types::id::SkillID;
use crate::domain::types::id::SlotIndex;
use crate::domain::types::statusbuff::enemy::*;
use crate::domain::types::statusbuff::operator::*;
use crate::domain::types::tick::*;

pub struct EnvironmentSnapshot {
    // 主控
    pub controlled_slotindex: SlotIndex,

    // 资源类
    pub sp: SpState,
    pub energys: [Energy; 4],
    pub special_resources: Vector<SpecialResource>,

    // 连携技
    pub combo_cd: Vector<Tick>,    // 连携技cd，减少至0触发
    pub qte_windows: HashMap<SkillID, ExpireTick>,

    // 敌人状态/debuff
    pub stagger_enemy: Stagger,
    pub status_enemy: HashMap<StatusEnemy, ExpireTick>, // 无层数，直接刷新
    pub active_debuffs_enemy: HashMap<BuffID, DeBuffEnemy>,
    pub physical_stack: InflictionStacks,
    pub arts_stack: ArtsStacks,

    // 我方状态/Buff
    pub active_buffs_operator: HashMap<(SlotIndex, BuffID), ActiveBuff>,     // 我方buff

    // 通用的隐形状态(计时，累计，触发)
    pub mechanism_states: HashMap<MechanismStateID, MechanismState>
}
