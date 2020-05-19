use crate::Error;
use anyhow::Result;
use async_graphql_parser::schema::Directive;
use async_graphql_parser::{Positioned, Value};

pub fn scalar_type(directives: &[Positioned<Directive>]) -> Result<Option<&str>> {
    let name = directives
        .iter()
        .find(|directive| directive.name.as_str() == "type")
        .and_then(|directive| {
            directive
                .arguments
                .iter()
                .find(|(name, _)| name.as_str() == "name")
                .map(|(_, value)| value)
        });
    if let Some(name) = name {
        if let Value::String(name) = &name.node {
            Ok(Some(name.as_str()))
        } else {
            bail!(Error {
                locations: vec![name.pos],
                message: "The \"name\" parameter type of the \"@type\" directive must be a string."
                    .to_string(),
            })
        }
    } else {
        Ok(None)
    }
}
