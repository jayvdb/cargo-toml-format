use cargo_fmt::{cargo_toml::CargoToml, toml_config::TomlFormatConfig};

#[test]
fn trims_empty_spaces_section_keys() {
    const BEFORE: &str = r#"[ a]
[b ]
a="a" # A description of the package.
[ c ]
b="b" # A description of the package.
"#;

    const AFTER: &str = r#"[a]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
"#;

    let mut config = TomlFormatConfig::new();
    config.trim_section_keys = true;

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_empty_spaces_keys() {
    const BEFORE: &str = r#"[a]
[b]

    a="a" # A description of the package.
[c]
    b="b" # A description of the package.
        c="c" # A description of the package.


[d] 
    
    d="d" # A description of the package.
"#;

    const AFTER: &str = r#"[a]
[b]
a="a" # A description of the package.
[c]
b="b" # A description of the package.
c="c" # A description of the package.
[d]
d="d" # A description of the package.
"#;

    let mut config = TomlFormatConfig::new();
    config.trim_keys = true;

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    println!("{}", toml.toml_document.to_string());
    toml.format();
    println!("{}", toml.toml_document.to_string());
    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trims_quotes_from_keys() {
    const BEFORE: &str = r#"[a]
    [b]
    "a"="a"
    [c]
    b={"a"={"a"="b"}} 
    "#;

    const AFTER: &str = r#"[a]
    [b]
    a="a"
    [c]
    b={a={a="b"}} 
    "#;

    let mut config = TomlFormatConfig::new();
    config.trim_key_quotes = true;

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}

#[test]
fn trimming_around_comments() {
    const BEFORE: &str = r#"[a] 
    a = { a = "b" } # A description of the package.
    b = { a = "b" }       # A description of the package.        
    c = "b"   #    A description of the package.
    d = []  # A description of the package.
    "#;

    const AFTER: &str = r#"[a]
a = { a = "b" } # A description of the package.
b = { a = "b" } # A description of the package.
c = "b" #    A description of the package.
d = [] # A description of the package.
    "#;

    let mut config = TomlFormatConfig::new();
    config.table_formatting = true;

    let mut toml = CargoToml::new(BEFORE.to_string(), config).unwrap();

    toml.format();

    assert_eq!(toml.toml_document.to_string(), AFTER);
}
