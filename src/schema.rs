table! {
    authors (id) {
        id -> Integer,
    }
}

table! {
    books (BookID) {
        BookID -> Integer,
        Title -> Varchar,
        SeriesID -> Nullable<Integer>,
        AuthorID -> Nullable<Integer>,
    }
}

table! {
    expenses (purchase_id) {
        purchase_id -> Unsigned<Bigint>,
        item_name -> Nullable<Varchar>,
        secondary_name -> Nullable<Varchar>,
        cost -> Nullable<Decimal>,
        purchase_date -> Nullable<Date>,
        fk_payment_type_id -> Nullable<Integer>,
        fk_categories_id -> Nullable<Integer>,
        fk_person_id -> Nullable<Integer>,
    }
}

table! {
    pics (pic_id) {
        pic_id -> Unsigned<Integer>,
        pic_path -> Nullable<Varchar>,
        quality -> Nullable<Bool>,
        fk_works_id -> Nullable<Integer>,
    }
}

table! {
    primary_key_dat (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
    }
}

table! {
    series (id) {
        id -> Integer,
    }
}

table! {
    works (id) {
        id -> Unsigned<Integer>,
        work_name -> Nullable<Varchar>,
        creation_date -> Nullable<Date>,
        location -> Nullable<Text>,
        bequeathment -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        medium -> Nullable<Varchar>,
        comment -> Nullable<Mediumtext>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    expenses,
    pics,
    primary_key_dat,
    series,
    works,
);
