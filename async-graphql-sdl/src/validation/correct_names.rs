use crate::visitor::{Visitor, VisitorContext};
use async_graphql_parser::schema::{
    Field, InputObjectType, InterfaceType, ObjectType, TypeDefinition,
};
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
                vec![type_definition.name().position()],
                format!("Type names cannot be prefixed with \"__\" two underscores.",),
            );
        }
    }

    fn enter_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        object_definition: &'a Positioned<ObjectType>,
    ) {
        check_correct_field_names(ctx, &object_definition.fields);
    }

    fn enter_interface_definition(
        &mut self,
        ctx: &mut VisitorContext,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
        check_correct_field_names(ctx, &interface_definition.fields);
    }

    fn enter_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
        for field in &input_object_definition.fields {
            if field.name.starts_with("__") {
                ctx.report_error(
                    vec![field.position()],
                    format!("Field names cannot be prefixed with \"__\" two underscores."),
                );
            }
        }
    }
}

fn check_correct_field_names(ctx: &mut VisitorContext, fields: &[Positioned<Field>]) {
    for field in fields {
        if field.name.starts_with("__") {
            ctx.report_error(
                vec![field.position()],
                format!("Field names cannot be prefixed with \"__\" two underscores."),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness::{expect_fails_rule, expect_passes_rule};

    pub fn factory<'a>() -> CorrectNames {
        CorrectNames
    }

    #[test]
    fn good_field_names() {
        expect_passes_rule(
            factory,
            r#"
            input MyObj {
                a: Int,
            }
            
            interface MyInterface {
                a: Int,
            }
            
            type Query {
                obj: MyObj,
            }
        "#,
        );
    }

    #[test]
    fn incorrect_object_field_name() {
        expect_fails_rule(
            factory,
            r#"
            type MyObj {
                __a: Int,
            }
        "#,
        );
    }

    #[test]
    fn incorrect_interface_field_name() {
        expect_fails_rule(
            factory,
            r#"
            interface MyInterface {
                __a: Int,
            }
        "#,
        );
    }

    #[test]
    fn incorrect_input_object_field_name() {
        expect_fails_rule(
            factory,
            r#"
            input MyInput {
                __a: Int,
            }
        "#,
        );
    }
}
