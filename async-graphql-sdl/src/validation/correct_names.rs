use crate::visitor::{Visitor, VisitorContext};
use async_graphql_parser::schema::TypeDefinition;
use async_graphql_parser::Positioned;

pub struct CorrectNames;

impl<'a> Visitor<'a> for CorrectNames {
    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        if type_definition.name().node.starts_with("__") {
            ctx.report_error(
                vec![type_definition.name().pos],
                format!("Type names cannot be prefixed with \"__\" two underscores.",),
            );
        }
    }
}
