#[test]
fn codegen() {
    use snapbox::assert_data_eq;
    use snapbox::Data;

    let tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let mut output_dir = std::path::PathBuf::from(tmp_dir);
    output_dir.push("lelwel");
    std::fs::create_dir_all(&output_dir).unwrap();

    lelwel::compile(
        "json.llw",
        output_dir.as_os_str().to_str().unwrap(),
        false,
        1,
        false,
        false,
    )
    .unwrap();

    let expected_root = std::path::Path::new(".");
    for entry in std::fs::read_dir(&output_dir).unwrap() {
        let entry = entry.unwrap();
        let actual_path = entry.path();
        let actual_name = entry.file_name();
        let actual = std::fs::read_to_string(&actual_path).unwrap();
        let expected_path = expected_root.join(actual_name);
        assert_data_eq!(actual, Data::read_from(&expected_path, None));
    }
}
