//! Regression tests for predefined (extension) rules on a proto3 string:
//!
//! - `IGNORE_IF_ZERO_VALUE` skips the predefined and `cel` rules of an empty
//!   field, as it skips the standard ones;
//! - a CEL `matches()` compiles through the runtime's `regex` re-export, so the
//!   crate holding the generated code needs no `regex` dependency of its own;
//! - the rule path names the extension by its own fully qualified name, not by
//!   the conformance suite's package.

use protoc_gen_protovalidate_buffa::emit::render;
use protoc_gen_protovalidate_buffa::scan::{
    CelRule, FieldKind, FieldValidator, Ignore, MessageValidators, PredefinedCel, RuleConst,
    StandardRules,
};

fn language_tag_rule() -> PredefinedCel {
    PredefinedCel {
        id: "string.language_tag".to_string(),
        message: "must be a BCP-47 language tag".to_string(),
        expression: "!rule || this.matches('^[a-z]{2,3}(-[A-Za-z0-9]{2,8})*$')".to_string(),
        rule_const: Some(RuleConst::Bool(true)),
        ext_number: 51_101,
        ext_name: "acme.v1.language_tag".to_string(),
        ext_field_type: "Bool".to_string(),
        family_override: None,
    }
}

fn render_field(ignore: Ignore) -> String {
    let field = FieldValidator {
        field_number: 1,
        field_name: "language_code".to_string(),
        rust_name: "language_code".to_string(),
        field_type: FieldKind::String,
        required: false,
        ignore,
        standard: StandardRules {
            predefined: vec![language_tag_rule()],
            ..Default::default()
        },
        cel: vec![CelRule {
            id: "language_code.short".to_string(),
            message: "too long".to_string(),
            expression: "this.size() <= 35".to_string(),
            is_cel_expression: false,
        }],
        oneof_index: None,
        oneof_name: None,
        is_legacy_required: false,
        is_group: false,
    };
    let message = MessageValidators {
        proto_name: "acme.v1.M".to_string(),
        package: "acme.v1".to_string(),
        source_file: "acme/v1/m.proto".to_string(),
        message_cel: Vec::new(),
        message_oneofs: Vec::new(),
        field_rules: vec![field],
        oneof_rules: Vec::new(),
        compile_error: None,
    };
    render(&[message])
        .expect("render must not fail")
        .into_iter()
        .filter_map(|file| file.content)
        .collect::<Vec<_>>()
        .join("\n")
}

fn compact(source: &str) -> String {
    source.split_whitespace().collect()
}

#[test]
fn predefined_rule_compiles_to_a_native_check() {
    let source = render_field(Ignore::Unspecified);
    assert!(
        !source.contains("unsupported CEL"),
        "the predefined rule must be transpiled:\n{source}"
    );
    assert!(source.contains("string.language_tag"), "{source}");
}

#[test]
fn matches_uses_the_runtime_regex_re_export() {
    let source = compact(&render_field(Ignore::Unspecified));
    assert!(
        source.contains("::protovalidate_buffa::regex::Regex::new("),
        "{source}"
    );
    assert!(!source.contains("OnceLock<::regex::"), "{source}");
    assert!(!source.contains("||::regex::"), "{source}");
}

#[test]
fn rule_path_names_the_extension_it_came_from() {
    let source = render_field(Ignore::Unspecified);
    assert!(source.contains("\"[acme.v1.language_tag]\""), "{source}");
    assert!(!source.contains("conformance.cases"), "{source}");
}

#[test]
fn ignore_if_zero_value_guards_predefined_and_cel_rules() {
    let guard = "if!self.language_code.is_empty(){{let__cel_this";
    let guarded = compact(&render_field(Ignore::IfZeroValue));
    // One guard per rule, in the owned and the view impl.
    assert_eq!(
        guarded.matches(guard).count(),
        4,
        "both the predefined and the cel rule need the zero-value guard:\n{guarded}"
    );
    let unguarded = compact(&render_field(Ignore::Unspecified));
    assert!(!unguarded.contains(guard), "{unguarded}");
}
