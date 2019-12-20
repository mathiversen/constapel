use constapel::Constapel;

#[test]
fn it_can_parse_yaml() {
    let yaml = r#"
      config:
        css:
          path: '.'
          files: many
          include: [colors]

      constants:
        colors:
          white: '#ffffff'
    "#;
    let c = Constapel::from_yaml(yaml.to_string()).unwrap();
    assert_eq!(&c.config["js"].files, "many");
}
