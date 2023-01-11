use crate::diesel::{Insertable, Queryable};
use crate::schema::UserRole;
use crate::schema::{assigned_issues, comments, issues, users};
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]

/** STRUCT LIST
 * Issue
 * NewIssue
 * IssueJson
 * User 
 * NewUser
 * UserJson
 * Comment
 * NewComment
 * CommentJson
 * AssignedIssue
 * NewAssignedIssue
 */

pub struct Issue {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub tags: Option<String>,
    pub is_open: bool,
    pub posted_at: NaiveDateTime,
}

#[derive(Insertable, Queryable,  Serialize, Deserialize)]
#[table_name = "issues"]
pub struct NewIssue<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub tags: Option<&'a str>,
    pub is_open: bool,
    pub posted_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueJson<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub tags: Option<&'a str>,
    pub is_open: bool,
}

// #[derive(AsChangeSet)
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub user_role: UserRole,
    pub added: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_role: UserRole,
    pub added: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct UserJson<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_role: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub issue_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub body: String,
    pub posted_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub issue_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub body: &'a str,
    pub posted_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentJson<'a> {
    pub issue_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub body: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct AssignedIssue {
    pub id: i32,
    pub issue_id: i32,
    pub user_id: i32,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "assigned_issues"]
pub struct NewAssignedIssue {
    pub issue_id: i32,
    pub user_id: i32,
}

impl<'a> From<UserJson<'a>> for NewUser<'a> {
    fn from(json: UserJson<'a>) -> Self {
        NewUser {
            first_name: json.first_name,
            last_name: json.last_name,
            user_role: match json.user_role {
                "User" => UserRole::User,
                "Admin" => UserRole::Admin,
                "Developer" => UserRole::Developer,
                _ => UserRole::User,
            },
            added: Utc::now().naive_utc(),
        }
    }
}

impl<'a> From<IssueJson<'a>> for NewIssue<'a> {
    fn from(json: IssueJson<'a>) -> Self {
        NewIssue {
            user_id: json.user_id,
            title: json.title,
            body: json.body,
            tags: json.tags,
            is_open: json.is_open,
            posted_at: Utc::now().naive_utc(),
        }
    }
}

impl<'a> From<CommentJson<'a>> for NewComment<'a> {
    fn from(json: CommentJson<'a>) -> Self {
        NewComment {
            issue_id: json.issue_id,
            user_id: json.user_id,
            parent_id: json.parent_id,
            body: json.body,
            posted_at: Utc::now().naive_utc(),
        }
    }
}