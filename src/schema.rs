// @generated automatically by Diesel CLI.
use diesel_derive_enum::DbEnum;
#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[DieselTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    Admin,
    Developer,
    User,
}

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;

    comments (id) {
        id -> Int4,
        issue_id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        body -> Varchar,
        posted_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    issues (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Varchar,
        tags -> Nullable<Varchar>,
        is_open -> Bool,
        posted_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        user_role -> UserRole,
        added -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    assigned_issues (id) {
        id -> Int4,
        issue_id -> Int4,
        user_id -> Int4,
    }
}
diesel::joinable!(assigned_issues -> issues (issue_id));
diesel::joinable!(assigned_issues -> users (user_id));
diesel::joinable!(comments -> issues (issue_id));
diesel::joinable!(comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(assigned_issues, comments, issues, users,);
