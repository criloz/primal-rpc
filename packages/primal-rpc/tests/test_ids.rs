use primal_rpc::v1::ids::SourceId;
use primal_rpc::v1::types::Package;
use primal_rpc::v1::ParseSpecContext;
use std::num::NonZeroU32;
use primal_rpc::v1::parser::{ParseSpecError, SyntaxError};

#[test]
fn test_invalid_uuid() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "bf43-9b561149883a")
      (enum
        :variants [
           :First
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(_)), "Expected Err, got {:?}", result);
}
#[test]
fn test_invalid_uuid_cant_be_zero() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "00000000-0000-0000-0000-000000000000")
      (enum
        :variants [
           :First
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::CantBeZero, ..})), "Expected Err, got {:?}", result);
}
#[test]
fn test_invalid_local_id() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id 5000000000)
      (enum
        :variants [
           :First
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(_)), "Expected Err, got {:?}", result);
}

#[test]
fn test_invalid_local_id_zero() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id 0)
      (enum
        :variants [
           :First
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(_)), "Expected Err, got {:?}", result);
}

#[test]
fn test_valid_local_id() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id 5)
      (enum
        :variants [
           :First
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Ok(_)), "Expected Ok, got {:?}", result);
}
