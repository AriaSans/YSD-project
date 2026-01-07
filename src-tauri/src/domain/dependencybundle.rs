use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::project_state::ProjectState;

#[derive(Serialize)]
pub struct ProjectViewContext {
    // 实时数据
    state: ProjectState,

    // 元数据字典（4名角色用到的数据id）
    meta_cache: HashMap<String, MetaDisplayinfo>,
}

// 仅包括展示数据
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDisplayinfo {
    name: String,
    icon_path: String,
    description: String,
}
