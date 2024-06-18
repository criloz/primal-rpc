use std::ops::{Coroutine, CoroutineState};
use std::pin::pin;
use uuid::Uuid;
use crate::events::{DfsPostorderEvent, EventGenerationContext};
use crate::events::tags::EventTag;
use crate::v1::ids::{CompilerId, EnumId, EnumVariantId, EvolveId, ShapeId};
use crate::v1::types::{BetweenVersionsIdSegment, Enumeration, EnumVariant, EvolveTrack, Package};
macro_rules! delegate {
    ($gen:expr) => {
        loop {
            match std::pin::Pin::new(&mut $gen).resume(()) {
                std::ops::CoroutineState::Yielded(evt) => { yield evt; }
                std::ops::CoroutineState::Complete(_) => { break; }
            }
        }
    };
}
impl EnumId {
    fn to_events<'pkg, 'ctx: 'pkg>(self, eg_ctx: &'ctx mut EventGenerationContext, package: &'pkg Package) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
        #[coroutine]
        move || {
            /*
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
             */
            let mut child_count = 0;
            {
                //emit shape events
                let enum_shape: &ShapeId = package.get(self).unwrap();
                let mut gen = enum_shape.to_events(eg_ctx, package);
                loop {
                    match pin!(&mut gen).resume(()) {
                        CoroutineState::Yielded(evt) => { yield evt }
                        CoroutineState::Complete(_) => { break }
                    }
                }
                child_count += 1;
            }
            let enum_def: &Enumeration = package.get(self).unwrap();


            match enum_def {
                Enumeration::DiscriminantUnion { .. } => { todo!() }
                Enumeration::Elementary { variants, id, docs, tags, default } => {
                    {
                        let mut gen = id.to_events(eg_ctx, package);
                        loop {
                            match pin!(&mut gen).resume(()) {
                                CoroutineState::Yielded(evt) => { yield evt }
                                CoroutineState::Complete(_) => { break }
                            }
                        }
                        child_count += 1;
                    }

                    {
                        //emit variant field
                        eg_ctx.strings.push("variants".to_string());
                        yield DfsPostorderEvent::Leaf {
                            tag: EventTag::String,
                        };
                        yield DfsPostorderEvent::Branch {
                            tag: EventTag::FieldKey,
                            child_count: 1,
                        };
                        for variant in variants {
                            todo!()
                        }
                        yield DfsPostorderEvent::Branch {
                            tag: EventTag::FieldValue,
                            child_count: variants.len(),
                        };
                        yield DfsPostorderEvent::Branch {
                            tag: EventTag::Field,
                            child_count: 2,
                        };
                        child_count += 1;
                    }
                    { //emit example
                    }
                    { //emit documentation
                    }
                    { //emit tags
                    }
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::ElementaryEnum,
                        child_count,
                    };
                }
            }
        }
    }
}

impl ShapeId {
    fn to_events<'pkg, 'ctx: 'pkg>(self, eg_ctx: &'ctx mut EventGenerationContext, package: &'pkg Package) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
        #[coroutine]
        move || {
            let mut child_count = 0;
            let shape_def = package.get(self).unwrap();
            if shape_def.is_ready {
                /*
                    CompilerIdentifier
                    EvolutionaryIdentifier
                    ShapeName
                    (? ShapeDerivedTrace)
                 */
                let compiler_id = shape_def.compiler_id.expect("[internal_error]: Shape is missing compiler id");
                eg_ctx.u32.push(compiler_id.0.map(|x| x.get()).expect("[internal_error]: Compiler id is missing"));
                yield DfsPostorderEvent::Leaf {
                    tag: EventTag::U32,
                };
                yield DfsPostorderEvent::Branch {
                    tag: EventTag::CompilerIdentifier,
                    child_count: 1,
                };
                child_count += 1;
                if !shape_def.cross_schema_id.is_empty() {
                    delegate!(cross_schema_id_gen(&shape_def.cross_schema_id, eg_ctx));
                    child_count += 1;
                } else {
                    unreachable!("[internal error] Shape is missing evolutionary id");
                }
                if let Some(name) = &shape_def.name {
                    eg_ctx.strings.push(name.clone());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::String,
                    };
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::ShapeName,
                        child_count: 1,
                    };
                    child_count += 1;
                } else {
                    unreachable!("[internal error] Shape is missing name");
                }
                if !shape_def.derived_trace.is_empty() {
                    for id in shape_def.derived_trace.iter() {
                        eg_ctx.u32.push(id.0.map(|x| x.get()).expect("[internal_error/shape_def.derived_trace]: Compiler id is missing"));
                        yield DfsPostorderEvent::Leaf {
                            tag: EventTag::U32,
                        };
                        yield DfsPostorderEvent::Branch {
                            tag: EventTag::CompilerIdentifier,
                            child_count: 1,
                        };
                    }
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::DerivedTrace,
                        child_count: shape_def.derived_trace.len(),
                    };
                    child_count += 1;
                }
                yield DfsPostorderEvent::Branch {
                    tag: EventTag::Shape,
                    child_count,
                };
            } else {
                unreachable!("[internal_error]: Shape is not ready for generation")
            }
        }
    }
}


