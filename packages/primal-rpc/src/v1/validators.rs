use regex::Regex;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub enum VariantNameValidError {
    PascalCase,
    Pattern,
    MultiplesSegments
}

pub fn is_enum_variant_valid(name: &str) -> Result<(), VariantNameValidError> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    if !re.is_match(name) {
        return Err(VariantNameValidError::Pattern);
    }
    if !name.is_case(Case::Pascal) {
        return Err(VariantNameValidError::PascalCase);
    }
    Ok(())
}
