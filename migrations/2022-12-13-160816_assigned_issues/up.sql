-- Your SQL goes here
CREATE TABLE assigned_issues (
    id SERIAL PRIMARY KEY,
    issue_id int NOT NULL,
    FOREIGN KEY (issue_id) REFERENCES issues(id),
    user_id int NOT NULL,
    FOREIGN KEY (user_id) REFERENCes users(id)
);

-- INSERT INTO assigned_issues (issue_id, user_id)
-- VALUES (2, 2), (4,2);