use crate::v1::ids::{ ResourceId, SourceId};
use crate::v1::procedures::evaluate;
use crate::v1::types::{Package,  StoreAccessor};
use indexmap::IndexSet;
use tree_sitter::{Node, Parser, Point, Tree};
use crate::v1::validators::VariantNameValidError;

pub fn make_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_clojure::language())
        .expect("Error loading Rust grammar");
    parser
}

pub fn parse_source(source: &str) -> Tree {
    let mut parser = make_parser();
    parser.parse(source, None).unwrap()
}

#[derive(Debug)]
pub struct ParseSpecContext<'s> {
    pub content: &'s str,
    version_checked: bool,
    pub package: &'s mut Package,
    pub source_id: SourceId,
}

impl<'s> ParseSpecContext<'s> {
    pub fn get<T, K: StoreAccessor<T>>(&'s self, key: K) -> Option<&T> {
        self.package.get(key)
    }
    pub fn new(
        source_id: SourceId,
        content: &'s str,
        package: &'s mut Package,
    ) -> Result<Self, SyntaxError> {
        let tree = parse_source(content);
        let mut ctx = ParseSpecContext {
            source_id,
            content,
            package,
            version_checked: false,
        };

        let result = visit_source(tree.root_node(), &mut ctx)?;
        Ok(ctx)
    }
}

pub fn is_not_extra(node: &Node) -> bool {
    if node.is_extra() {
        return false;
    }
    let extras = IndexSet::from(["(", ")", "[", "]"]);
    !extras.contains(node.kind())
}

pub fn is_not_extra_allow_docs(ctx: &ParseSpecContext, node: &Node) -> bool {
    if node.is_extra() {
        if node.kind() == "comment" && ctx.text(*node).starts_with(";;") {
            return true;
        }
        return false;
    }
    let extras = IndexSet::from(["(", ")", "[", "]"]);
    !extras.contains(node.kind())
}

#[derive(Debug)]
pub enum AttributeKind {
    Doc,
    Meta,
    Id,
}

#[derive(Debug)]
pub enum IdKind {
    Local,
    Global,
}

#[derive(Debug)]
pub enum ParseSpecError {
    UndefinedSymbol,
    InvalidEnumVariantName { cause: VariantNameValidError },
    InvalidId { expected: IdKind, got: IdKind },
    /// conflict between two or more id statements
    ConflictIdDefinition,
    ///two variant in the same enum share the same name
    ConflictVariantNameDefinition,
    InvalidNumbersOfIds { expected: usize, got: usize },
    InvalidNumberOfArguments {
        expected: usize,
        got: usize,
    },
    CantBeZero,
    TypeError {
        expected: String,
        got: String,
    },
    ExpectingKeyValue,
    UnattachedAttribute {
        kind: AttributeKind,
    },

    UnsupportedProperty {
        expected: IndexSet<String>,
        got: String,
    },
    FunctionNotFound {
        name: String,
    },
    ExpectingNode,
    ExpectingResource,
    MissingValue,

    ExpectedOperandGotOperator,
    IDLMissingVersion,
    UnsupportedIDLVersion {
        got: String,
        expecting: Vec<String>,
    },
    Custom(String),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub locations: Vec<Point>,
    pub value: ParseSpecError,
    pub source_id: SourceId,
}

impl<'s> ParseSpecContext<'s> {
    pub fn error(&self, node: Node, error: ParseSpecError) -> Result<(), SyntaxError> {
        Err(SyntaxError {
            locations: vec![node.start_position()],
            value: error,
            source_id: self.source_id,
        })
    }
    pub fn conflict<'a, I: IntoIterator<Item=Node<'a>>>(&self, nodes: I, error: ParseSpecError) -> Result<(), SyntaxError> {
        Err(SyntaxError {
            locations: nodes.into_iter().map(|x| x.start_position()).collect(),
            value: error,
            source_id: self.source_id,
        })
    }
    pub fn text(&self, node: Node) -> &'s str {
        node.utf8_text(self.content.as_bytes()).unwrap()
    }
    pub fn string(&self, node: Node) -> &'s str {
        let base = self.text(node);
        &base[1..(base.len() - 1)]
    }
    pub fn key(&self, node: Node) -> &'s str {
        let base = self.text(node);
        &base[1..base.len()]
    }
}

