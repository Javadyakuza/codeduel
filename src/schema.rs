// @generated automatically by Diesel CLI.

diesel::table! {
    questions (question_id) {
        question_id -> Int4,
        rival_id -> Int4,
        #[max_length = 64]
        question_title -> Varchar,
        question_body -> Text,
        creation_time -> Timestamp,
        deadline -> Timestamp,
        question_status -> Int4,
        daredevil -> Nullable<Int4>,
        #[max_length = 64]
        category -> Varchar,
    }
}

diesel::table! {
    responses (response_id) {
        response_id -> Int4,
        daredevil_id -> Int4,
        question_id -> Int4,
        response_code -> Text,
        correctness -> Bool,
        creation_time -> Timestamp,
    }
}

diesel::table! {
    test_cases (test_case_id) {
        test_case_id -> Int4,
        question_id -> Int4,
        test_inputs -> Text,
        test_outputs -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        total_payed -> Int4,
        total_claimed -> Int4,
    }
}

diesel::table! {
    wallets (user_id, sol_addr) {
        user_id -> Int4,
        #[max_length = 64]
        sol_addr -> Varchar,
    }
}

diesel::joinable!(questions -> users (rival_id));
diesel::joinable!(responses -> questions (question_id));
diesel::joinable!(responses -> users (daredevil_id));
diesel::joinable!(test_cases -> questions (question_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    questions,
    responses,
    test_cases,
    users,
    wallets,
);
