table! {
    devices (id) {
        id -> Bpchar,
        location -> Varchar,
        approved -> Bool,
    }
}

table! {
    envlogs (id) {
        id -> Int4,
        device_id -> Nullable<Bpchar>,
        created -> Timestamptz,
        temperature -> Numeric,
        humidity -> Numeric,
    }
}

joinable!(envlogs -> devices (device_id));

allow_tables_to_appear_in_same_query!(devices, envlogs,);
