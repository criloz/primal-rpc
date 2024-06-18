use crate::v1::ids::{EnumVariantId, ResourceId};
use crate::v1::parser::{is_not_extra_allow_docs, AttributeKind, ParseSpecContext, ParseSpecError, SyntaxError, IdKind};
use crate::v1::types::{EvolveTrack};
use indexmap::{IndexMap, IndexSet};
use std::num::{NonZeroU128, NonZeroU32};
use std::str::FromStr;
use tree_sitter::Node;
use uuid::Uuid;
use crate::v1::validators::{is_enum_variant_valid, VariantNameValidError};

pub fn id(
    callee: Node,
    args: &[Node],
    ctx: &mut ParseSpecContext,
) -> Result<Option<ResourceId>, SyntaxError> {
    return if args.len() == 1 {
        let val = args[0];
        if val.kind() == "str_lit" {
            let id = ctx.string(val);
            match uuid::Uuid::from_str(id) {
                Ok(id) => {
                    if let Some(v) = NonZeroU128::new(id.as_u128()) {
                        Ok(Some(ResourceId::EvolveId(ctx.package.add_evolve_id(
                            &val,
                            ctx.source_id,
                            EvolveTrack::Uuid(v),
                        ))))
                    } else {
                        ctx.error(val, ParseSpecError::CantBeZero).map(|_| None)

                    }
                }
                Err(_) => ctx
                    .error(
                        val,
                        ParseSpecError::TypeError {
                            expected: "uuid_lid".to_string(),
                            got: "str_lit".to_string(),
                        },
                    )
                    .map(|_| None),
            }
        } else if val.kind() == "num_lit" {
            let text = ctx.text(val);
            match u32::from_str(text) {
                Ok(U32_ID) => {
                    if let Some(U32_ID) = NonZeroU32::new(U32_ID) {
                        Ok(Some(ResourceId::EvolveId(ctx.package.add_evolve_id(
                            &val,
                            ctx.source_id,
                            EvolveTrack::Local(U32_ID),
                        ))))
                    } else {
                        ctx.error(val, ParseSpecError::CantBeZero).map(|_| None)
                    }
                }
                Err(_) => ctx
                    .error(
                        callee,
                        ParseSpecError::TypeError {
                            expected: "u32".to_string(),
                            got: "num_lit".to_string(),
                        },
                    )
                    .map(|_| None),
            }
        } else {
            ctx.error(
                callee,
                ParseSpecError::TypeError {
                    expected: "str_lit".to_string(),
                    got: val.kind().to_string(),
                },
            )
                .map(|_| None)
        }
    } else {
        ctx.error(
            callee,
            ParseSpecError::InvalidNumberOfArguments {
                expected: 1,
                got: args.len(),
            },
        )
            .map(|_| None)
    };
}

