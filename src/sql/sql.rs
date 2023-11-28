use crate::sql::sql_actions;
use std::fmt;

pub enum Sql {
    Insert,
    Update,
    BulkInsert,
    Delete,
}

impl Sql {
    pub fn run(&self) {
        match self {
            Sql::Insert => sql_actions::insert(),
            Sql::Update => sql_actions::update(),
            Sql::BulkInsert => sql_actions::insert_bulk(),
            Sql::Delete => sql_actions::delete(),
        }
    }
}

impl fmt::Display for Sql {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sql::Insert => write!(f, "Insert"),
            Sql::Update => write!(f, "Update"),
            Sql::BulkInsert => write!(f, "Bulk Insert"),
            Sql::Delete => write!(f, "Delete"),
        }
    }
}
