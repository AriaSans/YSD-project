use std::sync::{Arc, Mutex};

use crate::domain::combat_context::CombatContext;
use crate::domain::gamedb::GameDB;
use crate::domain::project_state::ProjectState;
use crate::domain::types::data::TargetSelector;
use crate::domain::types::id::SlotIndex;

use super::project_service::ProjectService;

pub struct CombatService {
    // 引用全局DB
    db: Arc<GameDB>,

    // 项目
    pub project: Arc<Mutex<ProjectState>>,

    // 运行时
    pub context: Arc<Mutex<Option<CombatContext>>>,
}

impl CombatService {
    pub fn new(
        game_db: Arc<GameDB>,
        project: Arc<Mutex<ProjectState>>,
        context: Arc<Mutex<Option<CombatContext>>>,
    ) -> Self {
        Self {
            db: game_db,
            project: project,
            context: context,
        }
    }

    // fn resolve_targets(&self, selector: TargetSelector, caster_slot: SlotIndex) -> Vec<SlotIndex> {
    //     let mut targets = vec![];
    //     // xxxxxxx 是否存在了锁之内？
    //     let snapshot = self.context.lock().unwrap().as_ref().unwrap();
    //     // if let Some(snapshot) = snapshot_guard.as_ref(){} else{}
    // }
}
