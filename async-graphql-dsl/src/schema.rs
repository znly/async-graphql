use async_graphql_parser::schema::{Definition, Document, TypeDefinition};
use std::ops::Deref;

pub struct Schema(Document);

impl Deref for Schema {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Schema {
    fn input_type_exists(&self, name: &str) -> bool {
        for definition in &self.0.definitions {
            if let Definition::TypeDefinition(ty) = &definition.node {
                match &ty.node {
                    TypeDefinition::Scalar(scalar) if scalar.name.as_str() == name => return true,
                    TypeDefinition::Enum(scalar) if scalar.name.as_str() == name => return true,
                    TypeDefinition::InputObject(scalar) if scalar.name.as_str() == name => {
                        return true
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn output_type_exists(&self, name: &str) -> bool {
        for definition in &self.0.definitions {
            if let Definition::TypeDefinition(ty) = &definition.node {
                match &ty.node {
                    TypeDefinition::Object(scalar) if scalar.name.as_str() == name => return true,
                    TypeDefinition::Interface(scalar) if scalar.name.as_str() == name => {
                        return true
                    }
                    TypeDefinition::Union(scalar) if scalar.name.as_str() == name => return true,
                    TypeDefinition::Enum(scalar) if scalar.name.as_str() == name => return true,
                    _ => {}
                }
            }
        }
        false
    }
}
