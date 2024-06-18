use std::num::NonZeroU32;
use crate::v1::types::{Enumeration, EnumVariant, EvolveTrack, Package, Shape, StoreAccessor};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct SourceId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct EnumVariantId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct IdentId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct KeyId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RefId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FileId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct StringId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct TypeId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct StructId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct EnumId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct TagId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct EvolveId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DocId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ShapeId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct CompilerId(pub Option<NonZeroU32>);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum ResourceId {
    SourceId(SourceId),
    EnumVariantId(EnumVariantId),
    IdentId(IdentId),
    KeyId(KeyId),
    RefId(RefId),
    FileId(FileId),
    StringId(StringId),
    TypeId(TypeId),
    StructId(StructId),
    EnumId(EnumId),
    TagId(TagId),
    EvolveId(EvolveId),
}

impl ResourceId {
    pub(crate) fn tty(&self) -> &'static str {
        match self {
            ResourceId::SourceId(_) => "source",
            ResourceId::EnumVariantId(_) => "enum_variant",
            ResourceId::IdentId(_) => "ident",
            ResourceId::KeyId(_) => "key",
            ResourceId::RefId(_) => "ref",
            ResourceId::FileId(_) => "field",
            ResourceId::StringId(_) => "string_lit",
            ResourceId::TypeId(_) => "type",
            ResourceId::StructId(_) => "struct",
            ResourceId::EnumId(_) => "enum",
            ResourceId::TagId(_) => "tag",
            ResourceId::EvolveId(_) => "id",
        }
    }
}


impl StoreAccessor<EvolveTrack> for EvolveId {
    fn get(self, package: &Package) -> Option<&EvolveTrack> {
        self.0.map(|x| x.get() - 1).and_then(|pos| package.evolution_ids.get(pos as usize))
    }
}

impl StoreAccessor<Enumeration> for EnumId {
    fn get(self, package: &Package) -> Option<&Enumeration> {
        self.0.map(|x| x.get() - 1).and_then(|pos| package.enumerations.get(pos as usize))
    }
}

impl StoreAccessor<Shape> for ShapeId {
    fn get(self, package: &Package) -> Option<&Shape> {
        self.0.map(|x| x.get() - 1).and_then(|pos| package.shapes.get(pos as usize))
    }
}


impl StoreAccessor<ShapeId> for EnumId {
    fn get(self, package: &Package) -> Option<&ShapeId> {
        self.0.map(|x| x.get() - 1).and_then(|pos| package.enumeration_shape.get(pos as usize))
    }
}


impl StoreAccessor<EnumVariant> for EnumVariantId {
    fn get(self, package: &Package) -> Option<&EnumVariant> {
        self.0.map(|x| x.get() - 1).and_then(|pos| package.enum_variant.get(pos as usize))
    }
}