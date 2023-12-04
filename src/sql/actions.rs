use crate::sql::query_generators;
use crate::utils::{cli_input, parse_clipboard};

pub fn insert() {
    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    query_generators::insert(table_name, headers, data);
}

pub fn update() {
    let where_fields: Vec<String> = cli_input("Where Fields:")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    query_generators::update(table_name, headers, data, where_fields);
}

pub fn insert_bulk() {
    let bulk_length = cli_input("Bulk length:").parse::<i32>().unwrap();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    query_generators::bulk_insert(table_name, headers, data, bulk_length);
}

pub fn delete() {
    let where_fields: Vec<String> = cli_input("Where Fields:")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let table_name = get_table_name();

    let (headers, data) = parse_clipboard();

    query_generators::delete(table_name, headers, data, where_fields);
}

fn get_table_name() -> String {
    let table_name = format!("`{}`", cli_input("Table name:"));
    cli_input("Copy data into clipboard and hit ENTER!");

    return table_name;
}
