use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status::{self, NoContent};
use rocket::serde::json::Json;
extern crate dotenv;

use crate::api::repository::{fetch_comments, NestedComment};
use crate::schema::{assigned_issues, comments, issues, users, UserRole};

use crate::api::models::{Issue, NewIssue, NewUser, User, UserJson};
use crate::connection::{DbConn, establish_connection};

use super::models::{AssignedIssue, Comment, CommentJson, IssueJson, NewAssignedIssue, NewComment};

// MEMO: To receive custom errors remove Status error handling and let errors propagate themselves

type UserComment = (String, String, String);

#[derive(Serialize, Deserialize)]
pub struct ToggleIssue {
    is_open: bool,
}

#[get("/")]
pub fn all_issues() -> Result<Json<Vec<Issue>>, Status> {
    let conn = &mut establish_connection();
    issues::table
        .limit(30)
        .load::<Issue>(conn)
        .map(Json)
        .map_err(error_status)
}


#[get("/issue/<issue_id>")]
pub fn get_issue(issue_id: i32) -> Result<Json<Issue>, Status> {
    let conn = &mut establish_connection();
    issues::table
        .find(issue_id)
        .get_result::<Issue>(conn)
        .map(Json)
        .map_err(error_status)
}

#[post("/issue", format = "application/json", data = "<issue>")]
pub fn post_issue(
    issue: Json<IssueJson>,
    mut conn: DbConn,
) -> Result<status::Created<Json<Issue>>, Status> {
    let new_issue: NewIssue = issue.into_inner().into();
    diesel::insert_into(issues::table)
        .values(new_issue)
        .get_result(&mut conn as &mut PgConnection)
        .map(|issue: Issue| {
            status::Created::new(format!( "{host}:{port}/issue/{id}", host=host(), port=port(), id=issue.id ))
            .body(
                Json(issue)
            )
        })
        .map_err(error_status)
}

#[delete("/issue/<issue_id>/user/<user_id>")]
pub fn delete_issue(issue_id: i32, user_id: i32, mut conn: DbConn) -> Result<NoContent, Status> {
    use crate::schema::issues::dsl::user_id as author_id;
    use crate::schema::users::dsl::user_role;
    let curr_user_role = users::table
        .find(user_id)
        .select(user_role)
        .get_result::<UserRole>(&mut conn as &mut PgConnection)
        .unwrap();
    let author_user_id = issues::table
        .find(issue_id)
        .select(author_id)
        .get_result::<i32>(&mut conn as &mut PgConnection)
        .unwrap();
    if user_id != author_user_id && curr_user_role != UserRole::Admin {
        return Err(Status::Unauthorized);
    }
    diesel::delete(issues::table.find(issue_id))
        .execute(&mut conn as &mut PgConnection)
        .map(|_| status::NoContent)
        .map_err(error_status)
}

#[put(
    "/issue/<id>/toggle/user/<user_id>",
    format = "application/json",
    data = "<issue>"
)]
pub fn toggle_issue_state(
    id: i32,
    issue: Json<ToggleIssue>,
    user_id: i32,
    mut conn: DbConn,
) -> Result<Json<Issue>, Status> {
    use crate::schema::issues::dsl::{is_open, user_id as author_id};
    use crate::schema::users::dsl::user_role;
    let curr_user_role = users::table
        .find(user_id)
        .select(user_role)
        .get_result::<UserRole>(&mut conn as &mut PgConnection)
        .unwrap();
    let author_user_id = issues::table
        .find(id)
        .select(author_id)
        .get_result::<i32>(&mut conn as &mut PgConnection)
        .unwrap();
    if user_id != author_user_id
        && curr_user_role != UserRole::Admin
        && curr_user_role != UserRole::Developer
    {
        return Err(Status::Unauthorized);
    }
    diesel::update(issues::table.find(id))
        .set(is_open.eq(!issue.into_inner().is_open))
        .get_result(&mut conn as &mut PgConnection)
        .map(Json)
        .map_err(error_status)
}

#[get("/user/<user_id>")]
pub fn get_user(user_id: i32, mut conn: DbConn) -> Result<Json<User>, Status> {
    users::table
        .find(user_id)
        .get_result::<User>(&mut conn as &mut PgConnection)
        .map(Json)
        .map_err(error_status)
}

