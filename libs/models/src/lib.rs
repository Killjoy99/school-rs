pub mod user;
pub mod institution;
pub mod academic_unit;
pub mod permission;
pub mod role;
pub mod role_permission;
pub mod user_role;
pub mod user_institution;

// Expose the Struct at root lavel
pub use user::User;
pub use permission::Permission;
pub use role::Role;
pub use institution::{Institution, InstitutionType};
pub use academic_unit::{AcademicUnit, AcademicUnitType};
pub use role_permission::RolePermission;
pub use user_role::UserRole;