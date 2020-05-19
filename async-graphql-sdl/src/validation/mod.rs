mod correct_names;
mod root_operation_types;
mod unique_type_names;

use crate::visitor::{visit, VisitorContext, VisitorNil};
use crate::Error;
use async_graphql_parser::schema::Document;

use correct_names::CorrectNames;
use root_operation_types::RootOperationTypes;
use unique_type_names::UniqueTypeNames;

pub fn check_rules(doc: &Document) -> Result<(), Vec<Error>> {
    let mut visitor = VisitorNil
        .with(UniqueTypeNames::default())
        .with(RootOperationTypes::default())
        .with(CorrectNames);

    let mut ctx = VisitorContext { errors: Vec::new() };
    visit(&mut visitor, &mut ctx, doc);
    if ctx.errors.is_empty() {
        Ok(())
    } else {
        Err(ctx.errors)
    }
}
