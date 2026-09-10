//! Exercise emitted validators, not just their generated source text.
#[allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    dead_code,
    non_camel_case_types,
    unused_imports,
    reason = "includes generated protobuf code"
)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));
}
use generated::buf::validate::conformance::cases::{
    __buffa::oneof::string_presence, StringPresence,
};
pub use generated::{buf::validate as pb_validate, google::protobuf as pb_google};
use protovalidate_buffa::Validate;

#[test]
fn optional_string_rules_validate_present_values_only() {
    assert!(StringPresence::default().validate().is_ok());
    for (value, accepted) in [
        ("", false),
        ("not-a-uuid", false),
        ("123e4567-e89b-12d3-a456-426614174000", true),
    ] {
        let message = StringPresence {
            identifier: Some(value.into()),
            ..Default::default()
        };
        assert_eq!(message.validate().is_ok(), accepted, "UUID {value:?}");
    }
    for (value, accepted) in [("", false), ("bad", false), ("person@example.com", true)] {
        let message = StringPresence {
            email: Some(value.into()),
            ..Default::default()
        };
        assert_eq!(message.validate().is_ok(), accepted, "email {value:?}");
    }
    for (value, accepted) in [("", false), ("green", false), ("red", true)] {
        let message = StringPresence {
            choice: Some(value.into()),
            ..Default::default()
        };
        assert_eq!(message.validate().is_ok(), accepted, "choice {value:?}");
    }
}

#[test]
fn oneof_string_rules_validate_selected_values() {
    for (value, accepted) in [
        ("", false),
        ("not-a-uuid", false),
        ("123e4567-e89b-12d3-a456-426614174000", true),
    ] {
        let message = StringPresence {
            selection: Some(string_presence::Selection::SelectedIdentifier(value.into())),
            ..Default::default()
        };
        assert_eq!(
            message.validate().is_ok(),
            accepted,
            "selected UUID {value:?}"
        );
    }
}

#[test]
fn optional_pattern_and_length_rules_are_retained() {
    for (value, accepted) in [
        ("", false),
        ("a", false),
        ("ABCDE", false),
        ("abcde", false),
        ("ab", true),
    ] {
        let message = StringPresence {
            code: Some(value.into()),
            ..Default::default()
        };
        assert_eq!(message.validate().is_ok(), accepted, "code {value:?}");
    }
}
