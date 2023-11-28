use crate::sql;
use std::fmt;

pub enum Actions {
    SqlInsert,
    SqlUpdate,
    SqlInsertBulk,
}

impl Actions {
    pub fn run(&self) {
        match self {
            Actions::SqlInsert => sql::insert(),
            Actions::SqlUpdate => sql::update(),
            Actions::SqlInsertBulk => sql::insert_bulk(),
        }
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Actions::SqlInsert => write!(f, "Sql Insert"),
            Actions::SqlUpdate => write!(f, "Sql Update"),
            Actions::SqlInsertBulk => write!(f, "Sql Insert Bulk"),
        }
    }
}
