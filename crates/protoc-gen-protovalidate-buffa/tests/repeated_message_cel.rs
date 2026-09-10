//! Message lists must remain visible to CEL schema resolution.
use protoc_gen_protovalidate_buffa::{
    emit::render,
    scan::{CelRule, FieldKind, FieldValidator, Ignore, MessageValidators, StandardRules},
};

fn message(name: &str, field_name: &str, kind: FieldKind) -> MessageValidators {
    MessageValidators {
        proto_name: format!("test.v1.{name}"),
        package: "test.v1".into(),
        source_file: "test.proto".into(),
        message_cel: vec![],
        message_oneofs: vec![],
        field_rules: vec![FieldValidator {
            field_number: 1,
            field_name: field_name.into(),
            field_type: kind,
            required: false,
            ignore: Ignore::Unspecified,
            standard: StandardRules::default(),
            cel: vec![],
            oneof_index: None,
            oneof_name: None,
            is_legacy_required: false,
            is_group: false,
        }],
        oneof_rules: vec![],
        compile_error: None,
    }
}

#[test]
fn nested_repeated_message_size_compiles_without_runtime_error() {
    let mut request = message(
        "Request",
        "details",
        FieldKind::Message {
            full_name: "test.v1.Details".into(),
        },
    );
    request.message_cel.push(CelRule {
        id: "min_fields".into(),
        message: "Need two fields".into(),
        expression: "has(this.details) && size(this.details.fields) >= 2".into(),
        is_cel_expression: false,
    });
    let details = message(
        "Details",
        "fields",
        FieldKind::Repeated(Box::new(FieldKind::Message {
            full_name: "test.v1.Field".into(),
        })),
    );
    let field = message("Field", "key", FieldKind::String);
    let source = render(&[request, details, field])
        .unwrap()
        .into_iter()
        .filter_map(|file| file.content)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!source.contains("unsupported CEL"), "{source}");
    assert!(source.contains(".fields"), "{source}");
    assert!(source.contains(".len()"), "{source}");
}
