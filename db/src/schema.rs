table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        done -> Bool,
        updated_at -> Timestamp,
        due -> Nullable<Timestamp>,
        description -> Nullable<Text>,
    }
}
