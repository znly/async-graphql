use async_graphql_parser::Value;
use std::collections::HashMap;

pub enum DirectiveLocation {
    /// Location adjacent to a schema definition.
    Schema,

    /// Location adjacent to a scalar definition.
    Scalar,

    /// Location adjacent to an object type definition.
    Object,

    /// Location adjacent to a field definition.
    FieldDefinition,

    /// Location adjacent to an argument definition.
    ArgumentDefinition,

    /// Location adjacent to an interface definition.
    Interface,

    /// Location adjacent to a union definition.
    Union,

    /// Location adjacent to an enum definition.
    Enum,

    /// Location adjacent to an enum value definition.
    EnumValue,

    /// Location adjacent to an input object type definition.
    InputObject,

    /// Location adjacent to an input object field definition.
    InputFieldDefinition,
}

pub struct MetaDirective {
    pub name: &'static str,
    pub locations: Vec<DirectiveLocation>,
    pub args: HashMap<&'static str, MetaInputValue>,
}

pub struct MetaInputValue {
    pub name: &'static str,
    pub ty: &'static str,
    pub default_value: Value,
}
