use rocket;
use crate::connection;
use crate::api;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
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
        ).launch();
}