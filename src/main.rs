use cli_sql_actions::{actions::Actions, utils::cli_select};

fn main() {
    let actions: Vec<Actions> = vec![
        Actions::SqlInsert,
        Actions::SqlUpdate,
        Actions::SqlInsertBulk,
    ];

    let selections: Vec<String> = actions.iter().map(|s| s.to_string()).collect();

    let action = match cli_select(selections) {
        Some(index) => &actions[index],
        None => panic!(),
    };

    action.run();
}
