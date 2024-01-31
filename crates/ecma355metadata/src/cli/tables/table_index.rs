// We want TableMask to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::mem;
use std::str::FromStr;
use crate::error::Error;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableIndex {
    Module = 0x00,
    TypeRef = 0x01,
    TypeDef = 0x02,
    FieldPtr = 0x03,
    Field = 0x04,
    MethodPtr = 0x05,
    MethodDef = 0x06,
    ParamPtr = 0x07,
    Param = 0x08,
    InterfaceImpl = 0x09,
    MemberRef = 0x0A,
    Constant = 0x0B,
    CustomAttribute = 0x0C,
    FieldMarshal = 0x0D,
    DeclSecurity = 0x0E,
    ClassLayout = 0x0F,
    FieldLayout = 0x10,
    StandAloneSig = 0x11,
    EventMap = 0x12,
    EventPtr = 0x13,
    Event = 0x14,
    PropertyMap = 0x15,
    PropertyPtr = 0x16,
    Property = 0x17,
    MethodSemantics = 0x18,
    MethodImpl = 0x19,
    ModuleRef = 0x1A,
    TypeSpec = 0x1B,
    ImplMap = 0x1C,
    FieldRva = 0x1D,
    EncLog = 0x1E,
    EncMap = 0x1F,
    Assembly = 0x20,
    AssemblyProcessor = 0x21,
    AssemblyOS = 0x22,
    AssemblyRef = 0x23,
    AssemblyRefProcessor = 0x24,
    AssemblyRefOS = 0x25,
    File = 0x26,
    ExportedType = 0x27,
    ManifestResource = 0x28,
    NestedClass = 0x29,
    GenericParam = 0x2A,
    MethodSpec = 0x2B,
    GenericParamConstraint = 0x2C,
    Document = 0x30,
    MethodDebugInformation = 0x31,
    LocalScope = 0x32,
    LocalVariable = 0x33,
    LocalConstant = 0x34,
    ImportScope = 0x35,
    StateMachineMethod = 0x36,
    CustomDebugInformation = 0x37,
}

impl From<u8> for TableIndex {
    fn from(val: u8) -> TableIndex {
        if val <= 0x2Cu8 || val >= 0x30u8 && val <= 0x37u8 {
            unsafe {
                mem::transmute(val)
            }
        } else {
            panic!("Invalid table index: 0x{:2X}", val)
        }
    }
}

impl FromStr for TableIndex {
    type Err = Error;

