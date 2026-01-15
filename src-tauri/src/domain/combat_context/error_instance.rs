use serde::Serialize;

use crate::domain::types::id::InstanceID;

#[derive(Serialize)]
pub struct ErrorInstance {
    pub skill_uid: InstanceID,
    pub error_type: ErrorType,
}


// 错误类型
#[derive(Serialize)]
enum ErrorType {
    Spdef,
}