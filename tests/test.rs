#[cfg(test)]
mod tests {
    use std::fs::{read_dir, File};
    use std::io::Read;
    use std::path::PathBuf;

    use es_module_lexer::{parse, pretty_error};

    #[test]
    fn test_parse_fixtures() {
        let entries: Vec<(PathBuf, String)> = read_dir("fixtures")
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();

                let mut file = File::open(entry.path()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                (entry.path(), contents)
            })
            .collect();

        for (path, content) in entries {
            match parse(&content) {
                Ok(_analysis) => {}
                Err(err) => {
                    let msg = pretty_error(&content, &err);
                    eprintln!("{}", msg);
                    panic!("failed to parse {:?}", &path)
                }
            };
        }
    }
}