    fn from_str(s: &str) -> Result<TableIndex, Error> {
        Ok(match s {
            "Module" => TableIndex::Module,
            "TypeRef" => TableIndex::TypeRef,
            "TypeDef" => TableIndex::TypeDef,
            "FieldPtr" => TableIndex::FieldPtr,
            "Field" => TableIndex::Field,
            "MethodPtr" => TableIndex::MethodPtr,
            "MethodDef" => TableIndex::MethodDef,
            "ParamPtr" => TableIndex::ParamPtr,
            "Param" => TableIndex::Param,
            "InterfaceImpl" => TableIndex::InterfaceImpl,
            "MemberRef" => TableIndex::MemberRef,
            "Constant" => TableIndex::Constant,
            "CustomAttribute" => TableIndex::CustomAttribute,
            "FieldMarshal" => TableIndex::FieldMarshal,
            "DeclSecurity" => TableIndex::DeclSecurity,
            "ClassLayout" => TableIndex::ClassLayout,
            "FieldLayout" => TableIndex::FieldLayout,
            "StandAloneSig" => TableIndex::StandAloneSig,
            "EventMap" => TableIndex::EventMap,
            "EventPtr" => TableIndex::EventPtr,
            "Event" => TableIndex::Event,
            "PropertyMap" => TableIndex::PropertyMap,
            "PropertyPtr" => TableIndex::PropertyPtr,
            "Property" => TableIndex::Property,
            "MethodSemantics" => TableIndex::MethodSemantics,
            "MethodImpl" => TableIndex::MethodImpl,
            "ModuleRef" => TableIndex::ModuleRef,
            "TypeSpec" => TableIndex::TypeSpec,
            "ImplMap" => TableIndex::ImplMap,
            "FieldRva" => TableIndex::FieldRva,
            "EncLog" => TableIndex::EncLog,
            "EncMap" => TableIndex::EncMap,
            "Assembly" => TableIndex::Assembly,
            "AssemblyProcessor" => TableIndex::AssemblyProcessor,
            "AssemblyOS" => TableIndex::AssemblyOS,
            "AssemblyRef" => TableIndex::AssemblyRef,
            "AssemblyRefProcessor" => TableIndex::AssemblyRefProcessor,
            "AssemblyRefOS" => TableIndex::AssemblyRefOS,
            "File" => TableIndex::File,
            "ExportedType" => TableIndex::ExportedType,
            "ManifestResource" => TableIndex::ManifestResource,
            "NestedClass" => TableIndex::NestedClass,
            "GenericParam" => TableIndex::GenericParam,
            "MethodSpec" => TableIndex::MethodSpec,
            "GenericParamConstraint" => TableIndex::GenericParamConstraint,
            "Document" => TableIndex::Document,
            "MethodDebugInformation" => TableIndex::MethodDebugInformation,
            "LocalScope" => TableIndex::LocalScope,
            "LocalVariable" => TableIndex::LocalVariable,
            "LocalConstant" => TableIndex::LocalConstant,
            "ImportScope" => TableIndex::ImportScope,
            "StateMachineMethod" => TableIndex::StateMachineMethod,
            "CustomDebugInformation" => TableIndex::CustomDebugInformation,
            _ => return Err(Error::UnknownTableName)
        })
    }
}

impl_display_via_debug!(TableIndex);

impl TableIndex {
    pub const MAX: usize = 0x37;

    pub fn each() -> impl Iterator<Item = TableIndex> {
        TableIndexIter(Some(TableIndex::Module))
    }

    fn next(self) -> Option<TableIndex> {
        let val = self as u8;
        let next_val = if val < 0x2Cu8 {
            val + 1
        } else if val < 0x30u8 {
            0x30
        } else if val < 0x37u8 {
            val + 1
        } else {
            return None
        };

        Some(unsafe {
            mem::transmute_copy(&next_val)
        })
    }
}

pub struct TableIndexIter(Option<TableIndex>);

impl Iterator for TableIndexIter {
    type Item = TableIndex;

    fn next(&mut self) -> Option<TableIndex> {
        if let Some(x) = self.0 {
            self.0 = x.next();

            Some(x)
        } else {
            None
        }
    }
}

