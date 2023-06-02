use std::io::Read;

fn invalid_json(c: char) {
    eprintln!("invalid json: {}", c);
    std::process::exit(1);
}

fn parse(s: String) -> Result<(), char> {
    let mut parser = vec![];

    s.bytes().try_for_each(|byte| -> Result<(), char> {
        let c = byte as char;
        // println!("{:?} {:?}", parser, c);

        match c {
            '{' | '[' => {
                parser.push(c);
                Ok(())
            }
            '}' => {
                let last = parser.pop().unwrap_or_default();
                match last {
                    '{' => Ok(()),
                    ':' | ',' => {
                        while parser.last().unwrap_or(&' ') != &'{' {
                            parser.pop();
                        }
                        parser.pop();
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            ']' => {
                let last = parser.pop().unwrap_or_default();
                match last {
                    '[' => Ok(()),
                    _ => Err(c),
                }
            }
            ',' if parser.last().unwrap_or(&' ') != &'"' => {
                parser.push(c);
                Ok(())
            }
            '\n' | '\t' | ' ' => Ok(()),
            c if c.is_numeric() && parser.last().unwrap_or(&' ') != &'"' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    ':' => Ok(()),
                    _ => Err(c),
                }
            }
            c if c.is_numeric() && parser.last().unwrap_or(&' ') == &'"' => Ok(()),
            '"' => {
                let last: char = parser.pop().unwrap_or_default();
                match last {
                    '"' => Ok(()),
                    _ => {
                        parser.push(last);
                        parser.push(c);
                        Ok(())
                    }
                }
            }
            't' if parser.last().unwrap_or(&' ') != &'"' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    ':' => {
                        parser.push(c);
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            'r' if parser.last().unwrap_or(&' ') != &'"' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    't' => {
                        parser.push(c);
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            'u' if parser.last().unwrap_or(&' ') == &'r' => {
                parser.push(c);
                Ok(())
            }
            'e' if parser.last().unwrap_or(&' ') == &'u' => {
                parser.pop().unwrap();
                parser.pop().unwrap();
                parser.pop().unwrap();
                Ok(())
            }
            'f' if parser.last().unwrap_or(&' ') != &'"' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    ':' => {
                        parser.push(c);
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            'a' if parser.last().unwrap_or(&' ') == &'f' => {
                parser.push(c);
                Ok(())
            }
            'l' if parser.last().unwrap_or(&' ') == &'a' => {
                parser.push(c);
                Ok(())
            }
            's' if parser.last().unwrap_or(&' ') == &'l' => {
                parser.push(c);
                Ok(())
            }
            'e' if parser.last().unwrap_or(&' ') == &'s' => {
                parser.pop().unwrap();
                parser.pop().unwrap();
                parser.pop().unwrap();
                parser.pop().unwrap();
                Ok(())
            }
            'n' if parser.last().unwrap_or(&' ') != &'"' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    ':' => {
                        parser.push(c);
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            'u' if parser.last().unwrap_or(&' ') == &'n' => {
                parser.push(c);
                Ok(())
            }
            'l' if parser.last().unwrap_or(&' ') == &'u' => {
                parser.push(c);
                Ok(())
            }
            'l' if parser.last().unwrap_or(&' ') == &'l' => {
                parser.pop();
                parser.pop();
                parser.pop();
                Ok(())
            }
            ':' => {
                let last = parser.last().unwrap_or(&' ');
                match last {
                    '{' | ',' => {
                        parser.push(c);
                        Ok(())
                    }
                    _ => Err(c),
                }
            }
            c if c.is_ascii() && parser.last().unwrap_or(&' ') == &'"' => Ok(()),

            c => Err(c),
        }
    })?;

    if parser.is_empty() {
        Ok(())
    } else {
        Err(parser.pop().unwrap_or_default())
    }
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .expect("usage: jsonparser <json-file>");

    let mut contents = String::new();
    std::fs::File::open(filepath)
        .expect("could not open file")
        .read_to_string(&mut contents)
        .expect("could not read file");

    match parse(contents) {
        Ok(_) => println!("valid json"),
        Err(c) => invalid_json(c),
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn empty_json() {
        assert!(parse(r#"{}"#.to_string()).is_ok());
        assert!(parse(r#"[]"#.to_string()).is_ok());
        assert!(parse(r#"[{}]"#.to_string()).is_ok());
    }

    #[test]
    #[ignore]
    fn list_in_dict() {
        assert!(parse(r#"{[]}"#.to_string()).is_err());
    }

    #[test]
    #[ignore]
    fn empty_key_value_invalid_json() {
        assert!(parse(r#"{:}"#.to_string()).is_err());
    }

    #[test]
    fn string() {
        assert!(parse(r#"{"key": "value"}"#.to_string()).is_ok());
    }

    #[test]
    fn numeric() {
        assert!(parse(r#"{"a": 1}"#.to_string()).is_ok());
    }

    #[test]
    fn boolean() {
        assert!(parse(r#"{"a": true}"#.to_string()).is_ok());
    }

    #[test]
    fn unquoted_string() {
        assert!(parse(r#"{"key": value}"#.to_string()).is_err());
    }

    #[test]
    fn null() {
        assert!(parse(r#"{"key": null}"#.to_string()).is_ok());
    }

    #[test]
    fn invalid_json() {
        assert!(parse(r#"{"#.to_string()).is_err());
        assert!(parse(r#"}"#.to_string()).is_err());
        assert!(parse(r#"1"#.to_string()).is_err());
        assert!(parse(r#"["#.to_string()).is_err());
        assert!(parse(r#"]"#.to_string()).is_err());
    }

    #[test]
    fn combination_of_data_types() {
        let s = r#"{"key1": true, "key2": false, "key3": null, "key4": "value", "key5": 101}"#;
        assert!(parse(s.to_string()).is_ok());
    }

    #[test]
    fn array_and_dict_as_value() {
        let s = r#"{"key": "value", "key-n": 101, "key-o": {}, "key-l": []}"#;
        assert!(parse(s.to_string()).is_ok());
    }

    #[test]
    #[ignore]
    fn fixtures() {
        let fixtures = std::fs::read_dir("fixtures").unwrap();

        let mut failed = vec![];

        for fixture in fixtures {
            let filepath = fixture.unwrap().path();
            let filename = filepath.file_name().unwrap().to_str().unwrap();
            let mut contents = String::new();
            std::fs::File::open(&filepath)
                .expect("could not open file")
                .read_to_string(&mut contents)
                .expect("could not read file");

            if filename.contains("fail") {
                if !parse(contents).is_err() {
                    failed.push(filename.to_string());
                }
            } else if filename.contains("pass") {
                if parse(contents).is_err() {
                    failed.push(filename.to_string());
                }
            }
        }

        println!("{:?}", failed);
        assert!(failed.is_empty());
    }
}
