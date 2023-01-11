-- Your SQL goes here
CREATE TABLE issues (
    id SERIAL PRIMARY KEY,
    user_id int NOT NULL,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    tags VARCHAR,
    is_open BOOLEAN NOT NULL,
    posted_at Timestamp NOT NULL
);

CREATE TYPE user_role AS ENUM ('admin', 'developer', 'user');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    user_role user_role NOT NULL,
    added Timestamp NOT NULL
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    issue_id int NOT NULL,
    FOREIGN KEY (issue_id) REFERENCES issues(id),
    user_id int NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    parent_id int,
    body VARCHAR NOT NULL,
    posted_at Timestamp NOT NULL
);

INSERT INTO issues (user_id, title, body, tags, is_open, posted_at)
VALUES (1, 'UI is broken in Mac', 'As you can see below UI is broken in Mac', 'MacOs, bug, feature: UI', TRUE, '2022-12-23T04:30:12.249335'),
        (1, 'Suggestion, allow to close multiple tabs at once', 'Basically the title', 'enhancement, feature: tabs', TRUE, '2022-12-24T04:30:12.249335'),
        (1, 'Need help with error', 'See the attached image, can anyone help solve this error', 'error, help needed', FALSE, '2022-12-24T04:30:12.249335'),
        (1, 'Improvement: Add a back button', 'Add a back button to the app so I can go back to the previous page I was in', 'enhancement, feature: back button', TRUE, '2022-12-25T04:30:12.249335');

INSERT INTO users (first_name, last_name, user_role, added)
VALUES ('John', 'Doe', 'user', '2022-12-25T04:30:12.249335'),
        ('Gabriel', 'Alfonso', 'developer', '2022-12-26T04:30:12.249335'),
        ('Matthew', 'Collahann', 'admin', '2022-12-27T04:30:12.249335');

INSERT INTO comments (issue_id, user_id, parent_id, body, posted_at)
VALUES (2, 1, NULL,'I also think handling multiple tabs at once would be pretty helpful. Upvote.', '2022-12-23T04:30:12.249335'),
        (2, 2, 1, 'The developer team can see the value in selecting mutlitple tabs and being able to act on this collection of tabs, however this is not a trivial feature to implement and would take some time. Unfortunaly, this is not a priority at the moment as we have more core features being developed and others getting fleshed out or outright fixed.', '2022-12-24T04:30:12.249335'),
        (4, 3, NULL, 'I agree with you that a back button would massibly improve the user experience. So much in fact that we are already working on implementing one. At the latest we should be releasing the update in 2-3 months.', '2022-12-24T04:30:12.249335');