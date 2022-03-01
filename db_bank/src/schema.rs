table! {
    accounts (id) {
        id -> Int4,
        num -> Varchar,
        amount -> Int4,
    }
}

table! {
    logtransfers (id) {
        id -> Int4,
        account_org -> Varchar,
        account_dest -> Varchar,
        amount -> Int4,
        date_tr -> Date,
    }
}

table! {
    transfers (account_id, user_id) {
        account_id -> Int4,
        user_id -> Int4,
        selected -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

joinable!(transfers -> accounts (account_id));
joinable!(transfers -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    logtransfers,
    transfers,
    users,
);