#[post("/user", format = "application/json", data = "<user>")]
pub fn post_user(
    user: Json<UserJson>,
    mut conn: DbConn,
) -> Result<status::Created<Json<User>>, Status> {
    let new_user: NewUser = user.into_inner().into();
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&mut conn as &mut PgConnection)
        .map(|user: User| {
            status::Created::new(format!( "{host}:{port}/issue/{id}", host=host(), port=port(), id=user.id ))
            .body(
                Json(user)
            )
        })
        .map_err(error_status)
}

#[get("/comment/<comment_id>")]
pub fn get_comment(comment_id: i32, mut conn: DbConn) -> Result<Json<Comment>, Status> {
    comments::table
        .find(comment_id)
        .get_result::<Comment>(&mut conn as &mut PgConnection)
        .map(Json)
        .map_err(error_status)
}

#[post("/comment", format = "application/json", data = "<comment>")]
pub fn post_comment(
    comment: Json<CommentJson>,
    mut conn: DbConn,
) -> Result<status::Created<Json<Comment>>, Status> {
    let new_comment: NewComment = comment.into_inner().into();
    diesel::insert_into(comments::table)
        .values(new_comment)
        .get_result(&mut conn as &mut PgConnection)
        .map(|comment: Comment| {
            status::Created::new(format!( "{host}:{port}/issue/{id}", host=host(), port=port(), id=comment.id ))
            .body(
                Json(comment)
            )
        })
        .map_err(error_status)
}

#[delete("/comment/<id>")]
pub fn delete_comment(id: i32, mut conn: DbConn) -> Result<NoContent, Status> {
    diesel::delete(comments::table.find(id))
        .execute(&mut conn as &mut PgConnection)
        .map(|_| status::NoContent)
        .map_err(error_status)
}

#[get("/issue/<id>/comments")]
pub fn get_issue_comments(id: i32, mut conn: DbConn) -> Result<Json<Vec<NestedComment>>, Status> {
    fetch_comments(id, &mut conn).map_err(error_status)
}

#[get("/user/<id>/comments")]
pub fn get_user_comments(id: i32, mut conn: DbConn) -> Result<Json<Vec<UserComment>>, Status> {
    use crate::schema::comments::{body, user_id};
    use crate::schema::users::dsl::{first_name, last_name, users};
    comments::table
        .filter(user_id.eq(id))
        .inner_join(users)
        .select((first_name, last_name, body))
        .load::<UserComment>(&mut conn as &mut PgConnection)
        .map(Json)
        .map_err(error_status)
}

#[post("/assign", format = "application/json", data = "<assign>")]
pub fn assign_issue(
    assign: Json<NewAssignedIssue>,
    mut conn: DbConn,
) -> Result<status::Created<Json<AssignedIssue>>, Status> {
    use crate::schema::users::user_role;
    let assigned_issue = assign.into_inner();
    let curr_user_role = users::table
        .find(assigned_issue.user_id)
        .select(user_role)
        .get_result::<UserRole>(&mut conn as &mut PgConnection)
        .unwrap();
    // Set requirement that a user assigned with an issue must be a Developer
    if curr_user_role != UserRole::Developer {
        return Err(Status::BadRequest);
    }
    diesel::insert_into(assigned_issues::table)
        .values(assigned_issue)
        .get_result(&mut conn as &mut PgConnection)
        .map(|assign: AssignedIssue| {
            status::Created::new(format!( "{host}:{port}/issue/{id}", host=host(), port=port(), id=assign.id ))
            .body(
                Json(assign)
            )
        })
        .map_err(error_status)
}

#[get("/comment/<id>/children")]
pub fn get_child_comments(id: i32, mut conn: DbConn) -> Result<Json<Vec<UserComment>>, Status> {
    use crate::schema::comments::{body, parent_id};
    use crate::schema::users::dsl::{first_name, last_name, users};
    comments::table
        .filter(parent_id.eq(id))
        .inner_join(users)
        .select((first_name, last_name, body))
        .load::<UserComment>(&mut conn as &mut PgConnection)
        .map(Json)
        .map_err(error_status)
}

fn host() -> String {
    dotenv::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    dotenv::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        Error::DatabaseError(_, _) => Status::BadRequest,
        _ => Status::InternalServerError,
    }
}
