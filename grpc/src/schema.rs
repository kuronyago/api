table! {
    projects (id) {
        id -> Int4,
        external -> Uuid,
        creator -> Uuid,
        free -> Int4,
        spent -> Int4,
        frozen -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
