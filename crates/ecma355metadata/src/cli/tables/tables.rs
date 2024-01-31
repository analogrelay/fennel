use crate::cli::{BlobHandle, FieldAttributes, GuidHandle, MethodAttributes, MethodImplAttributes, ParamAttributes, StringHandle, TypeAttributes};
use crate::{table_def, coded_index};

coded_index!(ResolutionScope, [
    Module, 
    ModuleRef, 
    AssemblyRef, 
    TypeRef,
]);

coded_index!(TypeDefOrRef, [
    TypeDef,
    TypeRef,
    TypeSpec,
]);

table_def!(Module, [
    generation: u16, 
    name: StringHandle, 
    mvid: GuidHandle, 
    enc_id: GuidHandle, 
    enc_base_id: GuidHandle,
]);

table_def!(TypeRef, [
    resolution_scope: (ResolutionScope),
    name: StringHandle,
    namespace: StringHandle,
]);

table_def!(TypeDef, [
    flags: TypeAttributes as u32,
    type_name: StringHandle,
    type_namespace: StringHandle,
    extends: (TypeDefOrRef),
    field_list: [Field],
    method_list: [MethodDef],
]);

table_def!(MethodDef, [
    rva: u32,
    impl_flags: MethodImplAttributes as u16,
    flags: MethodAttributes as u16,
    name: StringHandle,
    signature: BlobHandle,
    params: [Param],
]);

table_def!(Param, [
    flags: ParamAttributes as u16,
    sequence: u16,
    name: StringHandle,
]);

table_def!(Field, [
    flags: FieldAttributes as u16,
    name: StringHandle,
    signature: BlobHandle,
]);

table_def!(InterfaceImpl, [
    class: [TypeDef],
    interface: (TypeDefOrRef),
]);