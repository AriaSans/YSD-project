use serde::{Deserialize, Serialize};

use crate::domain::combat_context::action::EffectType;
use crate::domain::setting::AppSetting;
use crate::domain::types::data::DynamicValue;
use crate::domain::types::fixed::Fixed;
use crate::domain::types::id::SkillID;
use crate::domain::types::tick::{Tick, TimeMs};
use crate::domain::types::trigger::TriggerCondition;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillConfig {
    // === 基础信息 ===
    pub sk_id: SkillID,
    pub sk_type: SkillType,
    pub icon_path_sk: String, // 图标地址

    // === 技能效果 ===
    pub effects: Vec<SkillEvent>,

    // === 时间tick ===
    pub duration_total_tick: Tick,
    pub backswing_start_tick: Tick, // 后摇时间点(action判定点)，在此点后为绝对垃圾时间
    pub stamp_otherpoint_tick: Option<Vec<Tick>>, // 多段检测点，如果攻击为多段可能使用

    // === 可选信息 ===
    // 基础攻击 —— 重击可滑动区间，自然重击点在stamp_backswing_ms，小于此需要闪避打断前几段后摇
    pub next_attack_id: Option<SkillID>,
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

    // === 各等级参数 ===
    pub params: Vec<Vec<Fixed>>,

    // === 省略：name，description ===
}

