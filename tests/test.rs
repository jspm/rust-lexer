#[cfg(test)]
mod tests {
    use rust_lexer::{parse, pretty_error, SourceAnalysis};
    use std::fs::{read_dir, File};
    use std::io::Read;
    use std::path::PathBuf;

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
            .filter(|(path, _)| {
                !path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .ends_with(".min.js")
            })
            .collect();

        for (path, content) in entries {
            match parse(&content) {
                Ok(_analysis) => {}
                Err(err) => {
                    println!(">>>>>>>>> {:?}", &path);
                    let msg = pretty_error(&content, &err);
                    eprintln!("{}", msg);
                    panic!("failed to parse {:?}", &path)
                }
            };
        }
    }
}
