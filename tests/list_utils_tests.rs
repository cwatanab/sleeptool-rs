use sleeptool_rs::settings_gui::list_utils::{
    add_to_csv, parse_csv, remove_from_csv,
};

#[test]
fn parse_csv_empty() {
    assert!(parse_csv("").is_empty());
}

#[test]
fn parse_csv_single() {
    assert_eq!(parse_csv("foo"), vec!["foo"]);
}

#[test]
fn parse_csv_multiple() {
    assert_eq!(parse_csv("a, b, c"), vec!["a", "b", "c"]);
}

#[test]
fn parse_csv_trims_whitespace() {
    assert_eq!(parse_csv("  a  ,b  ,  c"), vec!["a", "b", "c"]);
}

#[test]
fn parse_csv_filters_empty() {
    assert_eq!(parse_csv("a,,b,  ,c"), vec!["a", "b", "c"]);
}

#[test]
fn parse_csv_lowercases() {
    assert_eq!(parse_csv("FOO, Bar"), vec!["foo", "bar"]);
}

#[test]
fn add_to_csv_appends_new() {
    assert_eq!(add_to_csv("", "foo"), "foo");
    assert_eq!(add_to_csv("a", "b"), "a, b");
}

#[test]
fn add_to_csv_dedupes_case_insensitive() {
    assert_eq!(add_to_csv("foo, bar", "FOO"), "foo, bar");
    assert_eq!(add_to_csv("foo", "foo"), "foo");
}

#[test]
fn add_to_csv_trims_input() {
    assert_eq!(add_to_csv("", "  hello  "), "hello");
}

#[test]
fn add_to_csv_ignores_empty() {
    assert_eq!(add_to_csv("a", ""), "a");
    assert_eq!(add_to_csv("a", "   "), "a");
}

#[test]
fn remove_from_csv_by_index() {
    assert_eq!(remove_from_csv("a, b, c", 1), "a, c");
    assert_eq!(remove_from_csv("a, b, c", 0), "b, c");
    assert_eq!(remove_from_csv("a, b, c", 2), "a, b");
}

#[test]
fn remove_from_csv_out_of_bounds() {
    assert_eq!(remove_from_csv("a, b, c", 5), "a, b, c");
}

#[test]
fn remove_from_csv_empty() {
    assert_eq!(remove_from_csv("", 0), "");
}

#[test]
fn roundtrip_parse_csv_add_to_csv() {
    let csv = "alpha.exe, beta.exe, gamma.exe";
    let items = parse_csv(csv);
    let mut reconstructed = String::new();
    for item in items {
        reconstructed = add_to_csv(&reconstructed, &item);
    }
    assert_eq!(reconstructed, csv);
}