// 用来存放json文件，ms需要映射为tick
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillConfigData {
    // === 基础信息 ===
    pub sk_id: SkillID,
    pub sk_type: SkillType,
    pub icon_path_sk: String, // 图标地址
    // === 技能效果 ===
    pub effects: Vec<SkillEventMs>,

    // === 时间ms ===
    pub duration_total_ms: TimeMs,
    pub backswing_start_ms: TimeMs, // 后摇时间点(action判定点)，在此点后为绝对垃圾时间
    pub stamp_otherpoint_ms: Option<Vec<TimeMs>>, // 多段检测点，如果攻击为多段可能使用

    // === 可选信息 ===
    // 基础攻击 —— 重击可滑动区间，自然重击点在stamp_backswing_ms，小于此需要闪避打断前几段后摇
    pub next_attack_id: Option<SkillID>,
    pub next_attack_window_ms: Option<AttackWindow<TimeMs>>, // 下一段攻击窗口(min, default, max)
    // 战技 —— 耗费sp100
    pub sp_cost: Option<f64>,
    // 连携技 —— 需要cd，需要前置条件触发
    pub trigger_conditions: Option<TriggerCondition>,
    pub combo_cd_ms: Option<TimeMs>,
    // 终极技 —— 需要能量（能量值满才能施放，可以由干员配置中的能量值上限确定，此处可省略），战技消耗技力时全队回复6.5/100体力，连携技默认10点（特殊除外，页面会标出，如42，别礼）
    pub energy_cost: Option<f64>,
    // 极限闪避 —— 回复sp7.5，暂停sp回复越0.5s
    pub stamina_cost: Option<f64>,
    // 处决 —— 需要主控，回复sp，失衡时间/上限/sp回复值由boss确定
    // 其他：连携技、终结技冻结时间每个角色相同，由全局变量AppSetting确定

    // === 规则标签 ===
    #[serde(default)]
    pub tags: Vec<SkillTag>,

    // === 各等级参数 ===
    pub params: Vec<Vec<f64>>,
    // ==== 省略：name ===
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkillType {
    BasicAttack,
    BattleSkill,
    ComboSkill,
    Ultimate,
    GearSetSkill,
    WeaponSkill,
    ChangeControlled,
    Dodge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillEventMs {
    pub offset_ms: TimeMs,
    pub effect: EffectType<TimeMs, DynamicValue<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillEvent {
    pub offset_tick: Tick,
    pub effect: EffectType<Tick, DynamicValue<Fixed>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AttackWindow<T> {
    pub start: T,
    pub end: T,
    pub default: T, // 默认点位
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SkillTag {
    // === 打断相关 ===
    HardCommitment,       // 强硬直（完全不可打断，如大招演出）
    CancelableOnHit,      // 命中即自由（如重击，造成伤害后允许闪避）
    KeepComboOnInterrupt, // 被打断时不重置连击段数

    // === 技能属性 ===
    Melee,   // 近战
    Ranged,  // 远程
    Healing, // 治疗
}

impl SkillEventMs {
    pub fn into_tick(self, setting: &AppSetting) -> SkillEvent {
        SkillEvent {
            offset_tick: self.offset_ms.to_tick(setting),
            effect: self.effect.convert(setting),
        }
    }
}

impl SkillConfigData {
    pub fn new(
        sk_id: SkillID,             // 技能id
        sk_type: SkillType,         // 技能类型
        effects: Vec<SkillEventMs>, // 效果
        icon_path: String,          // 图标
        duration_total_ms: TimeMs,  // 持续时间
        backswing_start_ms: TimeMs, // 后摇时间点(action判定点)
    ) -> Self {
        Self {
            sk_id,
            sk_type,
            icon_path_sk: icon_path,
            effects,
            duration_total_ms,
            backswing_start_ms,
            stamp_otherpoint_ms: None,
            next_attack_id: None,
            next_attack_window_ms: None,
            sp_cost: None,
            trigger_conditions: None,
            combo_cd_ms: None,
            energy_cost: None,
            stamina_cost: None,
            tags: vec![],
            params: vec![vec![]],
        }
    }

    pub fn into_tick(self, setting: &AppSetting) -> SkillConfig {
        // 1. 转换effects
        let effects_tick: Vec<SkillEvent> = self
                .effects
                .into_iter()
                .map(|ms| ms.into_tick(setting))
                .collect();

        SkillConfig {
            sk_id: self.sk_id,
            sk_type: self.sk_type,
            icon_path_sk: self.icon_path_sk,
            effects: effects_tick,
            duration_total_tick: self.duration_total_ms.to_tick(setting),
            backswing_start_tick: self.backswing_start_ms.to_tick(setting),
            stamp_otherpoint_tick: self
                .stamp_otherpoint_ms
                .map(|ms| ms.into_iter().map(|i| i.to_tick(setting)).collect()),
            next_attack_id: self.next_attack_id,
            next_attack_window_ms: self.next_attack_window_ms.map(|ms| AttackWindow {
                start: ms.start.to_tick(setting),
                end: ms.end.to_tick(setting),
                default: ms.default.to_tick(setting),
            }),
            sp_cost: self.sp_cost.map(|f| Fixed::from_float(f)),
            trigger_conditions: self.trigger_conditions.map(|tc| tc),
            combo_cd_tick: self.combo_cd_ms.map(|ms| ms.to_tick(setting)),
            stamina_cost: self.stamina_cost.map(|s| Fixed::from_float(s)),
            energy_cost: self.energy_cost.map(|f| Fixed::from_float(f)),
            tags: self.tags,
            params: self
                .params
                .iter()
                .map(|v| v.iter().map(|f| Fixed::from_float(*f)).collect())
                .collect(),
        }
    }

    pub fn set_basic(
        &mut self,
        stamp_otherpoint_ms: Option<Vec<TimeMs>>, // 多段检测点，如果攻击为多段可能使用
        next_attack_window_ms: Option<AttackWindow<TimeMs>>, // 下次攻击窗口
    ) {
        self.stamp_otherpoint_ms = stamp_otherpoint_ms;
        self.next_attack_window_ms = next_attack_window_ms;
    }

    pub fn set_basic_final(
        &mut self,
        stamp_otherpoint_ms: Option<Vec<TimeMs>>, // 多段检测点，如果攻击为多段可能使用
    ) {
        self.stamp_otherpoint_ms = stamp_otherpoint_ms;
    }

    pub fn set_battle(
        &mut self,
        stamp_otherpoint_ms: Option<Vec<TimeMs>>, // 多段检测点，如果攻击为多段可能使用
    ) {
        self.stamp_otherpoint_ms = stamp_otherpoint_ms;
    }

    pub fn set_dodge(
        &mut self,
        stamina_cost: Option<f64>, // 耗费体力
    ) {
        self.stamina_cost = stamina_cost;
    }
}
