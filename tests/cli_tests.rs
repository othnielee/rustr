use rustr::cli::{parse_args_from, CliArgs};

fn v(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

#[test]
fn no_arguments() {
    let cfg = parse_args_from::<_, String>(Vec::<String>::new()).unwrap();
    assert_eq!(
        cfg,
        CliArgs {
            test: false,
            build: false,
            release: false,
            release_bin: None,
            project: None,
            project_name: None,
            project_args: vec![],
        }
    );
}

#[test]
fn positional_project_then_args() {
    let cfg = parse_args_from(v(&["myproj", "--foo", "bar"])).unwrap();
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(cfg.project_args, vec![String::from("--foo"), "bar".into()]);
}

#[test]
fn project_flag_anywhere() {
    let cfg = parse_args_from(v(&["--release", "foo", "--project=myproj", "--verbose"])).unwrap();

    assert!(cfg.release);
    assert_eq!(cfg.project, Some("myproj".into()));
    assert_eq!(
        cfg.project_args,
        vec![String::from("foo"), String::from("--verbose")]
    );
}

#[test]
fn release_bin_space_and_equals() {
    // space form
    let cfg = parse_args_from(v(&["--release-bin", "/opt/bin", "myproj", "--flag"])).unwrap();
    assert_eq!(cfg.release_bin, Some(Some("/opt/bin".into())));
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(cfg.project_args, vec![String::from("--flag")]);

    // equals form
    let cfg_eq = parse_args_from(v(&["myproj", "--release-bin=/custom"])).unwrap();
    assert_eq!(cfg_eq.release_bin, Some(Some("/custom".into())));
    assert_eq!(cfg_eq.project_name, Some("myproj".into()));
}

#[test]
fn release_bin_no_dest_defaults() {
    let cfg = parse_args_from(v(&["myproj", "--release-bin"])).unwrap();
    assert_eq!(cfg.release_bin, Some(None));
    assert_eq!(cfg.project_name, Some("myproj".into()));
}

#[test]
fn build_and_release_flags_any_order() {
    let cfg = parse_args_from(v(&["--build", "myproj", "--release"])).unwrap();
    assert!(cfg.build);
    assert!(cfg.release);
    assert_eq!(cfg.project_name, Some("myproj".into()));
}

#[test]
fn missing_project_value_error() {
    let err = parse_args_from::<_, String>(v(&["--project"])).unwrap_err();
    assert!(
        err.to_string().contains("Missing project name"),
        "unexpected error text: {err}"
    );
}

#[test]
fn unknown_long_option_pass_through() {
    let cfg = parse_args_from(v(&["myproj", "--unknown-flag", "-vv"])).unwrap();
    assert_eq!(
        cfg.project_args,
        vec![String::from("--unknown-flag"), String::from("-vv")]
    );
}

#[test]
fn dash_dash_as_argument_separator() {
    // Test that "--" acts as an explicit argument separator
    let cfg = parse_args_from(v(&["myproj", "--", "--build", "-h"])).unwrap();
    assert_eq!(cfg.project_name, Some("myproj".into()));
    // "--build" and "-h" should be treated as project args, not flags
    assert!(!cfg.build);
    assert_eq!(
        cfg.project_args,
        vec![String::from("--build"), String::from("-h")]
    );
}

#[test]
fn interleaved_flags_and_args() {
    // Test complex interleaving of flags and args
    let cfg = parse_args_from(v(&["--test", "arg1", "--project=myproj", "arg2"])).unwrap();
    assert!(cfg.test);
    assert_eq!(cfg.project, Some("myproj".into()));
    assert_eq!(
        cfg.project_args,
        vec![String::from("arg1"), String::from("arg2")]
    );
}

#[test]
fn empty_arg_list() {
    // Test with empty arg list but with an = sign
    let cfg = parse_args_from(v(&["--project="])).unwrap_err();
    assert!(cfg.to_string().contains("Missing project name"));
}

#[test]
fn single_dash_options() {
    // Test that single dash options other than -h and -V are passed through
    let cfg = parse_args_from(v(&["myproj", "-x", "-foo"])).unwrap();
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(
        cfg.project_args,
        vec![String::from("-x"), String::from("-foo")]
    );
}

#[test]
fn project_with_hyphen() {
    // Test that a project name with a hyphen is handled correctly
    let cfg = parse_args_from(v(&["my-project", "--build"])).unwrap();
    assert!(cfg.build);
    assert_eq!(cfg.project_name, Some("my-project".into()));
}

#[test]
fn multiple_flag_formats() {
    // Test multiple flag formats in same command
    let cfg =
        parse_args_from(v(&["--test", "--project", "myproj", "--release-bin=/path"])).unwrap();

    assert!(cfg.test);
    assert_eq!(cfg.project, Some("myproj".into()));
    assert_eq!(cfg.release_bin, Some(Some("/path".into())));
}

#[test]
fn ordering_of_flags() {
    // Test that the order of flags doesn't affect precedence
    let cfg1 = parse_args_from(v(&["myproj", "--release", "--build"])).unwrap();
    let cfg2 = parse_args_from(v(&["myproj", "--build", "--release"])).unwrap();

    // Both should have build=true, release=true
    assert!(cfg1.build && cfg1.release);
    assert!(cfg2.build && cfg2.release);
}

#[test]
fn flags_after_project_name() {
    let cfg = parse_args_from(v(&["myproj", "--build", "--", "--some-arg"])).unwrap();
    assert!(cfg.build);
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(cfg.project_args, vec![String::from("--some-arg")]);
}

#[test]
fn whitespace_in_values() {
    // Test handling of whitespace in option values
    let cfg = parse_args_from(v(&[
        "--project",
        "my project",
        "--release-bin",
        "/path with spaces",
    ]))
    .unwrap();

    assert_eq!(cfg.project, Some("my project".into()));
    assert_eq!(cfg.release_bin, Some(Some("/path with spaces".into())));
}

#[test]
fn same_option_multiple_times() {
    // Test what happens when an option is specified multiple times
    // Last one should win
    let cfg = parse_args_from(v(&[
        "--project=first",
        "--project=second",
        "--release-bin=/path1",
        "--release-bin=/path2",
    ]))
    .unwrap();

    assert_eq!(cfg.project, Some("second".into()));
    assert_eq!(cfg.release_bin, Some(Some("/path2".into())));
}

#[test]
fn special_characters_in_values() {
    // Test handling of special characters in option values
    let cfg = parse_args_from(v(&[
        "--project=proj#$%@!",
        "--release-bin=/path/with/!@#$%^&*()",
    ]))
    .unwrap();

    assert_eq!(cfg.project, Some("proj#$%@!".into()));
    assert_eq!(cfg.release_bin, Some(Some("/path/with/!@#$%^&*()".into())));
}

#[test]
fn unicode_characters() {
    // Test handling of Unicode characters
    let cfg = parse_args_from(v(&["--project=项目名称", "--release-bin=/路径/到/二进制"])).unwrap();

    assert_eq!(cfg.project, Some("项目名称".into()));
    assert_eq!(cfg.release_bin, Some(Some("/路径/到/二进制".into())));
}

#[test]
fn multiple_dash_behavior() {
    let cfg = parse_args_from(v(&["myproj", "---weird-flag"])).unwrap();
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(cfg.project_args, vec![String::from("---weird-flag")]);
}

#[test]
fn escaped_characters() {
    let cfg =
        parse_args_from(v(&["--project=proj\\name", "--release-bin=/path\\to\\bin"])).unwrap();
    assert_eq!(cfg.project, Some("proj\\name".into()));
    assert_eq!(cfg.release_bin, Some(Some("/path\\to\\bin".into())));
}

#[test]
fn empty_project_args() {
    let cfg = parse_args_from(v(&["myproj", "", "--flag", ""])).unwrap();
    assert_eq!(cfg.project_name, Some("myproj".into()));
    assert_eq!(
        cfg.project_args,
        vec![String::from(""), String::from("--flag"), String::from("")]
    );
}

#[test]
fn deep_project_path() {
    let cfg = parse_args_from(v(&["--project=/very/deep/nested/path/project"])).unwrap();
    assert_eq!(cfg.project, Some("/very/deep/nested/path/project".into()));
}
