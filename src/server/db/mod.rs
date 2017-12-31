pub mod schema;
use dotenv::dotenv;
use std::env;
use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

lazy_static! {
    pub static ref DB_POOL: r2d2::Pool<ConnectionManager<SqliteConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config = r2d2::Config::builder()
            .pool_size(2)
            .build();

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        pool
    };
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl DB {
    pub fn conn(&self) -> &SqliteConnection {
        &*self.0
    }
}

pub fn get_db() -> DB {
    match DB_POOL.get() {
        Ok(conn) => DB(conn),
        Err(e) => panic!("{:?}", e)
    }
}
