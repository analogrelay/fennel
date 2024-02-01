use crate::cli::{AssemblyFlags, AssemblyHashAlgorithm, BlobHandle, FieldAttributes, GuidHandle, MethodAttributes, MethodImplAttributes, ParamAttributes, StringHandle, TypeAttributes};
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

coded_index!(MemberRefParent, [
    TypeDef,
    TypeRef,
    ModuleRef,
    MethodDef,
    TypeSpec,
]);

coded_index!(HasConstant, [
    Field,
    Param,
    Property,
]);

coded_index!(HasCustomAttribute, [
    MethodDef,
    Field,
    TypeRef,
    TypeDef,
    Param,
    InterfaceImpl,
    MemberRef,
    Module,
    NonExistent, // Permission table, which is not used in the ECMA-335 spec
    Property,
    Event,
    StandAloneSig,
    ModuleRef,
    TypeSpec,
    Assembly,
    AssemblyRef,
    File,
    ExportedType,
    ManifestResource,
    GenericParam,
    GenericParamConstraint,
    MethodSpec,
]);

coded_index!(CustomAttributeType, [
    NonExistent,
    NonExistent,
    MethodDef,
    MemberRef,
    NonExistent,
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

table_def!(MemberRef, [
    class: (MemberRefParent),
    name: StringHandle,
    signature: BlobHandle,
]);

table_def!(Constant, [
    typ: u8,
    reserved: u8,
    parent: (HasConstant),
    value: BlobHandle,
]);

table_def!(CustomAttribute, [
    parent: (HasCustomAttribute),
    typ: (CustomAttributeType),
    value: BlobHandle,
]);

table_def!(Assembly, [
    hash_alg_id: AssemblyHashAlgorithm as u32,
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags as u32,
    public_key: BlobHandle,
    name: StringHandle,
    culture: StringHandle,
]);

table_def!(AssemblyRef, [
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags as u32,
    public_key_or_token: BlobHandle,
    name: StringHandle,
    culture: StringHandle,
    hash_value: BlobHandle,
]);