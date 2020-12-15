#[derive(Debug)]
pub struct ParseError {
    pub idx: usize,
    pub line: usize,
    pub col: usize,
}

impl ParseError {
    pub fn from_source_and_index(source: &str, idx: usize) -> ParseError {
        let substring = match source.get(0..idx + 1) {
            None => {
                return ParseError {
                    idx,
                    line: 0,
                    col: 0,
                }
            }
            Some(s) => s,
        };

        let (line, last_line) = substring
            .lines()
            .fold((0, None), |(count, _), line| (count + 1, Some(line)));

        ParseError {
            idx,
            line,
            col: last_line.unwrap().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_error::ParseError;

    #[test]
    fn basic() {
        let source = r#"
			import './export.js';
			import { s as p } from './reexport1.js';
			//     ^
			//     assume error here (3:20)
			import d from './export.js';
		"#;

        let err_idx = source.find('{').unwrap();
        let err = ParseError::from_source_and_index(source, err_idx);
        assert_eq!(err.line, 3);
        assert_eq!(err.col, 11);
    }

    #[test]
    fn empty_source() {
        let source = "";
        let err = ParseError::from_source_and_index(source, 42);
        assert_eq!(err.line, 0);
        assert_eq!(err.col, 0);
    }
}
