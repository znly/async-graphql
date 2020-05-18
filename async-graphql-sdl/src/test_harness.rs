use crate::visitor::{visit, RuleError, Visitor, VisitorContext};
use async_graphql_parser::parse_schema;

pub fn expect_passes_rule<'a, V, F>(factory: F, dsl: &str)
where
    V: Visitor<'a>,
    F: Fn() -> V,
{
    if let Err(errors) = validate(factory, dsl) {
        for err in errors {
            println!("{}", err);
        }
        panic!("Expected rule to pass, but errors found");
    }
}

pub fn expect_fails_rule<'a, V, F>(factory: F, dsl: &str)
where
    V: Visitor<'a>,
    F: Fn() -> V,
{
    if validate(factory, dsl).is_ok() {
        panic!("Expected rule to fail, but no errors were found");
    }
}

pub fn validate<'a, V, F>(factory: F, dsl: &str) -> Result<(), Vec<RuleError>>
where
    V: Visitor<'a>,
    F: Fn() -> V,
{
    let doc = parse_schema(dsl).expect("Parse error");
    let mut ctx = VisitorContext::default();
    let mut visitor = factory();
    visit(&mut visitor, &mut ctx, unsafe {
        ::std::mem::transmute(&doc)
    });
    if !ctx.errors.is_empty() {
        return Err(ctx.errors);
    }
    Ok(())
}
