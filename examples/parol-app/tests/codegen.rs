#[test]
fn codegen() {
    use snapbox::assert_data_eq;
    use snapbox::Data;

    let tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let mut output_dir = std::path::PathBuf::from(tmp_dir);
    output_dir.push("parol");
    std::fs::create_dir_all(&output_dir).unwrap();

    let expected_root = std::path::Path::new(".");

    let mut builder = parol::build::Builder::with_explicit_output_dir(&output_dir);
    builder.grammar_file("json.par");
    builder.parser_output_file("parser.rs");
    builder.actions_output_file("grammar_trait.rs");
    builder.generate_parser().unwrap();

    for entry in std::fs::read_dir(&output_dir).unwrap() {
        let entry = entry.unwrap();
        let actual_path = entry.path();
        let actual_name = entry.file_name();
        let actual = std::fs::read_to_string(&actual_path).unwrap();
        let expected_path = expected_root.join(actual_name);
        assert_data_eq!(actual, Data::read_from(&expected_path, None).raw());
    }
}