bitflags! {
    pub struct TableMask : u64 {
        const Module = 1 << TableIndex::Module as u64;
        const TypeRef = 1 << TableIndex::TypeRef as u64;
        const TypeDef = 1 << TableIndex::TypeDef as u64;
        const FieldPtr = 1 << TableIndex::FieldPtr as u64;
        const Field = 1 << TableIndex::Field as u64;
        const MethodPtr = 1 << TableIndex::MethodPtr as u64;
        const MethodDef = 1 << TableIndex::MethodDef as u64;
        const ParamPtr = 1 << TableIndex::ParamPtr as u64;
        const Param = 1 << TableIndex::Param as u64;
        const InterfaceImpl = 1 << TableIndex::InterfaceImpl as u64;
        const MemberRef = 1 << TableIndex::MemberRef as u64;
        const Constant = 1 << TableIndex::Constant as u64;
        const CustomAttribute = 1 << TableIndex::CustomAttribute as u64;
        const FieldMarshal = 1 << TableIndex::FieldMarshal as u64;
        const DeclSecurity = 1 << TableIndex::DeclSecurity as u64;
        const ClassLayout = 1 << TableIndex::ClassLayout as u64;
        const FieldLayout = 1 << TableIndex::FieldLayout as u64;
        const StandAloneSig = 1 << TableIndex::StandAloneSig as u64;
        const EventMap = 1 << TableIndex::EventMap as u64;
        const EventPtr = 1 << TableIndex::EventPtr as u64;
        const Event = 1 << TableIndex::Event as u64;
        const PropertyMap = 1 << TableIndex::PropertyMap as u64;
        const PropertyPtr = 1 << TableIndex::PropertyPtr as u64;
        const Property = 1 << TableIndex::Property as u64;
        const MethodSemantics = 1 << TableIndex::MethodSemantics as u64;
        const MethodImpl = 1 << TableIndex::MethodImpl as u64;
        const ModuleRef = 1 << TableIndex::ModuleRef as u64;
        const TypeSpec = 1 << TableIndex::TypeSpec as u64;
        const ImplMap = 1 << TableIndex::ImplMap as u64;
        const FieldRva = 1 << TableIndex::FieldRva as u64;
        const EncLog = 1 << TableIndex::EncLog as u64;
        const EncMap = 1 << TableIndex::EncMap as u64;
        const Assembly = 1 << TableIndex::Assembly as u64;
        const AssemblyProcessor = 1 << TableIndex::AssemblyProcessor as u64;
        const AssemblyOS = 1 << TableIndex::AssemblyOS as u64;
        const AssemblyRef = 1 << TableIndex::AssemblyRef as u64;
        const AssemblyRefProcessor = 1 << TableIndex::AssemblyRefProcessor as u64;
        const AssemblyRefOS = 1 << TableIndex::AssemblyRefOS as u64;
        const File = 1 << TableIndex::File as u64;
        const ExportedType = 1 << TableIndex::ExportedType as u64;
        const ManifestResource = 1 << TableIndex::ManifestResource as u64;
        const NestedClass = 1 << TableIndex::NestedClass as u64;
        const GenericParam = 1 << TableIndex::GenericParam as u64;
        const MethodSpec = 1 << TableIndex::MethodSpec as u64;
        const GenericParamConstraint = 1 << TableIndex::GenericParamConstraint as u64;
        const Document = 1 << TableIndex::Document as u64;
        const MethodDebugInformation = 1 << TableIndex::MethodDebugInformation as u64;
        const LocalScope = 1 << TableIndex::LocalScope as u64;
        const LocalVariable = 1 << TableIndex::LocalVariable as u64;
        const LocalConstant = 1 << TableIndex::LocalConstant as u64;
        const ImportScope = 1 << TableIndex::ImportScope as u64;
        const StateMachineMethod = 1 << TableIndex::StateMachineMethod as u64;
        const CustomDebugInformation = 1 << TableIndex::CustomDebugInformation as u64;

        // Coded Indexes
        const ResolutionScope =
            TableMask::Module.bits() | 
            TableMask::ModuleRef.bits() | 
            TableMask::AssemblyRef.bits() | 
            TableMask::TypeRef.bits();
    }
}

impl TableMask {
    pub fn has_table(&self, val: TableIndex) -> bool {
        self.bits() & (1 << val as u64) != 0
    }

    pub fn tables(&self) -> impl Iterator<Item = TableIndex> + '_ {
        // TODO: This ain't fast, but it works
        TableIndex::each().filter(move |x| self.has_table(*x))
    }
}

impl std::convert::From<TableIndex> for TableMask {
    fn from(value: TableIndex) -> TableMask {
        TableMask::from_bits_truncate(1 << value as u64)
    }
}

impl std::fmt::Display for TableMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.iter_names().fold(String::new(), |mut acc, (name, _)| {
            if !acc.is_empty() {
                acc.push_str(" | ");
            }
            acc.push_str(name);
            acc
        }).fmt(f)
    }
}