impl EnumVariantId {
    fn to_events<'pkg, 'ctx: 'pkg>(self, eg_ctx: &'ctx mut EventGenerationContext, package: &'pkg Package) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
        #[coroutine]
        move || {
            /*
                 (EnumVariantUnit
                    LocalIdentifier
                    CompilerIdentifier
                    EvolutionaryIdentifier
                    EnumVariantName
                    (? Documentation)
                    (? Tags)
                    (? DerivedTrace)
                 )
             */
            let mut child_count = 0;
            let enum_variant: &EnumVariant = package.get(self).unwrap();
            {
                //emit shape id
                delegate!(enum_variant.local_id.to_events(eg_ctx, package));
                child_count += 1;
            }
            if let Some(compiler_id) = enum_variant.compiler_id {
                delegate!(compiler_id.to_events(eg_ctx, package));
                child_count += 1;
            } else {
                unreachable!("[internal error]: EnumVariant is missing compiler id")
            }
            if !enum_variant.cross_schema_id.is_empty() {
                delegate!(cross_schema_id_gen(&enum_variant.cross_schema_id, eg_ctx));
                child_count += 1;
            } else {
                unreachable!("[internal error]: EnumVariant is missing evolutionary id");
            }
        }
    }
}

impl CompilerId {
    fn to_events<'pkg, 'ctx: 'pkg>(self, eg_ctx: &'ctx mut EventGenerationContext, package: &'pkg Package) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
        #[coroutine]
        move || {
            eg_ctx.u32.push(self.0.unwrap().get());
            yield DfsPostorderEvent::Leaf {
                tag: EventTag::U32,
            };
            yield DfsPostorderEvent::Branch {
                tag: EventTag::CompilerIdentifier,
                child_count: 1,
            };
        }
    }
}

impl EvolveId {
    fn to_events<'pkg, 'ctx: 'pkg>(self, eg_ctx: &'ctx mut EventGenerationContext, package: &'pkg Package) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
        #[coroutine]
        move || {
            /*
                 (| LocalIdentifier GlobalIdentifier)
             */
            let et: &EvolveTrack = package.get(self).unwrap();
            match et {
                EvolveTrack::Local(v) => {
                    eg_ctx.u32.push(v.get());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::U32,
                    };
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::LocalIdentifier,
                        child_count: 1,
                    };
                }
                EvolveTrack::Uuid(v) => {
                    let id = Uuid::from_u128(v.get());
                    eg_ctx.strings.push(id.to_string());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::String,
                    };
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::Uuid,
                        child_count: 1,
                    };
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::GlobalIdentifier,
                        child_count: 1,
                    };
                }
            }
        }
    }
}

fn cross_schema_id_gen<'pkg, 'ctx: 'pkg>(id: &'ctx [BetweenVersionsIdSegment], eg_ctx: &'ctx mut EventGenerationContext) -> impl Coroutine<(), Yield=DfsPostorderEvent<EventTag>, Return=()> + 'pkg {
    #[coroutine]
    move || {
        for id in id {
            match id {
                BetweenVersionsIdSegment::U32(v) => {
                    eg_ctx.u32.push(v.get());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::U32,
                    };
                }
                BetweenVersionsIdSegment::Uuid(id) => {
                    let id = Uuid::from_u128(id.get());
                    eg_ctx.strings.push(id.to_string());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::String,
                    };
                    yield DfsPostorderEvent::Branch {
                        tag: EventTag::Uuid,
                        child_count: 1,
                    };
                }
                BetweenVersionsIdSegment::String(v) => {
                    eg_ctx.strings.push(v.clone());
                    yield DfsPostorderEvent::Leaf {
                        tag: EventTag::String,
                    };
                }
            }
        }
        yield DfsPostorderEvent::Branch {
            tag: EventTag::EvolutionaryIdentifier,
            child_count: id.len(),
        };
    }
}