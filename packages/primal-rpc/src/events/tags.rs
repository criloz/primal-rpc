use strum::{EnumCount, EnumIter};
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, EnumCount, EnumIter, strum::Display)]
pub enum EventTag {
    /// an elementary enum only contains unit variants 
    /// tag and content properties are not defined
    ElementaryEnum,
    ///a Key Value pair, used as field or property
    Field,
    /// the key of a field
    FieldKey,
    /// the value of a field
    FieldValue,
    /// a string value
    String,
    /// a global unique identifier value in uuid format
    GlobalIdentifier,
    /// an u32 as  local identifier, 
    LocalIdentifier,
    /// an identifier that will be stable while transpiling the schema  
    CompilerIdentifier,
    /// an identifier that will be stable within evolving the schema
    EvolutionaryIdentifier,
    /// a enum variant name
    EnumVariantName,
    /// an enum variant without fields
    EnumVariantUnit,
    /// list of documentation strings
    Documentation,
    /// list of tags
    Tags,
    /// a u32 value
    U32,
    /// a uuid value
    Uuid,
    /// unique name of the shape within a package
    ShapeName,
    /// if the entity was derived from another,
    /// this contains the base model and the list of rules applied for the derivation
    /// using compiler id as reference
    DerivedTrace,
    Shape,
}

pub enum EventPatternValue {
    String(String),
    Identifier(String),
    VariantName(String),
}

impl EventTag {
    pub fn is_terminal(&self) -> bool {
        match self {
            EventTag::ElementaryEnum => { false }
            EventTag::Field => { false }
            EventTag::FieldKey => { false }
            EventTag::FieldValue => { false }
            EventTag::String => { true }
            EventTag::EnumVariantName => { true }
            EventTag::EnumVariantUnit => { false }
            EventTag::Documentation => { false }
            EventTag::Tags => { false }
            EventTag::GlobalIdentifier => { false }
            EventTag::LocalIdentifier => { false }
            EventTag::U32 => { true }
            EventTag::Uuid => { false }
            EventTag::CompilerIdentifier => { false }
            EventTag::EvolutionaryIdentifier => { false }
            EventTag::ShapeName => { false }
            EventTag::DerivedTrace => { false }
            EventTag::Shape => { false }
        }
    }

    //return pattern for non-terminals
    pub fn pattern(&self) -> Option<String> {
        match self {
            EventTag::ElementaryEnum => {
                Some(r##"
                 (ElementaryEnum
                    (| LocalIdentifier GlobalIdentifier)
                    Shape
                    ;; list of variants
                    (Field
                        (FieldKey (String "variants"))
                        (FieldValue (+ EnumVariantUnit))
                    )
                    ;; if a default variant is defined
                    (? (Field
                        (FieldKey (String "default"))
                        (FieldValue CompilerIdentifier)
                    ))
                    (? Documentation)
                    (? Tags)
                 )
                 "##.to_string())
            }
            EventTag::Field => {
                Some(r##"
                    (Field
                        FieldKey
                        FieldValu
                    )"##.to_string())
            }
            EventTag::FieldKey => {
                Some(r##"(FieldKey _)"##.to_string())
            }
            EventTag::FieldValue => {
                Some(r##"(FieldValue _)"##.to_string())
            }
            EventTag::String => { None }
            EventTag::EnumVariantName => { None }
            EventTag::EnumVariantUnit => {
                Some(r##"
                 (EnumVariantUnit
                    LocalIdentifier
                    CompilerIdentifier
                    EvolutionaryIdentifier
                    EnumVariantName
                    (? Documentation)
                    (? Tags)
                    (? DerivedTrace)
                 )
                 "##.to_string())
            }
            EventTag::Documentation => {
                Some(r##"(Documentation (+ String))"##.to_string())
            }
            EventTag::Tags => {
                Some(r##"(Tags (+ String))"##.to_string())
            }
            EventTag::GlobalIdentifier => {
                Some(r##"(GlobalIdentifier Uuid)"##.to_string())
            }
            EventTag::LocalIdentifier => {
                Some(r##"(LocalIdentifier U32)"##.to_string())
            }
            EventTag::U32 => { None }
            EventTag::Uuid => {
                Some(r##"(Uuid String)"##.to_string())
            }
            EventTag::CompilerIdentifier => {
                Some(r##"(CompilerIdentifier u32)"##.to_string())
            }
            EventTag::EvolutionaryIdentifier => {
                Some(r##"(EvolutionaryIdentifier (+ (| String Uuid U32)))"##.to_string())
            }
            EventTag::ShapeName => {
                Some(r##"(ShapeName String)"##.to_string())
            }
            EventTag::DerivedTrace => {
                Some(r##"(DerivedTrace (+ CompilerIdentifier))"##.to_string())
            }
            EventTag::Shape => {
                Some(r##"
                (Shape 
                    CompilerIdentifier
                    EvolutionaryIdentifier
                    ShapeName
                    (? DerivedTrace)
                )"##.to_string())
            }
        }
    }
}



