use async_graphql_parser::schema::{
    Definition, DirectiveDefinition, Document, EnumType, InputObjectType, InterfaceType,
    ObjectType, ScalarType, SchemaDefinition, TypeDefinition, UnionType,
};
use async_graphql_parser::{Pos, Positioned};

#[derive(Debug, PartialEq)]
pub struct RuleError {
    pub locations: Vec<Pos>,
    pub message: String,
}

pub struct VisitorContext {
    pub errors: Vec<RuleError>,
}

impl VisitorContext {
    pub fn report_error<T: Into<String>>(&mut self, locations: Vec<Pos>, msg: T) {
        self.errors.push(RuleError {
            locations,
            message: msg.into(),
        })
    }
}

#[allow(unused_variables)]
pub trait Visitor<'a> {
    fn enter_document(&mut self, ctx: &mut VisitorContext, doc: &'a Document) {}
    fn exit_document(&mut self, ctx: &mut VisitorContext, doc: &'a Document) {}

    fn enter_definition(
        &mut self,
        ctx: &mut VisitorContext,
        definition: &'a Positioned<Definition>,
    ) {
    }
    fn exit_definition(
        &mut self,
        ctx: &mut VisitorContext,
        definition: &'a Positioned<Definition>,
    ) {
    }

    fn enter_schema_definition(
        &mut self,
        ctx: &mut VisitorContext,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
    }
    fn exit_schema_definition(
        &mut self,
        ctx: &mut VisitorContext,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
    }

    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
    }
    fn exit_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
    }

    fn enter_directive_definition(
        &mut self,
        ctx: &mut VisitorContext,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
    }
    fn exit_directive_definition(
        &mut self,
        ctx: &mut VisitorContext,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
    }

    fn enter_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
    }
    fn exit_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
    }

    fn enter_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        object_definition: &'a Positioned<ObjectType>,
    ) {
    }
    fn exit_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        object_definition: &'a Positioned<ObjectType>,
    ) {
    }

    fn enter_interface_definition(
        &mut self,
        ctx: &mut VisitorContext,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
    }
    fn exit_interface_definition(
        &mut self,
        ctx: &mut VisitorContext,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
    }

    fn enter_union_definition(
        &mut self,
        ctx: &mut VisitorContext,
        union_definition: &'a Positioned<UnionType>,
    ) {
    }
    fn exit_union_definition(
        &mut self,
        ctx: &mut VisitorContext,
        union_definition: &'a Positioned<UnionType>,
    ) {
    }

    fn enter_enum_definition(
        &mut self,
        ctx: &mut VisitorContext,
        enum_definition: &'a Positioned<EnumType>,
    ) {
    }
    fn exit_enum_definition(
        &mut self,
        ctx: &mut VisitorContext,
        enum_definition: &'a Positioned<EnumType>,
    ) {
    }

    fn enter_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
    }
    fn exit_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
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
    A: Visitor<'a>,
    B: Visitor<'a>,
{
    fn enter_document(&mut self, ctx: &mut VisitorContext, doc: &'a Document) {
        self.0.enter_document(ctx, doc);
        self.1.enter_document(ctx, doc);
    }

    fn exit_document(&mut self, ctx: &mut VisitorContext, doc: &'a Document) {
        self.0.exit_document(ctx, doc);
        self.1.exit_document(ctx, doc);
    }

    fn enter_definition(
        &mut self,
        ctx: &mut VisitorContext,
        definition: &'a Positioned<Definition>,
    ) {
        self.0.enter_definition(ctx, definition);
        self.1.enter_definition(ctx, definition);
    }

    fn exit_definition(
        &mut self,
        ctx: &mut VisitorContext,
        definition: &'a Positioned<Definition>,
    ) {
        self.0.exit_definition(ctx, definition);
        self.1.exit_definition(ctx, definition);
    }

    fn enter_schema_definition(
        &mut self,
        ctx: &mut VisitorContext,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
        self.0.enter_schema_definition(ctx, schema_definition);
        self.1.enter_schema_definition(ctx, schema_definition);
    }

    fn exit_schema_definition(
        &mut self,
        ctx: &mut VisitorContext,
        schema_definition: &'a Positioned<SchemaDefinition>,
    ) {
        self.0.exit_schema_definition(ctx, schema_definition);
        self.1.exit_schema_definition(ctx, schema_definition);
    }

    fn enter_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        self.0.enter_type_definition(ctx, type_definition);
        self.1.enter_type_definition(ctx, type_definition);
    }

    fn exit_type_definition(
        &mut self,
        ctx: &mut VisitorContext,
        type_definition: &'a Positioned<TypeDefinition>,
    ) {
        self.0.exit_type_definition(ctx, type_definition);
        self.1.exit_type_definition(ctx, type_definition);
    }

    fn enter_directive_definition(
        &mut self,
        ctx: &mut VisitorContext,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
        self.0.enter_directive_definition(ctx, directive_definition);
        self.1.enter_directive_definition(ctx, directive_definition);
    }

    fn exit_directive_definition(
        &mut self,
        ctx: &mut VisitorContext,
        directive_definition: &'a Positioned<DirectiveDefinition>,
    ) {
        self.0.exit_directive_definition(ctx, directive_definition);
        self.1.exit_directive_definition(ctx, directive_definition);
    }

    fn enter_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
        self.0.enter_scalar_definition(ctx, scalar_definition);
        self.1.enter_scalar_definition(ctx, scalar_definition);
    }

    fn exit_scalar_definition(
        &mut self,
        ctx: &mut VisitorContext,
        scalar_definition: &'a Positioned<ScalarType>,
    ) {
        self.0.exit_scalar_definition(ctx, scalar_definition);
        self.1.exit_scalar_definition(ctx, scalar_definition);
    }

    fn enter_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        object_definition: &'a Positioned<ObjectType>,
    ) {
        self.0.enter_object_definition(ctx, object_definition);
        self.1.enter_object_definition(ctx, object_definition);
    }

    fn exit_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        object_definition: &'a Positioned<ObjectType>,
    ) {
        self.0.exit_object_definition(ctx, object_definition);
        self.1.exit_object_definition(ctx, object_definition);
    }

    fn enter_interface_definition(
        &mut self,
        ctx: &mut VisitorContext,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
        self.0.enter_interface_definition(ctx, interface_definition);
        self.1.enter_interface_definition(ctx, interface_definition);
    }

    fn exit_interface_definition(
        &mut self,
        ctx: &mut VisitorContext,
        interface_definition: &'a Positioned<InterfaceType>,
    ) {
        self.0.exit_interface_definition(ctx, interface_definition);
        self.1.exit_interface_definition(ctx, interface_definition);
    }

    fn enter_union_definition(
        &mut self,
        ctx: &mut VisitorContext,
        union_definition: &'a Positioned<UnionType>,
    ) {
        self.0.enter_union_definition(ctx, union_definition);
        self.1.enter_union_definition(ctx, union_definition);
    }

    fn exit_union_definition(
        &mut self,
        ctx: &mut VisitorContext,
        union_definition: &'a Positioned<UnionType>,
    ) {
        self.0.exit_union_definition(ctx, union_definition);
        self.1.exit_union_definition(ctx, union_definition);
    }

    fn enter_enum_definition(
        &mut self,
        ctx: &mut VisitorContext,
        enum_definition: &'a Positioned<EnumType>,
    ) {
        self.0.enter_enum_definition(ctx, enum_definition);
        self.1.enter_enum_definition(ctx, enum_definition);
    }

    fn exit_enum_definition(
        &mut self,
        ctx: &mut VisitorContext,
        enum_definition: &'a Positioned<EnumType>,
    ) {
        self.0.exit_enum_definition(ctx, enum_definition);
        self.1.exit_enum_definition(ctx, enum_definition);
    }

    fn enter_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
        self.0
            .enter_input_object_definition(ctx, input_object_definition);
        self.1
            .enter_input_object_definition(ctx, input_object_definition);
    }

    fn exit_input_object_definition(
        &mut self,
        ctx: &mut VisitorContext,
        input_object_definition: &'a Positioned<InputObjectType>,
    ) {
        self.0
            .exit_input_object_definition(ctx, input_object_definition);
        self.1
            .exit_input_object_definition(ctx, input_object_definition);
    }
}

