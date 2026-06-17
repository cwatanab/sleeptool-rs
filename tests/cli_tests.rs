use sleeptool_rs::cli::parse_args;

#[test]
fn parse_args_empty_yields_default() {
    let cli = parse_args(&[]).unwrap();
    assert!(!cli.hibernate);
    assert!(!cli.legacy_input);
}

#[test]
fn parse_args_hibernate_short() {
    let cli = parse_args(&["-H".to_string()]).unwrap();
    assert!(cli.hibernate);
    assert!(!cli.legacy_input);
}

#[test]
fn parse_args_hibernate_long() {
    let cli = parse_args(&["--hibernate".to_string()]).unwrap();
    assert!(cli.hibernate);
}

#[test]
fn parse_args_legacy_input_short() {
    let cli = parse_args(&["-O".to_string()]).unwrap();
    assert!(cli.legacy_input);
    assert!(!cli.hibernate);
}

#[test]
fn parse_args_legacy_input_long() {
    let cli = parse_args(&["--legacy-input".to_string()]).unwrap();
    assert!(cli.legacy_input);
}

#[test]
fn parse_args_combined() {
    let cli = parse_args(&["-H".to_string(), "-O".to_string()]).unwrap();
    assert!(cli.hibernate);
    assert!(cli.legacy_input);
}

#[test]
fn parse_args_unknown_flag_is_ignored() {
    let cli = parse_args(&["--unknown".to_string()]).unwrap();
    assert!(!cli.hibernate);
    assert!(!cli.legacy_input);
}

#[test]
fn parse_args_help_exits_with_ok() {
    let result = parse_args(&["-h".to_string()]);
    assert!(result.is_err());
}

#[test]
fn parse_args_help_long_exits_with_ok() {
    let result = parse_args(&["--help".to_string()]);
    assert!(result.is_err());
}

#[test]
fn parse_args_version_exits_with_ok() {
    let result = parse_args(&["-V".to_string()]);
    assert!(result.is_err());
}

#[test]
fn parse_args_version_long_exits_with_ok() {
    let result = parse_args(&["--version".to_string()]);
    assert!(result.is_err());
}
