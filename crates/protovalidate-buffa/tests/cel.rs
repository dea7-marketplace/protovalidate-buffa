//! Boundary behavior for runtime CEL conversion utilities.
use protovalidate_buffa::cel::parse_duration;

#[test]
fn duration_units_require_a_number_and_supported_suffix() {
    for (input, seconds) in [
        ("1h", 3600),
        ("-1h", -3600),
        ("+0.5h", 1800),
        ("1m", 60),
        ("1s", 1),
    ] {
        assert_eq!(
            parse_duration(input).map(|value| value.num_seconds()),
            Some(seconds),
            "{input}"
        );
    }
    for input in ["", "h", "1", "1day", "+", "NaNh"] {
        assert!(parse_duration(input).is_none(), "{input}");
    }
}
