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
            .insert(*type_definition.name(), type_definition.position())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness::{expect_fails_rule, expect_passes_rule};

    pub fn factory<'a>() -> UniqueTypeNames<'a> {
        UniqueTypeNames::default()
    }

    #[test]
    fn good_type_names() {
        expect_passes_rule(
            factory,
            r#"
            scalar A

            input B {
                a: Int
            }
        "#,
        );
    }

    #[test]
    fn duplicate_type_names() {
        expect_fails_rule(
            factory,
            r#"
            scalar A

            input A {
                a: Int
            }
        "#,
        );
    }
}
