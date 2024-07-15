#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};

pub type Conn = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Conn>>;

pub fn init_pool(url: &str) -> Pool {
    use log::info;
    info!("configuring database ...");
    let manager = ConnectionManager::<Conn>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
