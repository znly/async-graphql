use crate::visitor::{Visitor, VisitorContext};
use async_graphql_parser::schema::{Definition, Document, SchemaDefinition, TypeDefinition};
use async_graphql_parser::Positioned;
use std::collections::HashSet;

#[derive(Default)]
pub struct RootOperationTypes<'a> {
    objects: HashSet<&'a str>,
}

impl<'a> Visitor<'a> for RootOperationTypes<'a> {
    fn enter_document(&mut self, ctx: &mut VisitorContext, doc: &'a Document) {
        for definition in &doc.definitions {
            if let Definition::TypeDefinition(type_definition) = &definition.node {
                if let TypeDefinition::Object(object_definiton) = &type_definition.node {
                    self.objects.insert(object_definiton.name.as_str());
                }
            }
        }
    }

    fn enter_schema_definition(
        &mut self,
        ctx: &mut VisitorContext,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
        if let Some(name) = &schema_definition.query {
            if !self.objects.contains(name.as_str()) {
                ctx.report_error(
                    vec![name.pos],
                    format!("Not defined or not an object type.",),
                );
            }
        } else {
            ctx.report_error(
                vec![schema_definition.position()],
                format!(
                    "The query root operation type must be provided and must be an Object type.",
                ),
            );
        }

        if let Some(name) = &schema_definition.mutation {
            if !self.objects.contains(name.as_str()) {
                ctx.report_error(
                    vec![name.pos],
                    format!("Not defined or not an object type.",),
                );
            }
        }

        if let Some(name) = &schema_definition.subscription {
            if !self.objects.contains(name.as_str()) {
                ctx.report_error(
                    vec![name.pos],
                    format!("Not defined or not an object type.",),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_harness::{expect_fails_rule, expect_passes_rule};

    pub fn factory<'a>() -> RootOperationTypes<'a> {
        RootOperationTypes::default()
    }

    #[test]
    fn good_operation_types() {
        expect_passes_rule(
            factory,
            r#"
            type Query
            type Mutation
            type Subscription

            schema {
                query: Query
                mutation: Mutation
                subscription: Subscription
            }
        "#,
        );
    }

    #[test]
    fn query_is_input() {
        expect_fails_rule(
            factory,
            r#"
            input Query
            type Mutation
            type Subscription

            schema {
                query: Query
                mutation: Mutation
                subscription: Subscription
            }
        "#,
        );
    }

    #[test]
    fn mutation_is_input() {
        expect_fails_rule(
            factory,
            r#"
            type Query
            input Mutation
            type Subscription

            schema {
                query: Query
                mutation: Mutation
                subscription: Subscription
            }
        "#,
        );
    }

    #[test]
    fn subscription_is_input() {
        expect_fails_rule(
            factory,
            r#"
            type Query
            type Mutation
            input Subscription

            schema {
                query: Query
                mutation: Mutation
                subscription: Subscription
            }
        "#,
        );
    }

    #[test]
    fn query_not_provided() {
        expect_fails_rule(
            factory,
            r#"
            type Mutation
            type Subscription

            schema {
                mutation: Mutation
                subscription: Subscription
            }
        "#,
        );
    }
}
