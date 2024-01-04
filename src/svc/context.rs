use crate::domain::user::entity::PgUserRepository;
use crate::pkg::database;
use crate::svc::config::Config;
#[derive(Clone)]
pub struct ServiceContext {
    pub config: Config,
    pub pool: database::Pool,

    pub user_repository: PgUserRepository,
}

pub fn init_service(c: Config) -> ServiceContext {
    let pool = database::init_pool(&c.database.url);
    ServiceContext {
        config: c,
        pool: pool,
        user_repository: PgUserRepository {},
    }
}
