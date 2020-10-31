table! {
    device (id) {
        id -> Int4,
        location -> Varchar,
    }
}

table! {
    envlog (id) {
        id -> Int4,
        device_id -> Nullable<Int4>,
        created -> Timestamptz,
        temperature -> Numeric,
        humidity -> Numeric,
    }
}

joinable!(envlog -> device (device_id));

allow_tables_to_appear_in_same_query!(
    device,
    envlog,
);
