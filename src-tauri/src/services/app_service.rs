use std::sync::{Arc, Mutex};

use crate::domain::combat_context::CombatContext;
use crate::domain::gamedb::GameDB;
use crate::domain::project_state::ProjectState;

use super::combat_service::CombatService;
use super::project_service::ProjectService;

pub struct AppService {
    // 引用全局DB
    pub db: Arc<GameDB>,

    // 项目
    pub project: Arc<Mutex<ProjectState>>,

    // 运行时
    pub context: Arc<Mutex<Option<CombatContext>>>,

    pub project_service: ProjectService,
    pub combat_service: CombatService,
}

impl AppService {
    pub fn new() -> Self {
        let share_db = Arc::new(GameDB::link());
        let share_project = Arc::new(Mutex::new(ProjectState::new()));
        let share_context = Arc::new(Mutex::new(None));

        Self {
            project_service: ProjectService::new(
                share_db.clone(), 
                share_project.clone()
            ),
            combat_service: CombatService::new(
                share_db.clone(),
                share_project.clone(),
                share_context.clone(),
            ),
            db: share_db,
            project: share_project,
            context: share_context,
        }
    }
}
