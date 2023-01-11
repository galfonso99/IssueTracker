#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;


use crate::schema::users::dsl::{users, first_name, last_name};
use crate::schema::comments::dsl::{comments, parent_id, body, issue_id};
use rocket_contrib::json::Json;
use diesel::result::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayComment {
    body: String,
    first_name: String,
    last_name: String,
    id: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NestedComment {
    pub parent: DisplayComment,
    pub children: Vec<DisplayComment>,
}

pub fn fetch_comments(issue_ID: i32, conn: &PgConnection) -> Result<Json<Vec<NestedComment>>, Error> {
    use crate::schema::comments::dsl::id;
    let mut nested_comments: Vec<NestedComment> = vec![];
    let main_comments = comments.filter(issue_id.eq(issue_ID).and(parent_id.is_null()))
        .limit(10)
        .inner_join(users)
        .select((body, first_name, last_name, id))
        .load::<(String, String, String, i32)>(conn)?
        .into_iter().map(|comment| DisplayComment {body: comment.0, first_name: comment.1, last_name: comment.2, id: comment.3})
        .collect::<Vec<DisplayComment>>();
    for comment in main_comments {
        let child_comments = fetch_child_comments(comment.id, conn)?;
        nested_comments.push(
            NestedComment {
                parent: comment,
                children: child_comments
            }
        )
    };
    Ok(Json(nested_comments))
}

pub fn fetch_child_comments(par_id: i32, conn: &PgConnection) -> Result<Vec<DisplayComment>, Error> {
    use crate::schema::comments::dsl::{id, posted_at};
    let child_comments = comments.filter(parent_id.eq(par_id))
        .limit(5)
        .inner_join(users)
        .select((body, first_name, last_name, id))
        .order(posted_at.asc())
        .load::<(String, String, String, i32)>(conn)?
        .into_iter().map(|comment| DisplayComment {body: comment.0, first_name: comment.1, last_name: comment.2, id: comment.3})
        .collect::<Vec<DisplayComment>>();
    Ok(child_comments)
}