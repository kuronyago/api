table! {
    transfers (id) {
        id -> Int4,
        external -> Uuid,
        sender -> Uuid,
        recipient -> Uuid,
        issued -> Int4,
        gained -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
