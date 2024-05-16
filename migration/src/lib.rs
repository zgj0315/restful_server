pub use sea_orm_migration::prelude::*;

mod m20240516_102649_create_kl_role;
mod m20240516_102653_create_kl_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240516_102649_create_kl_role::Migration),
            Box::new(m20240516_102653_create_kl_user::Migration),
        ]
    }
}
