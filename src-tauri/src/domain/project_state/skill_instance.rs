use serde::{Deserialize, Serialize};

use crate::domain::types::id::{InstanceID, SkillID, SlotIndex};
use crate::domain::types::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstance {
    pub uid: InstanceID,    // 实例id
    pub slot_index: SlotIndex,// 所属干员槽位

    pub sk_id: SkillID,// 关联技能id
    pub start_tick: Tick,// 开始时间戳
    pub backswing_start_tick: Tick,// 后摇开始时间戳（部分技能可变更）
    pub other_stamp_tick: Option<Vec<Tick>>,// 其他节点时间戳（部分技能可变更）


}
