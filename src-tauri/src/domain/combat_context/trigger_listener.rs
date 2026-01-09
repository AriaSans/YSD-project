use crate::domain::types::id::{MechanismStateID, SkillID, SlotIndex};
use crate::domain::types::trigger::TriggerCondition;

pub struct TriggerListener {
    pub from_slot: SlotIndex,
    pub skill_id: SkillID,
    pub condition: TriggerCondition,
    pub icd_id: Option<MechanismStateID>, // 内置冷却计时器的id(用于去快表查CD)
}