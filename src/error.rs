use std::fmt::{Display, Formatter};
use std::{fmt, str};

#[derive(Debug)]
pub struct ParseError {
    pub idx: usize,
    pub line: usize,
    pub col: usize,
    pub msg: Option<&'static str>,
}

impl ParseError {
    pub fn from_source_and_index<'a, T: AsRef<[u8]>>(source: T, idx: usize) -> ParseError {
        ParseError::create(source, idx, None)
    }

    pub fn from_source_index_and_msg<'a, T: AsRef<[u8]>>(
        source: T,
        idx: usize,
        msg: &'static str,
    ) -> ParseError {
        ParseError::create(source, idx, Some(msg))
    }

    fn create<'a, T: AsRef<[u8]>>(source: T, idx: usize, msg: Option<&'static str>) -> ParseError {
        let substring = match str::from_utf8(source.as_ref())
            .ok()
            .and_then(|s| s.get(0..idx + 1))
        {
            None => {
                return ParseError {
                    idx,
                    line: 0,
                    col: 0,
                    msg,
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
            msg,
        }
    }
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError at {}:{}", self.line, self.col)
    }
}

pub fn pretty_error(src: &str, err: &ParseError) -> String {
    let msg = err.msg.unwrap_or("unexpected token");
    let line_number = err.line.to_string();
    let pad_width = line_number.len() + 1;
    let line_code = src
        .lines()
        .skip(err.line - 1)
        .next()
        .expect("line is not in source");

    let mut output = format!("ParseError: at {}:{}\n", err.line, err.col);
    output += &format!("{:width$}|\n", "", width = pad_width);
    output += &format!("{:width$}|{}\n", line_number, line_code, width = pad_width);
    output += &format!(
        "{:width$}|{}^ {}\n",
        "",
        " ".repeat(err.col - 1),
        msg,
        width = pad_width
    );

    output
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;

    #[test]
    fn basic() {
        let source = r#"
			import './export.js';
			import { s as p } from './reexport1.js';
			//     ^ assume error here (3:20)
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
