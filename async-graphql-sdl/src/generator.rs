use async_graphql_parser::schema::{Definition, Document, TypeDefinition};
use async_graphql_parser::Positioned;
use proc_macro2::TokenStream;
use std::collections::HashMap;

pub struct Generator<'a> {
    doc: &'a Document,
    types: HashMap<&'a str, &'a Positioned<TypeDefinition>>,
}

impl<'a> Generator<'a> {
    pub fn new(doc: &'a Document) -> Self {
        let mut types = HashMap::new();
        for definition in &doc.definitions {
            if let Definition::TypeDefinition(type_definition) = &definition.node {
                types.insert(*type_definition.name(), type_definition);
            }
        }
        Self { doc, types }
    }

    pub fn boilerplate(self) -> TokenStream {
        // let types = Vec::new();
        //
        // for ty in self.types.values() {
        //     match &ty.node {
        //         TypeDefinition::Scalar(scalar) => {
        //         }
        //         TypeDefinition::Object(_) => {}
        //         TypeDefinition::Interface(_) => {}
        //         TypeDefinition::Union(_) => {}
        //         TypeDefinition::Enum(_) => {}
        //         TypeDefinition::InputObject(_) => {}
        //     }
        // }
        //
        // quote! { #types }
        todo!()
    }
}
