use std::num::NonZeroU32;
use primal_rpc::v1::ids::SourceId;
use primal_rpc::v1::parser::{IdKind, ParseSpecError, SyntaxError};
use primal_rpc::v1::ParseSpecContext;
use primal_rpc::v1::types::Package;
use primal_rpc::v1::validators::VariantNameValidError;

#[test]
fn test_invalid_variant_name_pattern() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           :First-Town
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidEnumVariantName{cause: VariantNameValidError::Pattern}, ..})), "Expected Err, got {:?}", result);
}

#[test]
fn test_invalid_variant_name_case() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           :camelCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidEnumVariantName{cause: VariantNameValidError::PascalCase}, ..})), "Expected Err, got {:?}", result);
}


#[test]
fn test_invalid_variant_name_not_id() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           :PascalCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidNumbersOfIds{..}, ..})), "Expected Err, got {:?}", result);
}

#[test]
fn test_invalid_variant_name_multiples_ids() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           (id 2)
           :PascalCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidNumbersOfIds{..}, ..})), "Expected Err, got {:?}", result);
}


#[test]
fn test_invalid_variant_name_id_not_local() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
           :PascalCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidId{expected:IdKind::Local, got:IdKind::Global}, ..})), "Expected Err, got {:?}", result);
}


#[test]
fn test_invalid_variant_name_repeated_id() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           :PascalCase
           (id 1)
           :CamelCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::ConflictIdDefinition, ..})), "Expected Err, got {:?}", result);
}


#[test]
fn test_invalid_variant_name_repeated_name() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           :PascalCase
           (id 2)
           :PascalCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::ConflictVariantNameDefinition, ..})), "Expected Err, got {:?}", result);
}


#[test]
fn test_invalid_variant_name_variant_name_with_multiples_segments() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "56ecc9e7-1867-4a0e-850d-bf4393b8e2c0")
      (enum
        :variants [
           (id 1)
           :PascalCase:CamelCase
        ]
      )
     )
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    let result = ParseSpecContext::new(source_id, content, &mut package);
    assert!(matches!(result, Err(SyntaxError{value: ParseSpecError::InvalidEnumVariantName {cause:VariantNameValidError::MultiplesSegments}, ..})), "Expected Err, got {:?}", result);
}