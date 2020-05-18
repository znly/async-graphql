use crate::validation::check_rules;
use anyhow::Result;
use async_graphql_parser::parse_schema;
use async_graphql_parser::schema::Document;
use std::fmt::Write;
use std::ops::Deref;

pub struct Schema(Document);

impl Deref for Schema {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Schema {
    pub fn parse(source: &str) -> Result<Self> {
        let doc = parse_schema(source)?;
        check_rules(&doc).map_err(|errors| {
            let mut output = String::new();
            for err in errors {
                writeln!(output, "{}", err).unwrap();
            }
            anyhow::anyhow!(output)
        })?;
        Ok(Schema(doc))
    }
}
