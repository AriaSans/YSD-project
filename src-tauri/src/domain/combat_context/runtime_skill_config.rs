use std::sync::Arc;

use crate::domain::gamedb::skills::{AttackWindow, SkillTag, SkillType};
use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::SkillID;
use crate::domain::types::tick::Tick;
use crate::domain::types::trigger::TriggerCondition;

use super::action::EffectType;

// 计算好的SkillConfig
#[derive(Debug, Clone)]
pub struct RuntimeSkillConfig {
    // === 基础信息 ===
    pub sk_id: SkillID,
    pub sk_type: SkillType,

    // === 技能效果 ===
    pub effects: Vec<RuntimeSkillEvent>,

    // === 时间tick ===
    pub duration_total_tick: Tick,
    pub backswing_start_tick: Tick, // 后摇时间点(action判定点)，在此点后为绝对垃圾时间
    pub stamp_otherpoint_tick: Option<Vec<Tick>>, // 多段检测点，如果攻击为多段可能使用

    // === 可选信息 ===
    // 基础攻击 —— 重击可滑动区间，自然重击点在stamp_backswing_ms，小于此需要闪避打断前几段后摇
    pub next_attack_config: Option<Arc<RuntimeSkillConfig>>,
    pub next_attack_window_ms: Option<AttackWindow<Tick>>, // 下一段攻击窗口(min, max)
    // 战技 —— 耗费sp100
    pub sp_cost: Option<Fixed>,
    // 连携技 —— 需要cd，需要前置条件触发
    pub trigger_conditions: Option<TriggerCondition>,
    pub combo_cd_tick: Option<Tick>,
    // 终极技 —— 需要能量（能量值满才能施放，可以由干员配置中的能量值上限确定，此处可省略），战技消耗技力时全队回复6.5/100体力，连携技默认10点（特殊除外，页面会标出，如42，别礼）
    pub energy_cost: Option<Fixed>,
    // 极限闪避 —— 回复sp7.5，暂停sp回复越0.5s
    pub stamina_cost: Option<Fixed>,
    // 处决 —— 需要主控，回复sp，失衡时间/上限/sp回复值由boss确定
    // 其他：连携技、终结技冻结时间每个角色相同，由全局变量AppSetting确定

    // === 规则标签 ===
    pub tags: Vec<SkillTag>,

}

#[derive(Debug, Clone)]
pub struct RuntimeSkillEvent {
    pub offset_tick: Tick,
    pub effect: EffectType<Tick, Fixed>,
}