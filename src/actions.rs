// use std::fmt;

use crate::sql;

pub enum Actions {
    SqlInsert,
    SqlUpdate,
    SqlInsertBulk,
}

impl Actions {
    pub fn str(self) -> &'static str {
        match self {
            Actions::SqlInsert => "Sql Insert",
            Actions::SqlUpdate => "Sql Update",
            Actions::SqlInsertBulk => "Sql Insert Bulk",
        }
    }

    pub fn run(self) {
        match self {
            Actions::SqlInsert => sql::insert(),
            Actions::SqlUpdate => sql::update(),
            Actions::SqlInsertBulk => sql::insert_bulk(),
        }
    }
}

// impl fmt::Display for Actions {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Actions::SqlInsert => write!(f, "{}", Actions::SqlInsert.str()),
//             Actions::SqlUpdate => write!(f, "{}", Actions::SqlUpdate.str()),
//             Actions::SqlInsertBulk => write!(f, "{}", Actions::SqlInsertBulk.str()),
//         }
//     }
// }
