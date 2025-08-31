use serde::{Deserialize};

// DTO for assigning a permission to a role
#[derive(Debug, Deserialize)]
pub struct AssignRolePermissionSchema {
    pub role_id: i32,
    pub permission_id: i32,
}