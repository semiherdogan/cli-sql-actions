use cli_sql_actions::{sql::sql::Sql, utils::cli_select};

fn main() {
    let actions: Vec<Sql> = vec![Sql::Insert, Sql::Update, Sql::BulkInsert, Sql::Delete];

    let selections: Vec<String> = actions.iter().map(|s| s.to_string()).collect();

    let action: &Sql = match cli_select(selections) {
        Some(index) => &actions[index],
        None => panic!(),
    };

    action.run();
}
