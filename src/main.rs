use cli_sql_actions::{sql::sql_enum::SqlEnum, utils::cli_select};

fn main() {
    let actions: Vec<SqlEnum> = vec![
        SqlEnum::Insert,
        SqlEnum::Update,
        SqlEnum::BulkInsert,
        SqlEnum::Delete,
    ];

    let selections: Vec<String> = actions.iter().map(|s| s.to_string()).collect();

    let action: &SqlEnum = match cli_select(selections) {
        Some(index) => &actions[index],
        None => panic!(),
    };

    action.run();
}
