use serde::{Deserialize, Serialize};

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct OperatorID(pub String);  // "oper_surtr"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SkillID(pub String); // "sk_surtr_basic/battle/combo/ultimate"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct GearID(pub String); // "gear_type_name"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct GearSetID(pub String); // "set_name"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct WeaponID(pub String); // "wpn_type_name"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BuffID(pub String);  // "buff_amp"."debuff_name"

#[derive(Clone,PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct TalentID(pub String);  // "tal_surtr"


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