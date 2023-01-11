// @generated automatically by Diesel CLI.
use diesel_derive_enum::DbEnum;
#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Developer,
    User,
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
    use super::UserRoleMapping;

    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        user_role -> UserRoleMapping,
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
