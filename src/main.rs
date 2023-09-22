use cli_sql_actions::{actions::Actions, utils::cli_select};

fn main() {
    let actions = vec![
        Actions::SqlInsert.str(),
        Actions::SqlUpdate.str(),
        Actions::SqlInsertBulk.str(),
    ];

    match cli_select(actions.clone()) {
        0 => Actions::SqlInsert.run(),
        1 => Actions::SqlUpdate.run(),
        2 => Actions::SqlInsertBulk.run(),
        _ => panic!(),
    };
}
