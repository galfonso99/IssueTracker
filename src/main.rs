#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use rocket::Config;

mod api;
mod connection;
mod schema;



// #[rocket::main]
// async fn main() {
//     dotenv().ok();
//     create_routes().await;
// }


// pub async fn create_routes() {
//     rocket::build()
//         .mount("/api",
//                routes![
//                     api::handler::all_issues,
//                     api::handler::get_issue,
//                     api::handler::post_issue,
//                     api::handler::delete_issue,
//                     api::handler::post_user,
//                     api::handler::get_user,
//                     api::handler::post_comment,
//                     api::handler::get_comment,
//                     api::handler::delete_comment,
//                     api::handler::get_issue_comments,
//                     api::handler::get_user_comments,
//                     api::handler::get_child_comments,
//                     api::handler::assign_issue,
//                     api::handler::toggle_issue_state,
//                     ],
//         )
//         .launch().await
//         .expect("Error");
// }

#[launch]
pub async fn create_routes() -> _ {
    rocket::build()
        .mount("/api",
               routes![
                    api::handler::all_issues,
                    api::handler::get_issue,
                    api::handler::post_issue,
                    api::handler::delete_issue,
                    api::handler::post_user,
                    api::handler::get_user,
                    api::handler::post_comment,
                    api::handler::get_comment,
                    api::handler::delete_comment,
                    api::handler::get_issue_comments,
                    api::handler::get_user_comments,
                    api::handler::get_child_comments,
                    api::handler::assign_issue,
                    api::handler::toggle_issue_state,
                ],
        )

}