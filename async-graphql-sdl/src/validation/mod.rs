mod unique_type_names;

use crate::visitor::{visit, RuleError, VisitorContext, VisitorNil};
use async_graphql_parser::schema::Document;
use unique_type_names::UniqueTypeNames;

pub fn check_rules(doc: &Document) -> Result<(), Vec<RuleError>> {
    let mut visitor = VisitorNil.with(UniqueTypeNames::default());

    let mut ctx = VisitorContext { errors: Vec::new() };
    visit(&mut visitor, &mut ctx, doc);
    if ctx.errors.is_empty() {
        Ok(())
    } else {
        Err(ctx.errors)
    }
}
