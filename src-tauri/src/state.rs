use std::sync::{Arc, Mutex};

use crate::domain::combat_context::CombatContext;
use crate::domain::gamedb::GameDB;
use crate::domain::project_state::ProjectState;
use crate::domain::setting::AppSetting;
use crate::services::combat_service::CombatService;

pub struct AppState{
    // 静态数据
    pub setting: Mutex<AppSetting>,
    
    // 环境预览
    pub preview_result: Mutex<Option<CombatContext>>,
}