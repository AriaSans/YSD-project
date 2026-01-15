use std::sync::{Arc, Mutex};

use crate::domain::gamedb::GameDB;
use crate::domain::project_state::ProjectState;

pub struct ProjectService {
    pub db: Arc<GameDB>,
    pub state: Arc<Mutex<ProjectState>>,
}

impl ProjectService {
    pub fn new(game_db:Arc<GameDB>,project: Arc<Mutex<ProjectState>>) -> Self {
        Self { 
            db: game_db.clone(),
            state: project.clone() 
        }
    }
}