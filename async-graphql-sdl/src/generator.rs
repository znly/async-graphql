use crate::{directives, Error};
use anyhow::Result;
use async_graphql_parser::schema::{
    Definition, Document, Field, ObjectType, ScalarType, Type, TypeDefinition,
};
use async_graphql_parser::Positioned;
use inflector::Inflector;
use proc_macro2::{Ident, Span, TokenStream};
use std::collections::HashMap;

struct Context<'a> {
    doc: &'a Document,
    types: HashMap<&'a str, &'a Positioned<TypeDefinition>>,
    scalars: HashMap<String, String>,
}

impl<'a> Context<'a> {
    fn new(doc: &'a Document) -> Result<Self> {
        let mut types = HashMap::new();
        let mut scalars = HashMap::new();

        for definition in &doc.definitions {
            if let Definition::TypeDefinition(type_definition) = &definition.node {
                types.insert(*type_definition.name(), type_definition);
                if let TypeDefinition::Scalar(scalar) = &type_definition.node {
                    scalars.insert(
                        scalar.name.clone_inner(),
                        directives::scalar_type(&scalar.directives)?
                            .unwrap_or("String")
                            .to_string(),
                    );
                }
            }
        }

        Ok(Self {
            doc,
            types,
            scalars,
        })
    }
}

pub fn generate(doc: &Document) -> Result<TokenStream> {
    let ctx = Context::new(doc)?;
    let mut types = Vec::new();

    for ty in ctx.types.values() {
        match &ty.node {
            TypeDefinition::Scalar(scalar) => {}
            TypeDefinition::Object(object) => {
                types.push(generate_object(&ctx, object)?);
            }
            TypeDefinition::Interface(_) => {}
            TypeDefinition::Union(_) => {}
            TypeDefinition::Enum(_) => {}
            TypeDefinition::InputObject(_) => {}
        }
    }

    Ok(quote! { #(#types)* })
}

fn generate_object(ctx: &Context, object: &Positioned<ObjectType>) -> Result<TokenStream> {
    let name = object.name.as_str();
    let fields_name = Ident::new(&format!("{}Fields", name), Span::call_site());
    let mut trait_fields = Vec::new();
    let mut fields = Vec::new();

    for field in &object.fields {
        trait_fields.push(generate_object_trait_field(ctx, field)?);
    }

    for field in &object.fields {
        fields.push(generate_object_field(ctx, field)?);
    }

    Ok(quote! {
        #[async_graphql::async_trait::async_trait]
        pub trait #fields_name {
            #(#trait_fields)*
        }

        #[async_graphql::Object]
        impl #name where Self: #fields_name {
            #(#fields)*
        }
    })
}

fn generate_object_trait_field(ctx: &Context, field: &Positioned<Field>) -> Result<TokenStream> {
    let name = Ident::new(&field.name.as_str().to_table_case(), Span::call_site());
    Ok(quote! {
        async fn #name(&self) -> Int;
    })
}

fn generate_object_field(ctx: &Context, field: &Positioned<Field>) -> Result<TokenStream> {
    Ok(quote! {})
}

fn rust_type(ctx: &Context, ty: &Type) -> Result<TokenStream> {
    match ty {
        Type::NonNull(ty) => match &**ty {
            Type::List(ty) => {
                let ty = rust_type(ctx, ty)?;
                Ok(quote! { Vec<#ty> })
            }
            Type::Named(ty) => {
                let ty = concrete_rust_type(ctx, ty)?;
                Ok(quote! { #ty })
            }
            Type::NonNull(_) => unreachable!(),
        },
        Type::List(ty) => {
            let ty = rust_type(ctx, ty)?;
            Ok(quote! { Option<Vec<#ty>> })
        }
        Type::Named(ty) => {
            let ty = concrete_rust_type(ctx, ty)?;
            Ok(quote! { Option<#ty> })
        }
    }
}

fn concrete_rust_type(ctx: &Context, name: &str) -> Result<TokenStream> {
    match name {
        "Int" => Ok(quote! { i32 }),
        "Int64" => Ok(quote! { i64 }),
        "String" => Ok(quote! { String }),
        "Boolean" => Ok(quote! { bool }),
        "ID" => Ok(quote! { async_graphql::ID }),
        _ => {
            if let Some(rust_ty) = ctx.scalars.get(name) {
                let ty = syn::parse_str(rust_ty)?;
                Ok(quote! { #ty })
            } else {
                bail!(Error{
                    locations: vec![],
                })
            }
        }
    }
}
