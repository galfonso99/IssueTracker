use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2::{self};
use r2d2_diesel::ConnectionManager;
// use rocket::{outcome::Outcome, Request, State};
use rocket::outcome::Outcome;
use rocket::State;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub  r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// #[rocket::async_trait]
// impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
//     type Error = ();

//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         let pool = req.guard::<State<Pool>>().await;
//         match pool.success_or(()) {
//             Ok(conn) => Outcome::Success(DbConn(conn)),
//             Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//         }
//     }
// }

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}