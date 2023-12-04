use crate::sql::actions;
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
            Sql::Insert => actions::insert(),
            Sql::Update => actions::update(),
            Sql::BulkInsert => actions::insert_bulk(),
            Sql::Delete => actions::delete(),
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
