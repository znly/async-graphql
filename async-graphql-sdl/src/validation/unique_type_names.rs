use crate::visitor::{Visitor, VisitorContext};
use async_graphql_parser::schema::TypeDefinition;
use async_graphql_parser::{Pos, Positioned};
use std::collections::HashMap;

#[derive(Default)]
pub struct UniqueTypeNames<'a> {
    names: HashMap<&'a str, Pos>,
}

impl<'a> Visitor<'a> for UniqueTypeNames<'a> {
    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        if let Some(prev_pos) = self
            .names
            .insert(type_definition.name(), type_definition.position())
        {
            ctx.report_error(
                vec![type_definition.position(), prev_pos],
                format!(
                    "There can only be one type named \"{}\"",
                    type_definition.name()
                ),
            );
        }
    }
}
