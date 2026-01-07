use std::sync::Mutex;

use crate::domain::combat_context::CombatContext;
use crate::domain::gamedb::GameDB;
use crate::domain::project_state::ProjectState;
use crate::domain::setting::AppSetting;

pub struct AppState{
    // 动态数据
    pub project: Mutex<ProjectState>,
    pub runtime: Mutex<CombatContext>,

    // 静态数据
    pub game_db: GameDB,
    pub setting: AppSetting,
}