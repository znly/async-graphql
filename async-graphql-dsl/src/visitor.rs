use async_graphql_parser::schema::{
    Definition, DirectiveDefinition, Document, EnumType, InputObjectType, InterfaceType,
    ObjectType, ScalarType, SchemaDefinition, TypeDefinition, UnionType,
};
use async_graphql_parser::{Pos, Positioned};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RuleError {
    pub locations: Vec<Pos>,
    pub message: String,
}

pub struct VisitorContext<'a> {
    pub errors: Vec<RuleError>,
}

#[allow(unused_variables)]
pub trait Visitor<'a> {
    fn enter_document(&mut self, ctx: &mut VisitorContext<'a>, doc: &'a Document) {}
    fn exit_document(&mut self, ctx: &mut VisitorContext<'a>, doc: &'a Document) {}

    fn enter_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        definition: &'a Positioned<Definition>,
    ) {
    }
    fn exit_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        definition: &'a Positioned<Definition>,
    ) {
    }

    fn enter_schema_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
    }
    fn exit_schema_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
    }

    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
    }
    fn exit_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
    }

    fn enter_directive_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
    }
    fn exit_directive_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
    }

    fn enter_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
    }
    fn exit_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
    }

    fn enter_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        object_definition: &'a Positioned<ObjectType>,
    ) {
    }
    fn exit_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        object_definition: &'a Positioned<ObjectType>,
    ) {
    }

    fn enter_interface_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
    }
    fn exit_interface_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
    }

    fn enter_union_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        union_definition: &'a Positioned<UnionType>,
    ) {
    }
    fn exit_union_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        union_definition: &'a Positioned<UnionType>,
    ) {
    }

    fn enter_enum_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        enum_definition: &'a Positioned<EnumType>,
    ) {
    }
    fn exit_enum_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        enum_definition: &'a Positioned<EnumType>,
    ) {
    }

    fn enter_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
    }
    fn exit_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
    }
}

pub struct VisitorNil;

impl VisitorNil {
    pub fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

pub struct VisitorCons<A, B>(A, B);

impl<A, B> VisitorCons<A, B> {
    pub fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

impl<'a> Visitor<'a> for VisitorNil {}

impl<'a, A, B> Visitor<'a> for VisitorCons<A, B>
where
    A: Visitor<'a> + 'a,
    B: Visitor<'a> + 'a,
{
    fn enter_document(&mut self, ctx: &mut VisitorContext<'a>, doc: &'a Document) {
        self.0.enter_document(ctx, doc);
        self.1.enter_document(ctx, doc);
    }

    fn exit_document(&mut self, ctx: &mut VisitorContext<'a>, doc: &'a Document) {
        self.0.exit_document(ctx, doc);
        self.1.exit_document(ctx, doc);
    }

    fn enter_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        definition: &'a Positioned<Definition>,
    ) {
        self.0.enter_definition(ctx, definition);
        self.1.enter_definition(ctx, definition);
    }

    fn exit_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        definition: &'a Positioned<Definition>,
    ) {
        self.0.exit_definition(ctx, definition);
        self.1.exit_definition(ctx, definition);
    }

    fn enter_schema_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
        self.0.enter_schema_definition(ctx, schema_definition);
        self.1.enter_schema_definition(ctx, schema_definition);
    }

    fn exit_schema_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
        self.0.exit_schema_definition(ctx, schema_definition);
        self.1.exit_schema_definition(ctx, schema_definition);
    }

    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        self.0.enter_type_definition(ctx, type_definition);
        self.1.enter_type_definition(ctx, type_definition);
    }

    fn exit_type_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        self.0.exit_type_definition(ctx, type_definition);
        self.1.exit_type_definition(ctx, type_definition);
    }

    fn enter_directive_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
        self.0.enter_directive_definition(ctx, directive_definition);
        self.1.enter_directive_definition(ctx, directive_definition);
    }

    fn exit_directive_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
        self.0.exit_directive_definition(ctx, directive_definition);
        self.1.exit_directive_definition(ctx, directive_definition);
    }

    fn enter_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
        self.0.enter_scalar_definition(ctx, scalar_definition);
        self.1.enter_scalar_definition(ctx, scalar_definition);
    }

    fn exit_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
        self.0.exit_scalar_definition(ctx, scalar_definition);
        self.1.exit_scalar_definition(ctx, scalar_definition);
    }

    fn enter_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        object_definition: &'a Positioned<ObjectType>,
    ) {
        self.0.enter_object_definition(ctx, object_definition);
        self.1.enter_object_definition(ctx, object_definition);
    }

    fn exit_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        object_definition: &'a Positioned<ObjectType>,
    ) {
        self.0.exit_object_definition(ctx, object_definition);
        self.1.exit_object_definition(ctx, object_definition);
    }

    fn enter_interface_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
        self.0.enter_interface_definition(ctx, interface_definition);
        self.1.enter_interface_definition(ctx, interface_definition);
    }

    fn exit_interface_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
        self.0.exit_interface_definition(ctx, interface_definition);
        self.1.exit_interface_definition(ctx, interface_definition);
    }

    fn enter_union_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        union_definition: &'a Positioned<UnionType>,
    ) {
        self.0.enter_union_definition(ctx, union_definition);
        self.1.enter_union_definition(ctx, union_definition);
    }

    fn exit_union_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        union_definition: &'a Positioned<UnionType>,
    ) {
        self.0.exit_union_definition(ctx, union_definition);
        self.1.exit_union_definition(ctx, union_definition);
    }

    fn enter_enum_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        enum_definition: &'a Positioned<EnumType>,
    ) {
        self.0.enter_enum_definition(ctx, enum_definition);
        self.1.enter_enum_definition(ctx, enum_definition);
    }

    fn exit_enum_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        enum_definition: &'a Positioned<EnumType>,
    ) {
        self.0.exit_enum_definition(ctx, enum_definition);
        self.1.exit_enum_definition(ctx, enum_definition);
    }

    fn enter_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
        self.0
            .enter_input_object_definition(ctx, input_object_definition);
        self.1
            .enter_input_object_definition(ctx, input_object_definition);
    }

    fn exit_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext<'a>,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
        self.0
            .exit_input_object_definition(ctx, input_object_definition);
        self.1
            .exit_input_object_definition(ctx, input_object_definition);
    }
}

pub fn visit<'a, V: Visitor<'a>>(v: &mut V, ctx: &mut VisitorContext<'a>, doc: &'a Document) {
    v.enter_document(ctx, doc);
    visit_definitions(v, ctx, doc);
    v.exit_document(ctx, doc);
}

fn visit_definitions<'a, V: Visitor<'a>>(
    v: &mut V,
    ctx: &mut VisitorContext<'a>,
    doc: &'a Document,
) {
}
