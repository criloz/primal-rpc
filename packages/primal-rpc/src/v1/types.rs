use crate::v1::ids::{CompilerId, DocId, EnumId, EnumVariantId, EvolveId, IdentId, KeyId, RefId, ResourceId, ShapeId, SourceId, StructId, TagId};
use indexmap::{IndexMap, IndexSet};
use std::fmt::{Debug, Formatter};
use std::num::{NonZeroU128, NonZeroU32};
use std::ops::{Range};
use std::path::PathBuf;
use tree_sitter::Node;

pub enum BetweenVersionsIdSegment {
    U32(NonZeroU32),
    Uuid(NonZeroU128),
    String(String),
}

pub struct Shape {
    ///name should be unique in the whole package
    pub name: Option<String>,
    pub compiler_id: Option<CompilerId>,
    pub cross_schema_id: Vec<BetweenVersionsIdSegment>,
    pub derived_trace: Vec<CompilerId>,
    /// indicate that the shape is ready for use for event generation
    pub is_ready: bool,
}

pub enum Enumeration {
    DiscriminantUnion {
        tag: StructId,
        variants: IndexSet<EnumVariantId>,
    },
    Elementary {
        id: EvolveId,
        variants: IndexSet<EnumVariantId>,
        default: Option<EnumVariantId>,
        docs: Vec<DocId>,
        tags: Vec<TagId>,
    },
}


pub enum Type {
    Ref(RefId),
    Struct(StructId),
    Enum(EnumId),
}

pub enum EnumVariantKind {
    Unit,
    Type(Type),
    Named(StructId),
}

pub struct EnumVariantNameAstId(KeyId);

pub enum EnumVariantName {
    Ast(EnumVariantNameAstId),
    Generated(String),
}

pub struct EnumVariant {
    pub local_id: EvolveId,
    pub compiler_id: Option<CompilerId>,
    pub cross_schema_id: Vec<BetweenVersionsIdSegment>,
    pub derived_trace: Option<Vec<CompilerId>>,
    pub name: EnumVariantName,
    pub tags: IndexSet<TagId>,
    pub docs: Vec<DocId>,
    pub kind: EnumVariantKind,
}


pub struct StructDef {
    docs: Vec<StructId>,
    fields: IndexSet<SourceId>,
}

pub enum Value {}

pub enum Default {
    Derive,
    Value(Value),
}

pub struct Field {
    key: KeyId,
    tty: Type,
    default: Option<Default>,
    tags: Vec<TagId>,
}

pub struct Key {
    segments: Vec<String>,
}

pub struct Ident {
    value: String,
}

pub struct Span {
    source_id: SourceId,
    span: Range<u32>,
}

impl Span {
    fn new(node: &Node, source_id: SourceId) -> Self {
        let start = u32::try_from(node.start_byte()).expect("file too big");
        let end = u32::try_from(node.end_byte()).expect("file too big");

        Span {
            source_id,
            span: start..end,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    File { path: PathBuf },
    Memory { identifier: String },
}

pub enum Struct {
    Plain(StructDef),
    Alternation(StructId, StructId),
    Sum(StructId, StructId),
}

pub struct Definition {
    ident: IdentId,
    interned_ident: InternedIdentId,
    define: ResourceId,
}

pub struct InternedTagId(Option<NonZeroU32>);

pub struct InternedIdentId(Option<NonZeroU32>);

pub struct InternedTagName {
    value: String,
}

pub enum EvolveTrack {
    Local(NonZeroU32),
    Uuid(NonZeroU128),
}

pub enum Tag {
    Word,
    Default(Default),
}

#[derive(Default)]
pub struct Package {
    ///set of task found
    pub(super) reverse_interned_tag: IndexMap<String, InternedTagId>,
    pub(super) interned_tags: IndexMap<InternedTagId, InternedTagName>,
    /// tags as they appear in the ast
    pub(super) lit_tags: Vec<Tag>,
    pub(super) lit_tags_span: Vec<Span>,
    pub(super) lit_tags_interned: Vec<InternedTagId>,
    pub(super) definitions: Vec<Definition>,
    pub(super) definitions_spans: Vec<Span>,
    pub(super) references: Vec<InternedIdentId>,
    pub(super) references_spans: Vec<Span>,
    pub(super) strings: Vec<String>,
    pub(super) strings_spans: Vec<Span>,
    pub(super) fields: Vec<Field>,
    pub(super) fields_span: Vec<Span>,
    pub(super) idents: Vec<Ident>,
    pub(super) idents_span: Vec<Span>,
    pub(super) enumerations: Vec<Enumeration>,
    pub(super) enumerations_span: Vec<Option<Span>>,
    pub(super) enumeration_shape: Vec<ShapeId>,
    pub(super) keys: Vec<Key>,
    pub(super) keys_span: Vec<Span>,
    pub(super) structs: Vec<Struct>,
    pub(super) structs_span: Vec<Span>,
    pub(super) evolution_ids: Vec<EvolveTrack>,
    pub(super) evolution_span: Vec<Span>,
    pub(super) sources: Vec<Source>,
    pub(super) shapes: Vec<Shape>,

    pub(super) enum_variant: Vec<EnumVariant>,
    pub(super) enum_variant_span: Vec<Option<Span>>,
}

impl Debug for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Package").finish()
    }
}

pub trait StoreAccessor<T> {
    fn get(self, package: &Package) -> Option<&T>;
}

impl Package {
    pub fn get<T, Key: StoreAccessor<T>>(&self, k: Key) -> Option<&T> {
        k.get(self)
    }
    pub fn add_evolve_id(&mut self, node: &Node, source_id: SourceId, et: EvolveTrack) -> EvolveId {
        let span = Span::new(node, source_id);
        self.evolution_ids.push(et);
        self.evolution_span.push(span);
        let pos = u32::try_from(self.evolution_ids.len()).expect("file too big");
        EvolveId(NonZeroU32::new(pos))
    }
    fn intern_tag(&mut self, name: &str) -> InternedTagId {
        todo!()
    }
    pub fn add_tag(&mut self, node: &Node, source_id: SourceId, t: Tag, name: &str) -> TagId {
        let span = Span::new(node, source_id);
        let interned = self.intern_tag(name);
        self.lit_tags.push(t);
        self.lit_tags_span.push(span);
        self.lit_tags_interned.push(interned);
        let pos = u32::try_from(self.lit_tags.len()).expect("file too big");
        TagId(NonZeroU32::new(pos))
    }
}

