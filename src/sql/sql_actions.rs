use crate::sql::sql_query_generators::{
    bulk_insert_query, delete_query, insert_query, update_query,
};
use crate::utils::{cli_input, parse_clipboard};

pub fn insert() {
    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    insert_query(table_name, headers, data);
}

pub fn update() {
    let where_fields: Vec<String> = cli_input("Where Fields:")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    update_query(table_name, headers, data, where_fields);
}

pub fn insert_bulk() {
    let bulk_length = cli_input("Bulk length:").parse::<i32>().unwrap();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    bulk_insert_query(table_name, headers, data, bulk_length);
}

pub fn delete() {
    let where_fields: Vec<String> = cli_input("Where Fields:")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    delete_query(table_name, headers, data, where_fields);
}

fn get_table_name() -> String {
    let table_name = format!("`{}`", cli_input("Table name:"));
    cli_input("Copy data into clipboard and hit ENTER!");

    return table_name;
}
