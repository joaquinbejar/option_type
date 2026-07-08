//! Integration tests for the optional `utoipa` OpenAPI schema generation.
//!
//! The entire file body is gated on the `utoipa` feature, so the default
//! (feature-less) build compiles and links this test target as an empty crate.

#![cfg(feature = "utoipa")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use option_type::prelude::*;
use utoipa::{OpenApi, PartialSchema};

/// Serialize a generated schema and assert it is a non-empty JSON object.
fn assert_schema_non_empty(
    schema: utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    label: &str,
) {
    let value = serde_json::to_value(&schema).expect("schema serializes to JSON");
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("{label} schema must be a JSON object, got: {value}"));
    assert!(
        !object.is_empty(),
        "{label} schema object must be non-empty"
    );
}

#[test]
fn test_utoipa_option_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(<OptionType as PartialSchema>::schema(), "OptionType");
}

#[test]
fn test_utoipa_option_basic_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(
        <OptionBasicType<'static> as PartialSchema>::schema(),
        "OptionBasicType",
    );
}

#[test]
fn test_utoipa_asian_averaging_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(
        <AsianAveragingType as PartialSchema>::schema(),
        "AsianAveragingType",
    );
}

#[test]
fn test_utoipa_barrier_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(<BarrierType as PartialSchema>::schema(), "BarrierType");
}

#[test]
fn test_utoipa_binary_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(<BinaryType as PartialSchema>::schema(), "BinaryType");
}

#[test]
fn test_utoipa_lookback_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(<LookbackType as PartialSchema>::schema(), "LookbackType");
}

#[test]
fn test_utoipa_rainbow_type_schema_generates_non_empty_object() {
    assert_schema_non_empty(<RainbowType as PartialSchema>::schema(), "RainbowType");
}

/// A minimal OpenAPI document referencing every schema-carrying enum, to
/// exercise the full `utoipa` registration pipeline end to end.
#[derive(OpenApi)]
#[openapi(components(schemas(
    OptionType,
    AsianAveragingType,
    BarrierType,
    BinaryType,
    LookbackType,
    RainbowType,
)))]
struct ApiDoc;

#[test]
fn test_utoipa_openapi_doc_registers_all_enum_schemas() {
    let doc = ApiDoc::openapi();
    let json = serde_json::to_value(&doc).expect("OpenAPI document serializes to JSON");
    let schemas = json
        .get("components")
        .and_then(|components| components.get("schemas"))
        .and_then(|schemas| schemas.as_object())
        .expect("OpenAPI document exposes components.schemas");

    for name in [
        "OptionType",
        "AsianAveragingType",
        "BarrierType",
        "BinaryType",
        "LookbackType",
        "RainbowType",
    ] {
        assert!(
            schemas.contains_key(name),
            "schema `{name}` must be registered in the OpenAPI document"
        );
    }
}