pub fn read_enum_variants(
    variants: &mut IndexSet<EnumVariantId>,
    node: &Node,
    ctx: &mut ParseSpecContext,
) -> Result<(), SyntaxError> {
    let elm: Vec<Node> = filter_not_extra_allow_docs(&node, ctx);
    let kws = read_kw(&elm, ctx)?;
    let mut unique_id_index: IndexMap<NonZeroU32, Node> = Default::default();
    let mut unique_name_index: IndexMap<String, Node> = Default::default();

    for kw in &kws {
        match kw {
            KWArgs::Value { value, .. } => {
                return ctx.error(*value, ParseSpecError::ExpectingKeyValue)
            }
            KWArgs::KV {
                key,
                value,
                docs,
                meta,
                ids,
            } => {
                let key_text = ctx.key(*key);
                let mut segment = key_text.split(":");
                if segment.clone().count() != 1 {
                    return ctx.error(*key, ParseSpecError::InvalidEnumVariantName { cause: VariantNameValidError::MultiplesSegments });
                }
                let variant_name = segment.next().unwrap();
                //validate key
                is_enum_variant_valid(variant_name)
                    .map_err(|e|
                    ctx.error(*key, ParseSpecError::InvalidEnumVariantName { cause: e }).unwrap_err()
                    )?;
                //names can not repeat
                let normalized = variant_name.to_lowercase();
                if let Some(other) = unique_name_index.get(&normalized) {
                    return ctx.conflict([*key, *other], ParseSpecError::ConflictVariantNameDefinition);
                } else {
                    unique_name_index.insert(normalized, *key);
                }


                //check that only one id is defined
                if ids.len() != 1 {
                    return ctx.error(*key, ParseSpecError::InvalidNumbersOfIds { expected: 1, got: ids.len() });
                }

                let id = ids[0];
                match evaluate(id, ctx)? {
                    Some(ResourceId::EvolveId(evolve_id)) => {
                        //check that id is local
                        match ctx.get(evolve_id).unwrap() {
                            EvolveTrack::Local(local_id) => {
                                //check that the id 
                                if let Some(other) = unique_id_index.get(local_id) {
                                    return ctx.conflict([id, *other], ParseSpecError::ConflictIdDefinition);
                                } else {
                                    unique_id_index.insert(*local_id, id);
                                }
                                //transform meta to tags
                                if !meta.is_empty() {
                                    todo!()
                                }

                                //transform docs to internal representation
                                if !docs.is_empty() {
                                    todo!()
                                }
                                //if value exist transformed to types
                                if let Some(v) = value {
                                    todo!()
                                }
                                //let e = EnumVariant{}
                                //todo!()
                            }
                            EvolveTrack::Uuid(_) => {
                                return ctx.error(id, ParseSpecError::InvalidId { expected: IdKind::Local, got: IdKind::Global });
                            }
                        }
                    }
                    Some(x) => {
                        return ctx.error(id, ParseSpecError::TypeError { expected: "id".to_string(), got: x.tty().to_string() });
                    }
                    None => {
                        return ctx.error(id, ParseSpecError::TypeError { expected: "id".to_string(), got: "null".to_string() });
                    }
                }


                //todo!()
            }
            KWArgs::None { ids, docs, meta } => {
                return if !ids.is_empty() {
                    ctx.error(
                        ids[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Id,
                        },
                    )
                } else if !docs.is_empty() {
                    ctx.error(
                        ids[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Doc,
                        },
                    )
                } else {
                    ctx.error(
                        meta[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Meta,
                        },
                    )
                }
            }
        }
    }
    Ok(())
}

pub fn enum_decl(
    callee: Node,
    args: &[Node],
    ctx: &mut ParseSpecContext,
) -> Result<Option<ResourceId>, SyntaxError> {
    let kws = read_kw(args, ctx)?;
    let mut variants: IndexSet<EnumVariantId> = Default::default();
    for kw in &kws {
        match kw {
            KWArgs::Value { value, .. } => {
                return ctx
                    .error(*value, ParseSpecError::ExpectingKeyValue)
                    .map(|_| None);
            }
            KWArgs::KV {
                key,
                value,
                docs,
                meta,
                ids,
            } => match ctx.key(*key) {
                "variants" => {
                    if let Some(v) = value {
                        if v.kind() == "vec_lit" {
                            read_enum_variants(&mut variants, v, ctx)?;
                        } else {
                            return ctx
                                .error(
                                    *v,
                                    ParseSpecError::TypeError {
                                        got: v.kind().to_string(),
                                        expected: "vec_lit".to_string(),
                                    },
                                )
                                .map(|_| None);
                        }
                    } else {
                        return ctx.error(*key, ParseSpecError::MissingValue).map(|_| None);
                    }
                }
                _ => {
                    return ctx
                        .error(
                            *key,
                            ParseSpecError::UnsupportedProperty {
                                got: ctx.key(*key).to_string(),
                                expected: IndexSet::from([
                                    "variants".to_string(),
                                    "tag".to_string(),
                                    "content".to_string(),
                                ]),
                            },
                        )
                        .map(|_| None);
                }
            },
            KWArgs::None { ids, meta, docs } => {
                return if !ids.is_empty() {
                    ctx.error(
                        ids[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Id,
                        },
                    )
                        .map(|_| None)
                } else if !docs.is_empty() {
                    ctx.error(
                        ids[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Doc,
                        },
                    )
                        .map(|_| None)
                } else {
                    ctx.error(
                        meta[0],
                        ParseSpecError::UnattachedAttribute {
                            kind: AttributeKind::Meta,
                        },
                    )
                        .map(|_| None)
                }
            }
        }
    }
    println!("{:?}", kws);
    todo!()
}

