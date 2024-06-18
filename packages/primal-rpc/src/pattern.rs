use indexmap::IndexMap;
use tree_sitter::Node;
use crate::v1::parser::{parse_source, SyntaxError, visit_source};
use crate::v1::types::{Package, StoreAccessor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternTag<T> {
    /// a tag for a node
    Tag(T),
    /// Represents a node that can be repeated
    Many,
    /// Represents a node that can be repeated, and has at least one child
    Plus,
    /// Choice between two patterns
    Alternation,
    /// the Pattern that accepts any node
    Top,
    /// the Pattern that accepts no node
    Bottom,
    /// Optional node
    Maybe,

}


/// a tree to describe tree grammars
pub struct Pattern<T, V> {
    /// children count of the node
    pub child_count: Vec<u16>,
    /// the tag of the node
    pub tag: Vec<PatternTag<T>>,
    /// if true the children of the node are ordered
    pub ordered_children: Vec<bool>,
    /// pattern for terminal values
    pub value_pattern: Vec<Option<V>>,
    pub doc: Vec<String>,
}


#[derive(Debug)]
pub struct PatternsParseCtx<'s, T> {
    pub content: &'s str,
    pub tags: IndexMap<String, T>,
}

impl<'s, T> PatternsParseCtx<'s, T>
where
    T: strum::IntoEnumIterator + std::string::ToString,
{
    pub fn new(
        content: &'s str,
    ) -> Result<Self, SyntaxError> {
        let tree = parse_source(content);
        let mut tags: IndexMap<String, T> = Default::default();
        for x in T::iter() {
            tags.insert(x.to_string().to_lowercase(), x);
        }
        let mut ctx = PatternsParseCtx {
            content,
            tags,
        };

        let result = pattern_visit_source(tree.root_node(), &mut ctx)?;
        Ok(ctx)
    }
}

pub fn pattern_visit_source<T>(node: Node, ctx: &mut PatternsParseCtx<T>) -> Result<(), SyntaxError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;
    use crate::events::tags::EventTag;

    #[test]
    fn test_parse() {
        let source = r#"
        (pattern
            (tag "a")
            (child_count 1)
            (ordered_children true)
            (value_pattern "a")
            (doc "a")
        )
        "#;
        let mut ctx = PatternsParseCtx::<EventTag>::new(source);
       
    }
}