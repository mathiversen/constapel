use constapel::Constapel;

#[test]
fn parse_yaml_string() {
  let yaml = r#"
      output_files:
        js:
          path: '.'
          constants:
            - colors
      constants:
        colors:
          white: '#ffffff'
    "#;
  let c = Constapel::from_yaml(yaml.to_string()).unwrap();
  assert_eq!(&c.output_files["js"].path, ".");
  assert_eq!(
    &c.constants["colors"][&serde_yaml::Value::String("white".to_string())],
    "#ffffff"
  );
}