#[derive(Debug)]
pub enum KWArgs<'s> {
    Value {
        value: Node<'s>,
        docs: Vec<Node<'s>>,
        meta: Vec<Node<'s>>,
        ids: Vec<Node<'s>>,
    },
    KV {
        key: Node<'s>,
        value: Option<Node<'s>>,
        docs: Vec<Node<'s>>,
        meta: Vec<Node<'s>>,
        ids: Vec<Node<'s>>,
    },
    None {
        docs: Vec<Node<'s>>,
        meta: Vec<Node<'s>>,
        ids: Vec<Node<'s>>,
    },
}

pub fn read_meta<'s, 'ctx>(
    node: Node<'s>,
    meta_vec: &mut Vec<Node<'s>>,
    ctx: &'ctx ParseSpecContext<'ctx>,
) -> Result<Option<Node<'s>>, SyntaxError> {
    for n in 0..node.child_count() {
        let child = node.child(n).unwrap();
        if child.kind() == "meta_lit" {
            let meta_node = child.child(1).unwrap();
            meta_vec.push(meta_node);
        } else if child.kind() == "sym_name" {
            return Ok(Some(child));
        } else {
            return ctx
                .error(
                    child,
                    ParseSpecError::TypeError {
                        expected: "meta_lit|sym_name".to_string(),
                        got: child.kind().to_string(),
                    },
                )
                .map(|_| None);
        }
    }
    Ok(None)
}

