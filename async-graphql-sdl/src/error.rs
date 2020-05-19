use async_graphql_parser::Pos;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub locations: Vec<Pos>,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locations = self
            .locations
            .iter()
            .map(|pos| format!("{}:{}", pos.line, pos.column))
            .join(", ");
        write!(f, "[{}]: {}", locations, self.message)?;
        Ok(())
    }
}
