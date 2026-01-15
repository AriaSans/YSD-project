use serde::Serialize;

use crate::domain::types::id::{BuffID, SkillID};

#[derive(Serialize)]
pub struct CombatPreviewDTO {
    // 1. 全局统计
    pub total_damage: f64,

    // 2. 列示存储数据(每Tick)
    pub timeline_tick: Vec<i64>,
    pub team_sp_curve: Vec<i64>,

    // 3. 区块数据（UI）
    pub skill_blocks: Vec<UITimelineBlock>,
    pub buff_blocks: Vec<UIBuffBlock>,

    // 4. 警告列表
    pub warnings: Vec<TimelineWarning>
}

#[derive(Serialize)]
pub struct UITimelineBlock {
    pub skill_id: SkillID,
    pub stack_tick: i64,
    pub duration: i64,
}

#[derive(Serialize)]
pub struct UIBuffBlock {
    pub buff_id: BuffID,
    pub stack_tick: i64,
    pub duration: i64,
}

#[derive(Serialize)]
pub struct TimelineWarning {
    pub skill_instance_uid: u64,
    pub error_list: Vec<u64>,
}