pub fn read_kw<'s, 'ctx>(
    args: &'s [Node<'s>],
    ctx: &'ctx ParseSpecContext<'ctx>,
) -> Result<Vec<KWArgs<'s>>, SyntaxError> {
    let mut active_docs: Vec<Node<'s>> = Default::default();
    let mut active_key: Option<Node<'s>> = None;
    let mut active_meta: Vec<Node<'s>> = Default::default();
    let mut active_ids: Vec<Node<'s>> = Default::default();

    let mut all: Vec<KWArgs<'s>> = Vec::new();
    for n in args {
        if n.kind() == "comment" {
            if let Some(key) = active_key.take() {
                all.push(KWArgs::KV {
                    key,
                    value: None,
                    docs: std::mem::take(&mut active_docs),
                    meta: std::mem::take(&mut active_meta),
                    ids: std::mem::take(&mut active_ids),
                });
            } else {
                active_docs.push(*n);
            }
        } else if n.kind() == "kwd_lit" {
            if let Some(key) = active_key.take() {
                all.push(KWArgs::KV {
                    key,
                    value: None,
                    docs: std::mem::take(&mut active_docs),
                    meta: std::mem::take(&mut active_meta),
                    ids: std::mem::take(&mut active_ids),
                });
            } else {
                active_key = Some(*n);
            }
        } else if n.kind() == "sym_lit" {
            if let Some(value) = read_meta(*n, &mut active_meta, &ctx)? {
                if let Some(key) = active_key.take() {
                    all.push(KWArgs::KV {
                        key,
                        value: Some(value),
                        docs: std::mem::take(&mut active_docs),
                        meta: std::mem::take(&mut active_meta),
                        ids: std::mem::take(&mut active_ids),
                    });
                } else {
                    all.push(KWArgs::Value {
                        value,
                        docs: std::mem::take(&mut active_docs),
                        meta: std::mem::take(&mut active_meta),
                        ids: std::mem::take(&mut active_ids),
                    });
                }
            }
        } else if n.kind() == "list_lit" && n.child_count() > 0 {
            if let Some(elm) = first_not_extra_allow_docs(n, ctx) {
                if elm.kind() == "sym_name" || elm.kind() == "sym_lit" && ctx.text(elm) == "id" {
                    if let Some(key) = active_key.take() {
                        all.push(KWArgs::KV {
                            key,
                            value: None,
                            docs: std::mem::take(&mut active_docs),
                            meta: std::mem::take(&mut active_meta),
                            ids: std::mem::take(&mut active_ids),
                        });
                        active_ids.push(*n);
                    } else {
                        active_ids.push(*n);
                    }
                }
            } else if let Some(key) = active_key.take() {
                all.push(KWArgs::KV {
                    key,
                    value: Some(*n),
                    docs: std::mem::take(&mut active_docs),
                    meta: std::mem::take(&mut active_meta),
                    ids: std::mem::take(&mut active_ids),
                })
            } else {
                all.push(KWArgs::Value {
                    value: *n,
                    docs: std::mem::take(&mut active_docs),
                    meta: std::mem::take(&mut active_meta),
                    ids: std::mem::take(&mut active_ids),
                })
            }
        } else if let Some(key) = active_key.take() {
            all.push(KWArgs::KV {
                key,
                value: Some(*n),
                docs: std::mem::take(&mut active_docs),
                meta: std::mem::take(&mut active_meta),
                ids: std::mem::take(&mut active_ids),
            })
        } else {
            all.push(KWArgs::Value {
                value: *n,
                docs: std::mem::take(&mut active_docs),
                meta: std::mem::take(&mut active_meta),
                ids: std::mem::take(&mut active_ids),
            })
        }
    }
    if let Some(key) = active_key.take() {
        all.push(KWArgs::KV {
            key,
            value: None,
            docs: std::mem::take(&mut active_docs),
            meta: std::mem::take(&mut active_meta),
            ids: std::mem::take(&mut active_ids),
        })
    } else if active_docs.len() != 0 || active_meta.len() != 0 || active_ids.len() != 0 {
        all.push(KWArgs::None {
            docs: std::mem::take(&mut active_docs),
            meta: std::mem::take(&mut active_meta),
            ids: std::mem::take(&mut active_ids),
        })
    }
    Ok(all)
}

pub fn first_not_extra_allow_docs<'s, 'ctx>(
    node: &Node<'s>,
    ctx: &'ctx ParseSpecContext<'ctx>,
) -> Option<Node<'s>> {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        if is_not_extra_allow_docs(&ctx, &child) {
            return Some(child);
        }
    }
    None
}

pub fn filter_not_extra_allow_docs<'s, 'ctx>(
    node: &Node<'s>,
    ctx: &'ctx ParseSpecContext<'ctx>,
) -> Vec<Node<'s>> {
    let mut elm: Vec<Node> = Default::default();
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        if is_not_extra_allow_docs(&ctx, &child) {
            elm.push(child)
        }
    }
    elm
}

pub fn evaluate(node: Node, ctx: &mut ParseSpecContext) -> Result<Option<ResourceId>, SyntaxError> {
    let elm: Vec<Node> = filter_not_extra_allow_docs(&node, ctx);

    if elm.is_empty() {
        return Ok(None);
    }
    let callee = elm[0];
    let args = &elm[1..];
    let callee_name = ctx.text(callee).to_lowercase();
    let fn_ = match callee_name.as_str() {
        "id" => id,
        "enum" => enum_decl,
        _ => {
            return ctx
                .error(
                    callee,
                    ParseSpecError::FunctionNotFound {
                        name: callee_name.clone(),
                    },
                )
                .map(|_| None)
        }
    };
    fn_(callee, args, ctx)
}
