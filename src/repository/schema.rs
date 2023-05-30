// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        red_cr -> Float4,
        yellow_cr -> Float4,
        blue_cr -> Nullable<Float4>,
        white_cr -> Nullable<Float4>,
        slope_red -> Int4,
        slope_yellow -> Int4,
        slope_blue -> Nullable<Int4>,
        slope_white -> Nullable<Int4>,
    }
}

diesel::table! {
    holes (id) {
        id -> Uuid,
        course_id -> Uuid,
        number -> Int4,
        par -> Int4,
        distance_red -> Int4,
        distance_yellow -> Int4,
        distance_blue -> Nullable<Int4>,
        distance_white -> Nullable<Int4>,
    }
}

diesel::table! {
    rounds (id) {
        id -> Uuid,
        course_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    scores (id) {
        id -> Uuid,
        round_id -> Uuid,
        hole_id -> Uuid,
        strokes -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        hcp -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(holes -> courses (course_id));
diesel::joinable!(rounds -> courses (course_id));
diesel::joinable!(scores -> holes (hole_id));
diesel::joinable!(scores -> rounds (round_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    holes,
    rounds,
    scores,
    users,
);
