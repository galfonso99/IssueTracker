use std::env;
use std::ops::{Deref, DerefMut};

use diesel::prelude::PgConnection;
use diesel::prelude::Connection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
// use r2d2_diesel::ConnectionManager;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
// use rocket::{outcome::Outcome, Request, State};
use rocket::outcome::Outcome;
use rocket::State;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use dotenv::dotenv;

// type MyPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
 
fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn establish_connection() -> PgPool {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     init_pool(&database_url).expect("Failed to create pool")
// }

// pub fn init_pool() -> MyPool {
//     let manager = ConnectionManager::<PgConnection>::new(database_url());
//     Pool::new(manager).expect("db pool failed")
// }

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub  PgPooledConnection);

// impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
//         let pool = request.guard::<State<Pool>>()?;
//         match pool.get() {
//             Ok(conn) => Outcome::Success(DbConn(conn)),
//             Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//         }
//     }
// }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = req.guard::<DbConn>().await;
        match pool.succeeded() {
            Some(conn) => Outcome::Success(conn),
            _ => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
