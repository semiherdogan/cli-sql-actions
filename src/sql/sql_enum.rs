use crate::sql::actions;
use std::fmt;

pub enum SqlEnum {
    Insert,
    Update,
    BulkInsert,
    Delete,
}

impl SqlEnum {
    pub fn run(&self) {
        match self {
            SqlEnum::Insert => actions::insert(),
            SqlEnum::Update => actions::update(),
            SqlEnum::BulkInsert => actions::insert_bulk(),
            SqlEnum::Delete => actions::delete(),
        }
    }
}

impl fmt::Display for SqlEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SqlEnum::Insert => write!(f, "Insert"),
            SqlEnum::Update => write!(f, "Update"),
            SqlEnum::BulkInsert => write!(f, "Bulk Insert"),
            SqlEnum::Delete => write!(f, "Delete"),
        }
    }
}
