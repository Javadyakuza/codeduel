CREATE TABLE users (
    user_id SERIAL PRIMARY KEY NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    total_payed INTEGER NOT NULL DEFAULT 0,
    total_claimed INTEGER NOT NULL DEFAULT 0, 
    total_unclaimed INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE questions (
    question_id SERIAL PRIMARY KEY NOT NULL,
    rival_id INTEGER NOT NULL REFERENCES users(user_id),
    question_title VARCHAR(64) NOT NULL UNIQUE,
    question_body TEXT NOT NULL,
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deadline TIMESTAMP NOT NULL,
    question_status INTEGER NOT NULL DEFAULT 1,
    daredevil INTEGER DEFAULT NULL,
    reward INTEGER NOT NULL,
    prize_pool INTEGER NOT NULL DEFAULT 0,
    entrance_fee FLOAT4 NOT NULL, 
    category VARCHAR(64) NOT NULL
);
CREATE TABLE responses (
    response_id SERIAL PRIMARY KEY NOT NULL,
    daredevil_id INTEGER NOT NULL REFERENCES users(user_id),
    question_id INTEGER NOT NULL REFERENCES questions(question_id),
    response_code TEXT NOT NULL,
    correctness BOOLEAN NOT NULL,
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_daredevil_question UNIQUE (daredevil_id, question_id)
);
CREATE TABLE test_cases (
    test_case_id SERIAL PRIMARY KEY NOT NULL,
    question_id INTEGER NOT NULL REFERENCES questions(question_id),
    executable_solution TEXT NOT NULL,
    solution_executer TEXT NOT NULL, 
    test_inputs TEXT NOT NULL,
    test_outputs TEXT NOT NULL
);
CREATE TABLE wallets (
    user_id INTEGER NOT NULL REFERENCES users(user_id),
    sol_addr VARCHAR(64) NOT NULL,
    PRIMARY KEY (user_id, sol_addr)
);