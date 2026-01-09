use serde::{Deserialize, Serialize};
use ustr::Ustr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct OperatorID(pub Ustr);  // "oper_surtr"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SkillID(pub Ustr); // "sk_surtr_basic/battle/combo/ultimate"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct GearID(pub Ustr); // "gear_type_name"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct GearSetID(pub Ustr); // "set_name"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct WeaponID(pub Ustr); // "wpn_type_name"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BuffID(pub Ustr);  // "buff_amp"."debuff_name"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct TalentID(pub Ustr);  // "tal_surtr"

#[derive(Clone, Copy,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct MechanismStateID(pub Ustr);  // "mecs_type_statename"


#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct InstanceID(pub u64); // 自动增长


#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct SlotIndex(pub u8);

impl SlotIndex {
    // 判断是否对应全局，0、1、2、3代表1234位，4代表全局
    pub fn is_all(&self) -> bool {
        if self.0 == 4 {
            true
        }
        else {
            false
        }
    }
}