pub fn visit<'a, V: Visitor<'a>>(v: &mut V, ctx: &mut VisitorContext, doc: &'a Document) {
    v.enter_document(ctx, doc);
    visit_definitions(v, ctx, doc);
    v.exit_document(ctx, doc);
}

fn visit_definitions<'a, V: Visitor<'a>>(v: &mut V, ctx: &mut VisitorContext, doc: &'a Document) {
    for definition in &doc.definitions {
        visit_definition(v, ctx, definition);
    }
}

fn visit_definition<'a, V: Visitor<'a>>(
    v: &mut V,
    ctx: &mut VisitorContext,
    definition: &'a Positioned<Definition>,
) {
    v.enter_definition(ctx, definition);
    match &definition.node {
        Definition::SchemaDefinition(schema_definition) => {
            v.enter_schema_definition(ctx, schema_definition);
            v.exit_schema_definition(ctx, schema_definition);
        }
        Definition::TypeDefinition(schema_definition) => match &schema_definition.node {
            TypeDefinition::Scalar(scalar) => {
                v.enter_scalar_definition(ctx, scalar);
                v.exit_scalar_definition(ctx, scalar);
            }
            TypeDefinition::Object(object) => {
                v.enter_object_definition(ctx, object);
                v.exit_object_definition(ctx, object);
            }
            TypeDefinition::Interface(interface) => {
                v.enter_interface_definition(ctx, interface);
                v.exit_interface_definition(ctx, interface);
            }
            TypeDefinition::Union(union) => {
                v.enter_union_definition(ctx, union);
                v.exit_union_definition(ctx, union);
            }
            TypeDefinition::Enum(enum_type) => {
                v.enter_enum_definition(ctx, enum_type);
                v.exit_enum_definition(ctx, enum_type);
            }
            TypeDefinition::InputObject(input_object) => {
                v.enter_input_object_definition(ctx, input_object);
                v.exit_input_object_definition(ctx, input_object);
            }
        },
        Definition::DirectiveDefinition(directive_definition) => {
            v.enter_directive_definition(ctx, directive_definition);
            v.exit_directive_definition(ctx, directive_definition);
        }
    }
    v.exit_definition(ctx, definition);
}
