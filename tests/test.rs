use constapel::Constapel;

#[test]
fn it_can_parse_yaml() {
  let yaml = r#"
      output_files:
        js:
          path: '.'
          files: many
          constants: [colors]
      constants:
        colors:
          white: '#ffffff'
    "#;
  let c = Constapel::from_yaml(yaml.to_string()).unwrap();
  assert_eq!(&c.output_files["files"].path, "many");
}