pub fn visit_source(node: Node, ctx: &mut ParseSpecContext) -> Result<(), SyntaxError> {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        if child.kind() == "list_lit" {
            visit_top_list(child, ctx)?;
        }
    }
    Ok(())
}

pub fn visit_top_version(
    caller: Node,
    arg: &[Node],
    ctx: &mut ParseSpecContext,
) -> Result<(), SyntaxError> {
    if arg.len() != 1 {
        return ctx.error(
            caller,
            ParseSpecError::InvalidNumberOfArguments {
                expected: 1,
                got: arg.len(),
            },
        );
    }
    let version_number = arg[0];
    return if version_number.kind() == "str_lit" {
        let version_value = ctx.string(version_number);
        if version_value == "1" {
            ctx.version_checked = true;
        } else {
            return ctx.error(
                caller,
                ParseSpecError::UnsupportedIDLVersion {
                    got: version_value.to_string(),
                    expecting: vec!["1".to_string()],
                },
            );
        }
        Ok(())
    } else {
        ctx.error(
            caller,
            ParseSpecError::TypeError {
                expected: "str_lit".to_string(),
                got: version_number.kind().to_string(),
            },
        )
    };
}

pub fn visit_top_list(node: Node, ctx: &mut ParseSpecContext) -> Result<(), SyntaxError> {
    let mut elements: Vec<Node> = Default::default();

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        if is_not_extra(&child) {
            elements.push(child)
        }
    }
    if elements.len() >= 2 {
        let caller = elements[0];
        if caller.kind() == "sym_lit" {
            let name = ctx.text(caller.child(0).unwrap());
            match name {
                "version" => {
                    return visit_top_version(caller, &elements[1..], ctx);
                }
                "def" => {
                    return if ctx.version_checked {
                        visit_def(caller, &elements[1..], ctx)
                    } else {
                        ctx.error(caller, ParseSpecError::IDLMissingVersion)
                    }
                }
                _ => {
                    ctx.error(caller, ParseSpecError::UndefinedSymbol)?;
                }
            }
        } else {
            ctx.error(
                caller,
                ParseSpecError::TypeError {
                    got: caller.kind().to_string(),
                    expected: "sym_lit".to_string(),
                },
            )?;
        }
    }
    Ok(())
}

pub fn visit_def(
    caller: Node,
    arg: &[Node],
    ctx: &mut ParseSpecContext,
) -> Result<(), SyntaxError> {
    if arg.len() != 3 {
        return ctx.error(
            caller,
            ParseSpecError::InvalidNumberOfArguments {
                expected: 3,
                got: arg.len(),
            },
        );
    }
    let name = arg[0];
    if name.kind() != "sym_lit" {
        return ctx.error(
            name,
            ParseSpecError::TypeError {
                got: name.kind().to_string(),
                expected: "sym_lit".to_string(),
            },
        );
    }
    let first = arg[1];
    if first.kind() != "list_lit" {
        return ctx.error(
            name,
            ParseSpecError::TypeError {
                got: name.kind().to_string(),
                expected: "list_lit".to_string(),
            },
        );
    }
    if let Some(global_id) = evaluate(first, ctx)? {
        let global_id = if let ResourceId::EvolveId(id) = global_id {
            id
        } else {
            return ctx.error(
                name,
                ParseSpecError::TypeError {
                    got: global_id.tty().to_string(),
                    expected: "id".to_string(),
                },
            );
        };
        let second = arg[2];
        if second.kind() != "list_lit" {
            return ctx.error(
                name,
                ParseSpecError::TypeError {
                    got: name.kind().to_string(),
                    expected: "list_lit".to_string(),
                },
            );
        }
        if let Some(resource) = evaluate(second, ctx)? {
            if let ResourceId::EvolveId(_) = resource {
                return ctx.error(
                    name,
                    ParseSpecError::TypeError {
                        got: "id".to_string(),
                        expected: "resource".to_string(),
                    },
                );
            }
            todo!()
        } else {
            ctx.error(first, ParseSpecError::ExpectingResource)
        }
    } else {
        ctx.error(first, ParseSpecError::ExpectingResource)
    }
}
