table! {
    entries (id, date) {
        id -> Int4,
        date -> Timestamptz,
        entry_type -> Varchar,
        sgv -> Nullable<Float8>,
        direction -> Nullable<Varchar>,
        noise -> Nullable<Float8>,
        filtered -> Nullable<Float8>,
        unfiltered -> Nullable<Float8>,
        rssi -> Nullable<Float8>,
    }
}
