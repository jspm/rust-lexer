use parse_error::ParseError;

mod parse_error;

#[derive(Debug)]
pub enum Import {
    Dynamic(DynamicImport),
    Static(StaticImport),
    Meta(MetaImport),
}

#[derive(Debug)]
pub struct DynamicImport {
    pub statement_start: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct StaticImport {
    pub statement_start: usize,
    pub start: usize,
    pub end: usize,
    pub statement_end: usize,
}

#[derive(Debug)]
pub struct MetaImport {
    pub statement_start: usize,
    pub start: usize,
    pub end: usize,
    pub statement_end: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Export {
    pub start: usize,
    pub end: usize,
}

impl Export {
    pub fn to_string<'a>(self: &Export, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

#[derive(Debug)]
pub struct ParseState<'a> {
    src: &'a [u8],
    i: usize,
    template_stack: Vec<usize>,
    open_token_index_stack: Vec<usize>,
    template_depth: Option<usize>,
    open_token_depth: usize,
    last_token_index: usize,
    next_brace_is_class: bool,
    open_class_index_stack: Vec<bool>,
    last_dynamic_import: Option<usize>,
    analysis: SourceAnalysis,
}

#[derive(Debug)]
pub struct SourceAnalysis {
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

pub fn main() {
    let source = "hello world";
    let analysis = parse(source).expect("Parse error");

    for import in analysis.imports {
        let start = match &import {
            Import::Dynamic(impt) => impt.start,
            Import::Static(impt) => impt.start,
            Import::Meta(impt) => impt.start,
        };
        let end = match &import {
            Import::Dynamic(impt) => impt.end,
            Import::Static(impt) => impt.end,
            Import::Meta(impt) => impt.end,
        };
        println!(
            "Import: {}",
            std::str::from_utf8(&source.as_bytes()[start..end]).expect("Invalid utf8")
        );
    }

    for export in analysis.exports {
        let start = export.start;
        let end = export.end;
        println!(
            "Export: {}",
            std::str::from_utf8(&source.as_bytes()[start..end]).expect("Invalid utf8")
        );
    }
}

pub fn parse(input: &str) -> Result<SourceAnalysis, ParseError> {
    let mut state = ParseState {
        src: input.as_bytes(),
        i: 0,
        template_stack: Vec::<usize>::with_capacity(10),
        open_token_index_stack: Vec::<usize>::with_capacity(50),
        template_depth: None,
        open_token_depth: 0,
        last_token_index: usize::MAX,
        open_class_index_stack: Vec::<bool>::with_capacity(10),
        next_brace_is_class: false,
        last_dynamic_import: None,
        analysis: SourceAnalysis {
            imports: Vec::with_capacity(20),
            exports: Vec::with_capacity(20),
        },
    };

    let mut first = true;
    let mut skip_set_last_token = false;
    let mut last_slash_was_division = false;

    let len = state.src.len();
    while state.i < len - 1 {
        if first {
            first = false;
        } else {
            state.i += 1;
        }
        let ch = state.src[state.i];

        if ch == ' ' as u8 || ch < 14 && ch > 8 {
            continue;
        }

        match ch as char {
            'e' => {
                if state.open_token_depth == 0
                    && keyword_start(state.src, state.i)
                    && &state.src[state.i + 1..state.i + 6] == b"xport"
                {
                    try_parse_export_statement(&mut state)?;
                }
            }
            'i' => {
                if keyword_start(state.src, state.i)
                    && &state.src[state.i + 1..state.i + 6] == b"mport"
                {
                    try_parse_import_statement(&mut state)?;
                }
            }
            'c' => {
                if keyword_start(state.src, state.i)
                    && &state.src[state.i + 1..state.i + 5] == b"lass"
                    && is_br_or_ws(state.src[state.i + 5])
                {
                    state.next_brace_is_class = true;
                }
            }
            '(' => {
                state
                    .open_token_index_stack
                    .resize(state.open_token_depth + 1, 0);
                state.open_token_index_stack[state.open_token_depth] = state.last_token_index;
                state.open_token_depth += 1;
            }
            ')' => {
                if state.open_token_depth == 0 {
                    return Err(ParseError::from_source_and_index(input, state.i));
                }
                state.open_token_index_stack.pop();
                state.open_token_depth -= 1;
                if let Some(di) = state.last_dynamic_import {
                    if state.analysis.imports.len() == di + 1 {
                        match &mut state.analysis.imports[di] {
                            Import::Dynamic(import) => import.end = state.i,
                            _ => panic!("Expected dynamic import"),
                        }
                    }
                }
            }
            '{' => {
                // dynamic import followed by { is not a dynamic import (so remove)
                // this is a sneaky way to get around { import () {} } v { import () }
                // block / object ambiguity without a parser (assuming source is valid)
                if let Some(di) = state.last_dynamic_import {
                    if state.analysis.imports.len() == di + 1 {
                        match &state.analysis.imports[di] {
                            Import::Dynamic(import) => {
                                if import.end == state.last_token_index {
                                    state.analysis.imports.pop();
                                }
                            }
                            _ => panic!("Expected dynamic import"),
                        }
                    }
                }
                state
                    .open_class_index_stack
                    .resize(state.open_token_depth + 1, false);
                state.open_class_index_stack[state.open_token_depth] = state.next_brace_is_class;
                state.next_brace_is_class = false;
                state.open_token_depth += 1;
            }
            '}' => {
                if state.open_token_depth == 0 {
                    return Err(ParseError::from_source_and_index(input, state.i));
                }
                if let Some(td) = state.template_depth {
                    if state.open_token_depth == td {
                        state.open_token_depth -= 1;
                        state.template_depth = state.template_stack.pop();
                        template_string(&mut state)?;
                    } else {
                        state.open_token_depth -= 1;
                        if state.open_token_depth < td {
                            return Err(ParseError::from_source_and_index(input, state.i));
                        }
                    }
                }
            }
            '\'' => {
                single_quote_string(&mut state)?;
            }
            '"' => {
                double_quote_string(&mut state)?;
            }
            '/' => {
                let next_ch = state.src[state.i + 1] as char;
                if next_ch == '/' {
                    line_comment(&mut state)?;
                    // dont update lastToken
                    skip_set_last_token = true;
                } else if next_ch == '*' {
                    block_comment(&mut state)?;
                    // dont update lastToken
                    skip_set_last_token = true;
                } else {
                    // Division / regex ambiguity handling based on checking backtrack analysis of:
                    // - what token came previously (lastToken)
                    // - if a closing brace or paren, what token came before the corresponding
                    //   opening brace or paren (lastOpenTokenIndex)
                    let last_token = if state.last_token_index == usize::MAX {
                        '\0'
                    } else {
                        state.src[state.last_token_index] as char
                    };
                    if last_token == '\u{0}'
                        || is_expression_punctuator(last_token as u8)
                            && !(last_token == '.'
                                && (state.last_token_index > 1
                                    && state.src[state.last_token_index - 1] >= b'0'
                                    && state.last_token_index > 1
                                    && state.src[state.last_token_index - 1] <= b'9'))
                            && !(last_token == '+'
                                && state.last_token_index > 1
                                && state.src[state.last_token_index - 1] == b'+')
                            && !(last_token == '-'
                                && state.last_token_index > 1
                                && state.src[state.last_token_index - 1] == b'-')
                        || last_token == ')'
                            && is_paren_keyword(
                                state.src,
                                state.open_token_index_stack[state.open_token_depth],
                            )
                        || last_token == '}'
                            && (is_expression_terminator(
                                state.src,
                                state.open_token_index_stack[state.open_token_depth],
                            ) || state.open_class_index_stack[state.open_token_depth])
                        || is_expression_keyword(state.src, state.last_token_index)
                        || last_token == '/' && last_slash_was_division
                    {
                        regular_expression(&mut state)?;
                        last_slash_was_division = false;
                    } else {
                        last_slash_was_division = true;
                    }
                }
            }
            '`' => {
                template_string(&mut state)?;
            }
            _ => {}
        }
        if skip_set_last_token {
            skip_set_last_token = false;
        } else {
            state.last_token_index = state.i;
        }
    }

    if state.template_depth.is_some() || state.open_token_index_stack.len() > 0 {
        return Err(ParseError::from_source_and_index(input, state.i));
    }

    Ok(state.analysis)
}

fn try_parse_import_statement(state: &mut ParseState) -> Result<(), ParseError> {
    let start_index = state.i;

    state.i += 6;

    let ch = comment_whitespace(state)? as char;
    match ch {
        // dynamic import
        '(' => {
            state.open_token_depth += 1;
            *state
                .open_token_index_stack
                .get_mut(state.open_token_depth)
                .expect("out of capacity") = start_index;
            if state.src[state.last_token_index] == b'.' {
                return Ok(());
            }
            // dynamic import indicated by positive d
            state.analysis.imports.push(Import::Dynamic(DynamicImport {
                statement_start: start_index,
                start: state.i + 1,
                end: 0,
            }));
            return Ok(());
        }
        // import.meta
        '.' => {
            state.i += 1;
            let ch = comment_whitespace(state)?;
            // import.meta indicated by d == -2
            if ch == 'm'
                && &state.src[state.i + 1..state.i + 3] == b"eta"
                && state.src[state.last_token_index] != b'.'
            {
                state.analysis.imports.push(Import::Meta(MetaImport {
                    statement_start: start_index,
                    start: start_index,
                    end: state.i + 4,
                    statement_end: state.i + 4,
                }));
            }
            return Ok(());
        }

        _ => {
            // no space after "import" -> not an import keyword
            if ch != '"' && ch != '\'' && ch != '{' && ch != '*' && state.i == start_index + 6 {
                return Ok(());
            }

            // import statement only permitted at base-level
            if state.open_token_depth != 0 {
                state.i -= 1;
                return Ok(());
            }
            while state.i < state.src.len() {
                let ch = state.src[state.i] as char;
                if ch == '\'' || ch == '"' {
                    read_import_string(start_index, ch, state)?;
                    return Ok(());
                }
                state.i += 1;
            }
            return Err(ParseError::from_source_and_index_u8(state.src, state.i));
        }
    }
}

fn try_parse_export_statement(state: &mut ParseState) -> Result<(), ParseError> {
    let s_start_pos = state.i;

    state.i += 6;

    let cur_pos = state.i;

    let mut ch = comment_whitespace(state)?;

    if state.i == cur_pos && !is_punctuator(ch as u8) {
        return Ok(());
    }

    match ch {
        // export default ...
        'd' => {
            state.analysis.exports.push(Export {
                start: state.i,
                end: state.i + 7,
            });
            return Ok(());
        }

        // export async? function*? name () {
        'a' => {
            state.i += 5;
            comment_whitespace(state)?;
            state.i += 8;
            ch = comment_whitespace(state)?;
            if ch == '*' {
                state.i += 1;
                comment_whitespace(state)?;
            }
            let start_pos = state.i;
            read_to_ws_or_punctuator(state);
            state.analysis.exports.push(Export {
                start: start_pos,
                end: state.i,
            });
            state.i -= 1;
            return Ok(());
        }
        'f' => {
            state.i += 8;
            ch = comment_whitespace(state)?;
            if ch == '*' {
                state.i += 1;
                comment_whitespace(state)?;
            }
            let start_pos = state.i;
            read_to_ws_or_punctuator(state);
            state.analysis.exports.push(Export {
                start: start_pos,
                end: state.i,
            });
            state.i -= 1;
            return Ok(());
        }

        'c' | 'v' | 'l' => {
            if ch == 'c' {
                if &state.src[state.i + 1..state.i + 5] == b"lass"
                    && is_br_or_ws_or_punctuator_not_dot(state.src[state.i + 5])
                {
                    state.i += 5;
                    comment_whitespace(state)?;
                    let start_pos = state.i;
                    read_to_ws_or_punctuator(state);
                    state.analysis.exports.push(Export {
                        start: start_pos,
                        end: state.i,
                    });
                    state.i -= 1;
                    return Ok(());
                }
                state.i += 2;
            }

            // export var/let/const name = ...(, name = ...)+
            // destructured initializations not currently supported (skipped for { or [)
            // also, lexing names after variable equals is skipped (export var p = function () { ... }, q = 5 skips "q")
            state.i += 2;
            loop {
                state.i += 1;
                comment_whitespace(state)?;
                let start_pos = state.i;
                ch = read_to_ws_or_punctuator(state) as char;
                // stops on [ { destructurings or =
                if ch == '{' || ch == '[' || ch == '=' {
                    state.i -= 1;
                    return Ok(());
                }
                if state.i == start_pos {
                    return Ok(());
                }
                state.analysis.exports.push(Export {
                    start: start_pos,
                    end: state.i,
                });
                ch = comment_whitespace(state)?;
                if ch != ',' {
                    break;
                }
            }
            state.i -= 1;
            return Ok(());
        }

        // export {...}
        '{' => {
            state.i += 1;
            comment_whitespace(state)?;
            loop {
                let start_pos = state.i;
                read_to_ws_or_punctuator(state) as char;
                let end_pos = state.i;
                comment_whitespace(state)?;
                ch = read_export_as(state, start_pos, end_pos)? as char;
                // ,
                if ch == ',' {
                    state.i += 1;
                    ch = comment_whitespace(state)?;
                }
                if ch == '}' {
                    break;
                }
                if state.i == start_pos {
                    return Err(ParseError::from_source_and_index_u8(state.src, state.i));
                }
                if state.i > state.src.len() {
                    return Err(ParseError::from_source_and_index_u8(state.src, state.i));
                }
            }
            state.i += 1;
            comment_whitespace(state)?;
            state.i += 1;
            comment_whitespace(state)?;
            read_export_as(state, state.i, state.i)? as char;
            ch = comment_whitespace(state)?;
            if ch == 'f' && &state.src[state.i + 1..state.i + 4] == b"rom" {
                state.i += 4;
                read_import_string(s_start_pos, comment_whitespace(state)?, state)?;
            }
        }

        // export *
        '*' => {
            state.i += 1;
            comment_whitespace(state)?;
            state.i += 1;
            comment_whitespace(state)?;
            read_export_as(state, state.i, state.i)? as char;
            ch = comment_whitespace(state)?;
            if ch == 'f' && &state.src[state.i + 1..state.i + 4] == b"rom" {
                state.i += 4;
                read_import_string(s_start_pos, comment_whitespace(state)?, state)?;
            }
        }

        _ => {}
    }
    state.i -= 1;
    return Ok(());
}

/// Parses an export specifier coming after the `as` keyword,
/// and advances the parsing state to the position until after the next non-whitespace or non-comment char.
fn read_export_as(
    state: &mut ParseState,
    mut start_pos: usize,
    mut end_pos: usize,
) -> Result<u8, ParseError> {
    if state.i >= state.src.len() {
        return Ok(0);
    }

    let ch = state.src[state.i];

    if ch == b'a' {
        state.i += 2;
        comment_whitespace(state)?;
        start_pos = state.i;
        read_to_ws_or_punctuator(state);
        end_pos = state.i;
        comment_whitespace(state)?;
    }

    if state.i != start_pos {
        state.analysis.exports.push(Export {
            start: start_pos,
            end: end_pos,
        });
    }

    Ok(ch)
}

fn read_import_string(
    statement_start: usize,
    ch: char,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    if ch == '\'' {
        state.i += 1;
        let start = state.i;
        single_quote_string(state)?;
        state.analysis.imports.push(Import::Static(StaticImport {
            statement_start,
            start,
            end: state.i,
            statement_end: state.i,
        }));
        return Ok(());
    } else if ch == '"' {
        state.i += 1;
        let start = state.i;
        double_quote_string(state)?;
        state.analysis.imports.push(Import::Static(StaticImport {
            statement_start,
            start,
            end: state.i,
            statement_end: state.i,
        }));
        return Ok(());
    } else {
        return Err(ParseError::from_source_and_index_u8(state.src, state.i));
    }
}

/// Consumes the all the whitespace or comments until the first character
/// that is not a part of either of them, advances the parsing state to that position,
/// and returns the whitespace char (`\0`).
fn comment_whitespace(state: &mut ParseState) -> Result<char, ParseError> {
    while state.i < state.src.len() {
        let ch = state.src[state.i] as char;
        if ch == '/' {
            let next_ch = state.src[state.i + 1] as char;
            if next_ch as char == '/' {
                line_comment(state)?;
            } else if next_ch == '*' {
                block_comment(state)?;
            } else {
                return Ok(ch);
            }
        } else if !is_br_or_ws(ch as u8) {
            return Ok(ch);
        }
        state.i += 1;
    }
    return Ok('\0');
}

fn template_string(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() {
        match state.src[state.i] as char {
            '$' => {
                if state.src[state.i + 1] as char == '{' {
                    state.i += 1;
                    state.template_stack.push(state.template_depth.unwrap());
                    state.open_token_depth += 1;
                    state.template_depth = Some(state.open_token_depth);
                    return Ok(());
                }
            }
            '`' => return Ok(()),
            '\\' => state.i += 1,
            _ => (),
        }
    }
    Err(ParseError::from_source_and_index_u8(state.src, state.i))
}

fn block_comment(state: &mut ParseState) -> Result<(), ParseError> {
    state.i += 1;
    while state.i < state.src.len() {
        state.i += 1;
        if state.src[state.i] as char == '*' && state.src[state.i + 1] as char == '/' {
            state.i += 1;
            return Ok(());
        }
    }
    Err(ParseError::from_source_and_index_u8(state.src, state.i))
}

fn line_comment(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() - 1 {
        state.i += 1;
        match state.src[state.i] as char {
            '\n' | '\r' => return Ok(()),
            _ => (),
        }
    }
    Ok(())
}

fn single_quote_string(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() - 1 {
        state.i += 1;
        match state.src[state.i] as char {
            '\'' => return Ok(()),
            '\\' => state.i += 1,
            '\n' | '\r' => break,
            _ => (),
        }
    }
    Err(ParseError::from_source_and_index_u8(state.src, state.i))
}

fn double_quote_string(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() - 1 {
        state.i += 1;
        match state.src[state.i] as char {
            '"' => return Ok(()),
            '\\' => state.i += 1,
            '\n' | '\r' => break,
            _ => (),
        }
    }
    return Err(ParseError::from_source_and_index_u8(state.src, state.i));
}

fn regex_character_class(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() - 1 {
        state.i += 1;
        match state.src[state.i] as char {
            ']' => return Ok(()),
            '\\' => state.i += 1,
            '\n' | '\r' => break,
            _ => (),
        }
    }
    return Err(ParseError::from_source_and_index_u8(state.src, state.i));
}

fn regular_expression(state: &mut ParseState) -> Result<(), ParseError> {
    while state.i < state.src.len() - 1 {
        state.i += 1;
        match state.src[state.i] as char {
            '/' => return Ok(()),
            '[' => regex_character_class(state)?,
            '\\' => state.i += 1,
            '\n' | '\r' => break,
            _ => (),
        }
    }
    return Err(ParseError::from_source_and_index_u8(state.src, state.i));
}

fn read_to_ws_or_punctuator(state: &mut ParseState) -> u8 {
    // This would probably be more "rusty", but I'm not sure about performance of it,
    // we can test it later when we add benchmarks.
    //
    // state.src[i..]
    //     .iter()
    //     .find(|&&ch| is_br_or_ws(ch) || is_punctuator(ch))
    //     .unwrap_or(state.src.last().expect("state.src is empty"));
    while state.i < state.src.len() {
        let ch = state.src[state.i];
        if is_br_or_ws(ch) || is_punctuator(ch) {
            return ch;
        }
        state.i += 1;
    }
    return 0;
}

// Note: non-ascii BR and whitespace checks omitted for perf / footprint
// if there is a significant user need this can be reconsidered
// fn is_br(c: char) -> bool {
//     return c == '\r' || c == '\n';
// }

fn is_br_or_ws(c: u8) -> bool {
    return c > 8 && c < 14 || c == 32 || c == 160;
}

fn is_br_or_ws_or_punctuator_not_dot(c: u8) -> bool {
    return c > 8 && c < 14 || c == 32 || c == 160 || is_punctuator(c) && c as char != '.';
}

fn keyword_start(src: &[u8], i: usize) -> bool {
    return i == 0 || is_br_or_ws_or_punctuator_not_dot(src[i - 1]);
}

fn read_preceding_keyword(src: &[u8], i: usize, keyword_prefix: &[u8]) -> bool {
    let length = keyword_prefix.len();
    if i < length - 1 {
        return false;
    }
    if &src[(i + 1 - length)..i + 1] == keyword_prefix {
        if i == length - 1 || is_br_or_ws_or_punctuator_not_dot(src[i - length - 1]) {
            return true;
        }
    }
    false
}

fn is_expression_keyword(src: &[u8], i: usize) -> bool {
    match src[i] as char {
        'd' => match src[i - 1] as char {
            // void
            'i' => read_preceding_keyword(src, i - 2, b"vo"),
            // yield
            'l' => read_preceding_keyword(src, i - 2, b"yie"),
            _ => false,
        },
        'e' => match src[i - 1] as char {
            's' => match src[i - 2] as char {
                // else
                'l' => read_preceding_keyword(src, i - 3, b"e"),
                // case
                'a' => read_preceding_keyword(src, i - 3, b"c"),
                _ => false,
            },
            // delete
            't' => read_preceding_keyword(src, i - 2, b"dele"),
            _ => false,
        },
        'f' => {
            if src[i - 1] as char != 'o' || src[i - 2] as char != 'e' {
                false
            } else {
                match src[i - 3] as char {
                    // instanceof
                    'c' => read_preceding_keyword(src, i - 4, b"instan"),
                    // typeof
                    'p' => read_preceding_keyword(src, i - 4, b"ty"),
                    _ => false,
                }
            }
        }
        // in, return
        'n' => {
            read_preceding_keyword(src, i - 1, b"i") || read_preceding_keyword(src, i - 1, b"retur")
        }

        // do
        'o' => read_preceding_keyword(src, i - 1, b"d"),
        // debugger
        'r' => read_preceding_keyword(src, i - 1, b"debugge"),
        // await
        't' => read_preceding_keyword(src, i - 1, b"awai"),
        'w' => match src[i - 1] as char {
            // new
            'e' => read_preceding_keyword(src, i - 2, b"n"),
            // throw
            'o' => read_preceding_keyword(src, i - 2, b"thr"),
            _ => false,
        },
        _ => false,
    }
}

fn is_paren_keyword(src: &[u8], i: usize) -> bool {
    return match src[i] as char {
        'e' => read_preceding_keyword(src, i, b"whil"),
        'r' => read_preceding_keyword(src, i, b"fo"),
        'f' => read_preceding_keyword(src, i, b"i"),
        _ => false,
    };
}

fn is_punctuator(ch: u8) -> bool {
    // 23 possible punctuator endings: !%&()*+,-./:;<=>?[]^{}|~
    return ch as char == '!'
        || ch as char == '%'
        || ch as char == '&'
        || ch > 39 && ch < 48
        || ch > 57 && ch < 64
        || ch as char == '['
        || ch as char == ']'
        || ch as char == '^'
        || ch > 122 && ch < 127;
}

fn is_expression_punctuator(ch: u8) -> bool {
    // 20 possible expression endings: !%&(*+,-.:;<=>?[^{|~
    return ch as char == '!'
        || ch as char == '%'
        || ch as char == '&'
        || ch > 39 && ch < 47 && ch != 41
        || ch > 57 && ch < 64
        || ch as char == '['
        || ch as char == '^'
        || ch > 122 && ch < 127 && ch as char != '}';
}

// detects:
// => ; ) finally catch else class X
// as all of these followed by a { will indicate a statement brace
fn is_expression_terminator(src: &[u8], i: usize) -> bool {
    match src[i] as char {
        ';' | ')' => true,
        'y' => read_preceding_keyword(src, i, b"finall"),
        '>' => src[i - 1] as char == '=',
        'h' => read_preceding_keyword(src, i, b"catc"),
        'e' => read_preceding_keyword(src, i, b"els"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_export_as() {
    //     // TODO: we should probably refactor the functions that are taking the whole state struct,
    //     //       they are hard to test well because of the complicated setup.
    //     let mut state = ParseState {
    //         src: b"as bar  }",
    //         i: 0,
    //         template_stack: vec![],
    //         open_token_index_stack: vec![],
    //         template_depth: 0,
    //         open_token_depth: 0,
    //         last_token_index: 0,
    //         analysis: SourceAnalysis {
    //             imports: vec![],
    //             exports: vec![],
    //         },
    //     };

    //     read_export_as(&mut state, 0, 0);

    //     assert_eq!(state.analysis.exports, vec![Export { start: 3, end: 6 }]);
    //     assert_eq!(state.i, 8);
    // }

    #[test]
    fn test_is_expression_keyword() {
        // debugger, delete, do, else, in, instanceof, new,
        // return, throw, typeof, void, yield ,await
        assert!(is_expression_keyword(b"debugger", 7));
        assert!(is_expression_keyword(b"delete", 5));
        assert!(is_expression_keyword(b"do", 1));
        assert!(is_expression_keyword(b"else", 3));
        assert!(is_expression_keyword(b"in", 1));
        assert!(is_expression_keyword(b"instanceof", 9));
        assert!(is_expression_keyword(b"new", 2));
        assert!(is_expression_keyword(b"return", 5));
        assert!(is_expression_keyword(b"throw", 4));
        assert!(is_expression_keyword(b"typeof", 5));
        assert!(is_expression_keyword(b"void", 3));
        assert!(is_expression_keyword(b"yield", 4));
        assert!(is_expression_keyword(b"await", 4));
    }

    #[test]
    fn invalid_string() {
        let source = r#"import './export.js';

import d from './export.js';

import { s as p } from './reexport1.js';

import { z, q as r } from './reexport2.js';

   '

import * as q from './reexport1.js';

export { d as a, p as b, z as c, r as d, q }"#;

        let err = parse(source).err().unwrap();
        assert_eq!(err.line, 9);
        assert_eq!(err.col, 4);
    }

    #[test]
    fn invalid_export() {
        let source = r#"export { a = };"#;
        let err = parse(source).err().expect("Should error");
        assert_eq!(err.idx, 11);
    }

    #[test]
    fn single_parse_cases() {
        parse("export { x }").unwrap();
        parse("'asdf'").unwrap();
        parse("/asdf/").unwrap();
        parse("`asdf`").unwrap();
        parse("/**/").unwrap();
        parse("//").unwrap();
    }

    #[test]
    fn simple_export_with_unicode_conversions() {
        let source = r#"export var p𓀀s,q"#;
        let SourceAnalysis {
            imports, exports, ..
        } = parse(source).unwrap();
        assert_eq!(imports.len(), 0);
        assert_eq!(exports.len(), 2);
        assert_eq!(exports[0].to_string(source), "p𓀀s");
        assert_eq!(exports[1].to_string(source), "q");
    }

    // #[test]
    //   fn simple_import () {
    //     let source = r#"
    //       import test from "test";
    //       console.log(test);
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     const { s, e, ss, se, d } = imports[0];
    //     assert_eq!(d, -1);
    //     assert_eq!(source.slice(s, e), "test");
    //     assert_eq!(source.slice(ss, se), "import test from "test"");
    //     assert_eq!(exports.len(), 0);
    //   }

    //   #[test]
    //   fn import_export_with_comments () {
    //     let source = r#"

    //       import/* 'x' */ 'a';

    //       import /* 'x' */ 'b';

    //       export var z  /*  */
    //       export {
    //         a,
    //         // b,
    //         /* c */ d
    //       };
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 2);
    //     assert_eq!(source[imports[0].start..imports[0].end], "a");
    //     assert_eq!(source.slice(imports[0].statement_start, imports[0].statement_end), `import/* 'x' */ 'a'`);
    //     assert_eq!(source.slice(imports[1].s, imports[1].e), "b");
    //     assert_eq!(source.slice(imports[1].statement_start, imports[1].statement_end), `import /* 'x' */ 'b'`);
    //     assert_eq!(exports.toString(), "z,a,d");
    //   }

    //   #[test]
    //   fn exported_function () {
    //     let source = r#"
    //       export function a𓀀 () {

    //       }
    //       export class Q{

    //       }
    //     "#;
    //     const [, exports] = parse(source);
    //     assert_eq!(exports[0], "a𓀀");
    //     assert_eq!(exports[1], "Q");
    //   }

    //   #[test]
    //   fn export_destructuring () {
    //     let source = r#"
    //       export const { a, b } = foo;

    //       export { ok };
    //     "#;
    //     const [, exports] = parse(source);
    //     assert_eq!(exports[0], "ok");
    //   }

    //   #[test]
    //   fn minified_import_syntax () {
    //     let source = r#"import{TemplateResult as t}from"lit-html";import{a as e}from"./chunk-4be41b30.js";export{j as SVGTemplateResult,i as TemplateResult,g as html,h as svg}from"./chunk-4be41b30.js";window.JSCompiler_renameProperty='asdf';"#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 3);
    //     assert_eq!(imports[0].s, 32);
    //     assert_eq!(imports[0].e, 40);
    //     assert_eq!(imports[0].statement_start, 0);
    //     assert_eq!(imports[0].statement_end, 41);
    //     assert_eq!(imports[1].s, 61);
    //     assert_eq!(imports[1].e, 80);
    //     assert_eq!(imports[1].statement_start, 42);
    //     assert_eq!(imports[1].statement_end, 81);
    //     assert_eq!(imports[2].s, 156);
    //     assert_eq!(imports[2].e, 175);
    //     assert_eq!(imports[2].statement_start, 82);
    //     assert_eq!(imports[2].statement_end, 176);
    //   }

    //   #[test]
    //   fn more_minified_imports () {
    //     let source = r#"import"some/import.js";`
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     assert_eq!(imports[0].s, 7);
    //     assert_eq!(imports[0].e, 21);
    //     assert_eq!(imports[0].statement_start, 0);
    //     assert_eq!(imports[0].statement_end, 22);
    //   }

    //   #[test]
    //   fn return_bracket_division () {
    //     let source = r#"function variance(){return s/(a-1)}"#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //   }

    //   #[test]
    //   fn simple_reexport () {
    //     let source = r#"
    //       export { hello as default } from "test-dep";
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     const { s, e, ss, se, d } = imports[0];
    //     assert_eq!(d, -1);
    //     assert_eq!(source.slice(s, e), "test-dep");
    //     assert_eq!(source.slice(ss, se), "export { hello as default } from "test-dep"");

    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "default");
    //   }

    //   #[test]
    //   fn import_meta () {
    //     let source = r#"
    //       export var hello = 'world';
    //       console.log(import.meta.url);
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     const { s, e, ss, se, d } = imports[0];
    //     assert_eq!(d, -2);
    //     assert_eq!(ss, 53);
    //     assert_eq!(se, 64);
    //     assert_eq!(source.slice(s, e), "import.meta");
    //   }

    //   #[test]
    //   fn import_meta_edge_cases () {
    //     let source = r#"
    //       // Import meta
    //       import.
    //        meta
    //       // Not import meta
    //       a.
    //       import.
    //         meta
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     const { s, e, ss, se, d } = imports[0];
    //     assert_eq!(d, -2);
    //     assert_eq!(ss, 28);
    //     assert_eq!(se, 47);
    //     assert_eq!(source.slice(s, e), "import.\n       meta");
    //   }

    //   #[test]
    //   fn dynamic_import_method () {
    //     await init;
    //     let source = r#"
    //       class A {
    //         import() {
    //         }
    //       }
    //     "#;
    //     const [imports] = parse(source);
    //     assert_eq!(imports.len(), 0);
    //   }

    //   #[test]
    //   fn dynamic_import_edge_cases () {
    //     let source = r#"
    //       ({
    //         // not a dynamic import!
    //         import(not1) {}
    //       }
    //       {
    //         // is a dynamic import!
    //         import(is1);
    //       }
    //       a.
    //       // not a dynamic import!
    //       import(not2);
    //       a.
    //       b()
    //       // is a dynamic import!
    //       import(is2);

    //       const myObject = {
    //         import: ()=> import(some_url)
    //       }
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 3);
    //     var { s, e, ss, se, d } = imports[0];
    //     assert_eq!(ss, d);
    //     assert_eq!(se, 0);
    //     assert_eq!(source.substr(d, 6), "import");
    //     assert_eq!(source.slice(s, e), "is1");

    //     var { s, e, ss, se, d } = imports[1];
    //     assert_eq!(ss, d);
    //     assert_eq!(se, 0);
    //     assert_eq!(source.slice(s, e), "is2");

    //     var { s, e, ss, se, d } = imports[2];
    //     assert_eq!(ss, d);
    //     assert_eq!(se, 0);
    //     assert_eq!(source.slice(s, e), "some_url");
    //   }

    //   #[test]
    //   fn import_after_code () {
    //     let source = r#"
    //       export function f () {
    //         g();
    //       }

    //       import { g } from './test-circular2.js';
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     const { s, e, ss, se, d } = imports[0];
    //     assert_eq!(d, -1);
    //     assert_eq!(source.slice(s, e), "./test-circular2.js");
    //     assert_eq!(source.slice(ss, se), `import { g } from './test-circular2.js'`);
    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "f");
    //   }

    //   #[test]
    //   fn comments () {
    //     let source = r#"/*
    //     VERSION
    //   */import util from 'util';

    // //
    // function x() {
    // }

    //       /**/
    //       // '
    //       /* / */
    //       /*

    //          * export { b }
    //       \\*/export { a }

    //       function () {
    //         /***/
    //       }
    //     `
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 1);
    //     assert_eq!(source.slice(imports[0].s, imports[0].e), "util");
    //     assert_eq!(source.slice(imports[0].statement_start, imports[0].statement_end), `import util from 'util'`);
    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "a");
    //   }

    //   #[test]
    //   fn strings () {
    //     let source = r#"
    //       "";
    //       \`
    //         \${
    //           import(\`test/\${ import(b)}\`); /*
    //               \`  }
    //           */
    //         }
    //       \`
    //       export { a }
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 2);
    //     assert.notEqual(imports[0].d, -1);
    //     assert_eq!(imports[0].statement_start, imports[0].d);
    //     assert_eq!(imports[0].statement_end, 0);
    //     assert_eq!(source.slice(imports[0].d, imports[0].s), "import(");
    //     assert.notEqual(imports[1].d, -1);
    //     assert_eq!(imports[1].statement_start, imports[1].d);
    //     assert_eq!(imports[1].statement_end, 0);
    //     assert_eq!(source.slice(imports[1].d, imports[1].s), "import(");
    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "a");
    //   }

    //   #[test]
    //   fn bracket_matching () {
    //     pars")"
    //       instance.extend('parseExprAtom', function (nextMethod) {
    //         return function () {
    //           function parseExprAtom(refDestructuringErrors) {
    //             if (this.type === tt._import) {
    //               return parseDynamicImport.call(this);
    //             }
    //             return c(refDestructuringErrors);
    //           }
    //         }();
    //       }
    //       export { a }
    //     `);
    //   }

    //   #[test]
    //   fn division_regex_ambiguity () {
    //     let source = r#"
    //       /as)df/; x();
    //       a / 2; '  /  '
    //       while (true)
    //         /test'/
    //       x-/a'/g
    //       finally{}/a'/g
    //       (){}/d'export { b }/g
    //       ;{}/e'/g;
    //       {}/f'/g
    //       a / 'b' / c;
    //       /a'/ - /b'/;
    //       +{} /g -'/g'
    //       ('a')/h -'/g'
    //       if //x
    //       ('a')/i'/g;
    //       /asdf/ / /as'df/; // '
    //       \`\${/test/ + 5}\`
    //       /regex/ / x;
    //       function () {
    //         return /*asdf8*// 5/;
    //       }
    //       export { a };
    //     "#;
    //     let SourceAnalysis { imports, exports } = parse(source).unwrap();
    //     assert_eq!(imports.len(), 0);
    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "a");
    //   }

    //   #[test]
    //   fn template_string_expression_ambiguity() {
    //     let source = r#"
    //       \`$\`
    //       import 'a';
    //       \`\`
    //       export { b };
    //       \`a$b\`
    //       import(\`$\`);
    //       \`{$}\`
    //     "#;
    //     let analysis = parse(source).unwrap();
    //     assert_eq!(analysis.imports.len(), 2);
    //     assert_eq!(exports.len(), 1);
    //     assert_eq!(exports[0], "b");
    //   }

    //   #[test]
    //   fn many_exports() {
    //     let source = r#"
    //       export { _iconsCache as fas, prefix, faAbacus, faAcorn, faAd, faAddressBook, faAddressCard, faAdjust, faAirFreshener, faAlarmClock, faAlarmExclamation, faAlarmPlus, faAlarmSnooze, faAlicorn, faAlignCenter, faAlignJustify, faAlignLeft, faAlignRight, faAlignSlash, faAllergies, faAmbulance, faAmericanSignLanguageInterpreting, faAnalytics, faAnchor, faAngel, faAngleDoubleDown, faAngleDoubleLeft, faAngleDoubleRight, faAngleDoubleUp, faAngleDown, faAngleLeft, faAngleRight, faAngleUp, faAngry, faAnkh, faAppleAlt, faAppleCrate, faArchive, faArchway, faArrowAltCircleDown, faArrowAltCircleLeft, faArrowAltCircleRight, faArrowAltCircleUp, faArrowAltDown, faArrowAltFromBottom, faArrowAltFromLeft, faArrowAltFromRight, faArrowAltFromTop, faArrowAltLeft, faArrowAltRight, faArrowAltSquareDown, faArrowAltSquareLeft, faArrowAltSquareRight, faArrowAltSquareUp, faArrowAltToBottom, faArrowAltToLeft, faArrowAltToRight, faArrowAltToTop, faArrowAltUp, faArrowCircleDown, faArrowCircleLeft, faArrowCircleRight, faArrowCircleUp, faArrowDown, faArrowFromBottom, faArrowFromLeft, faArrowFromRight, faArrowFromTop, faArrowLeft, faArrowRight, faArrowSquareDown, faArrowSquareLeft, faArrowSquareRight, faArrowSquareUp, faArrowToBottom, faArrowToLeft, faArrowToRight, faArrowToTop, faArrowUp, faArrows, faArrowsAlt, faArrowsAltH, faArrowsAltV, faArrowsH, faArrowsV, faAssistiveListeningSystems, faAsterisk, faAt, faAtlas, faAtom, faAtomAlt, faAudioDescription, faAward, faAxe, faAxeBattle, faBaby, faBabyCarriage, faBackpack, faBackspace, faBackward, faBacon, faBadge, faBadgeCheck, faBadgeDollar, faBadgePercent, faBadgerHoney, faBagsShopping, faBalanceScale, faBalanceScaleLeft, faBalanceScaleRight, faBallPile, faBallot, faBallotCheck, faBan, faBandAid, faBarcode, faBarcodeAlt, faBarcodeRead, faBarcodeScan, faBars, faBaseball, faBaseballBall, faBasketballBall, faBasketballHoop, faBat, faBath, faBatteryBolt, faBatteryEmpty, faBatteryFull, faBatteryHalf, faBatteryQuarter, faBatterySlash, faBatteryThreeQuarters, faBed, faBeer, faBell, faBellExclamation, faBellPlus, faBellSchool, faBellSchoolSlash, faBellSlash, faBells, faBezierCurve, faBible, faBicycle, faBiking, faBikingMountain, faBinoculars, faBiohazard, faBirthdayCake, faBlanket, faBlender, faBlenderPhone, faBlind, faBlog, faBold, faBolt, faBomb, faBone, faBoneBreak, faBong, faBook, faBookAlt, faBookDead, faBookHeart, faBookMedical, faBookOpen, faBookReader, faBookSpells, faBookUser, faBookmark, faBooks, faBooksMedical, faBoot, faBoothCurtain, faBorderAll, faBorderBottom, faBorderCenterH, faBorderCenterV, faBorderInner, faBorderLeft, faBorderNone, faBorderOuter, faBorderRight, faBorderStyle, faBorderStyleAlt, faBorderTop, faBowArrow, faBowlingBall, faBowlingPins, faBox, faBoxAlt, faBoxBallot, faBoxCheck, faBoxFragile, faBoxFull, faBoxHeart, faBoxOpen, faBoxUp, faBoxUsd, faBoxes, faBoxesAlt, faBoxingGlove, faBrackets, faBracketsCurly, faBraille, faBrain, faBreadLoaf, faBreadSlice, faBriefcase, faBriefcaseMedical, faBringForward, faBringFront, faBroadcastTower, faBroom, faBrowser, faBrush, faBug, faBuilding, faBullhorn, faBullseye, faBullseyeArrow, faBullseyePointer, faBurgerSoda, faBurn, faBurrito, faBus, faBusAlt, faBusSchool, faBusinessTime, faCabinetFiling, faCalculator, faCalculatorAlt, faCalendar, faCalendarAlt, faCalendarCheck, faCalendarDay, faCalendarEdit, faCalendarExclamation, faCalendarMinus, faCalendarPlus, faCalendarStar, faCalendarTimes, faCalendarWeek, faCamera, faCameraAlt, faCameraRetro, faCampfire, faCampground, faCandleHolder, faCandyCane, faCandyCorn, faCannabis, faCapsules, faCar, faCarAlt, faCarBattery, faCarBuilding, faCarBump, faCarBus, faCarCrash, faCarGarage, faCarMechanic, faCarSide, faCarTilt, faCarWash, faCaretCircleDown, faCaretCircleLeft, faCaretCircleRight, faCaretCircleUp, faCaretDown, faCaretLeft, faCaretRight, faCaretSquareDown, faCaretSquareLeft, faCaretSquareRight, faCaretSquareUp, faCaretUp, faCarrot, faCars, faCartArrowDown, faCartPlus, faCashRegister, faCat, faCauldron, faCertificate, faChair, faChairOffice, faChalkboard, faChalkboardTeacher, faChargingStation, faChartArea, faChartBar, faChartLine, faChartLineDown, faChartNetwork, faChartPie, faChartPieAlt, faChartScatter, faCheck, faCheckCircle, faCheckDouble, faCheckSquare, faCheese, faCheeseSwiss, faCheeseburger, faChess, faChessBishop, faChessBishopAlt, faChessBoard, faChessClock, faChessClockAlt, faChessKing, faChessKingAlt, faChessKnight, faChessKnightAlt, faChessPawn, faChessPawnAlt, faChessQueen, faChessQueenAlt, faChessRook, faChessRookAlt, faChevronCircleDown, faChevronCircleLeft, faChevronCircleRight, faChevronCircleUp, faChevronDoubleDown, faChevronDoubleLeft, faChevronDoubleRight, faChevronDoubleUp, faChevronDown, faChevronLeft, faChevronRight, faChevronSquareDown, faChevronSquareLeft, faChevronSquareRight, faChevronSquareUp, faChevronUp, faChild, faChimney, faChurch, faCircle, faCircleNotch, faCity, faClawMarks, faClinicMedical, faClipboard, faClipboardCheck, faClipboardList, faClipboardListCheck, faClipboardPrescription, faClipboardUser, faClock, faClone, faClosedCaptioning, faCloud, faCloudDownload, faCloudDownloadAlt, faCloudDrizzle, faCloudHail, faCloudHailMixed, faCloudMeatball, faCloudMoon, faCloudMoonRain, faCloudRain, faCloudRainbow, faCloudShowers, faCloudShowersHeavy, faCloudSleet, faCloudSnow, faCloudSun, faCloudSunRain, faCloudUpload, faCloudUploadAlt, faClouds, faCloudsMoon, faCloudsSun, faClub, faCocktail, faCode, faCodeBranch, faCodeCommit, faCodeMerge, faCoffee, faCoffeeTogo, faCoffin, faCog, faCogs, faCoin, faCoins, faColumns, faComment, faCommentAlt, faCommentAltCheck, faCommentAltDollar, faCommentAltDots, faCommentAltEdit, faCommentAltExclamation, faCommentAltLines, faCommentAltMedical, faCommentAltMinus, faCommentAltPlus, faCommentAltSlash, faCommentAltSmile, faCommentAltTimes, faCommentCheck, faCommentDollar, faCommentDots, faCommentEdit, faCommentExclamation, faCommentLines, faCommentMedical, faCommentMinus, faCommentPlus, faCommentSlash, faCommentSmile, faCommentTimes, faComments, faCommentsAlt, faCommentsAltDollar, faCommentsDollar, faCompactDisc, faCompass, faCompassSlash, faCompress, faCompressAlt, faCompressArrowsAlt, faCompressWide, faConciergeBell, faConstruction, faContainerStorage, faConveyorBelt, faConveyorBeltAlt, faCookie, faCookieBite, faCopy, faCopyright, faCorn, faCouch, faCow, faCreditCard, faCreditCardBlank, faCreditCardFront, faCricket, faCroissant, faCrop, faCropAlt, faCross, faCrosshairs, faCrow, faCrown, faCrutch, faCrutches, faCube, faCubes, faCurling, faCut, faDagger, faDatabase, faDeaf, faDebug, faDeer, faDeerRudolph, faDemocrat, faDesktop, faDesktopAlt, faDewpoint, faDharmachakra, faDiagnoses, faDiamond, faDice, faDiceD10, faDiceD12, faDiceD20, faDiceD4, faDiceD6, faDiceD8, faDiceFive, faDiceFour, faDiceOne, faDiceSix, faDiceThree, faDiceTwo, faDigging, faDigitalTachograph, faDiploma, faDirections, faDisease, faDivide, faDizzy, faDna, faDoNotEnter, faDog, faDogLeashed, faDollarSign, faDolly, faDollyEmpty, faDollyFlatbed, faDollyFlatbedAlt, faDollyFlatbedEmpty, faDonate, faDoorClosed, faDoorOpen, faDotCircle, faDove, faDownload, faDraftingCompass, faDragon, faDrawCircle, faDrawPolygon, faDrawSquare, faDreidel, faDrone, faDroneAlt, faDrum, faDrumSteelpan, faDrumstick, faDrumstickBite, faDryer, faDryerAlt, faDuck, faDumbbell, faDumpster, faDumpsterFire, faDungeon, faEar, faEarMuffs, faEclipse, faEclipseAlt, faEdit, faEgg, faEggFried, faEject, faElephant, faEllipsisH, faEllipsisHAlt, faEllipsisV, faEllipsisVAlt, faEmptySet, faEngineWarning, faEnvelope, faEnvelopeOpen, faEnvelopeOpenDollar, faEnvelopeOpenText, faEnvelopeSquare, faEquals, faEraser, faEthernet, faEuroSign, faExchange, faExchangeAlt, faExclamation, faExclamationCircle, faExclamationSquare, faExclamationTriangle, faExpand, faExpandAlt, faExpandArrows, faExpandArrowsAlt, faExpandWide, faExternalLink, faExternalLinkAlt, faExternalLinkSquare, faExternalLinkSquareAlt, faEye, faEyeDropper, faEyeEvil, faEyeSlash, faFan, faFarm, faFastBackward, faFastForward, faFax, faFeather, faFeatherAlt, faFemale, faFieldHockey, faFighterJet, faFile, faFileAlt, faFileArchive, faFileAudio, faFileCertificate, faFileChartLine, faFileChartPie, faFileCheck, faFileCode, faFileContract, faFileCsv, faFileDownload, faFileEdit, faFileExcel, faFileExclamation, faFileExport, faFileImage, faFileImport, faFileInvoice, faFileInvoiceDollar, faFileMedical, faFileMedicalAlt, faFileMinus, faFilePdf, faFilePlus, faFilePowerpoint, faFilePrescription, faFileSearch, faFileSignature, faFileSpreadsheet, faFileTimes, faFileUpload, faFileUser, faFileVideo, faFileWord, faFilesMedical, faFill, faFillDrip, faFilm, faFilmAlt, faFilter, faFingerprint, faFire, faFireAlt, faFireExtinguisher, faFireSmoke, faFireplace, faFirstAid, faFish, faFishCooked, faFistRaised, faFlag, faFlagAlt, faFlagCheckered, faFlagUsa, faFlame, faFlask, faFlaskPoison, faFlaskPotion, faFlower, faFlowerDaffodil, faFlowerTulip, faFlushed, faFog, faFolder, faFolderMinus, faFolderOpen, faFolderPlus, faFolderTimes, faFolderTree, faFolders, faFont, faFontAwesomeLogoFull, faFontCase, faFootballBall, faFootballHelmet, faForklift, faForward, faFragile, faFrenchFries, faFrog, faFrostyHead, faFrown, faFrownOpen, faFunction, faFunnelDollar, faFutbol, faGameBoard, faGameBoardAlt, faGamepad, faGasPump, faGasPumpSlash, faGavel, faGem, faGenderless, faGhost, faGift, faGiftCard, faGifts, faGingerbreadMan, faGlass, faGlassChampagne, faGlassCheers, faGlassCitrus, faGlassMartini, faGlassMartiniAlt, faGlassWhiskey, faGlassWhiskeyRocks, faGlasses, faGlassesAlt, faGlobe, faGlobeAfrica, faGlobeAmericas, faGlobeAsia, faGlobeEurope, faGlobeSnow, faGlobeStand, faGolfBall, faGolfClub, faGopuram, faGraduationCap, faGreaterThan, faGreaterThanEqual, faGrimace, faGrin, faGrinAlt, faGrinBeam, faGrinBeamSweat, faGrinHearts, faGrinSquint, faGrinSquintTears, faGrinStars, faGrinTears, faGrinTongue, faGrinTongueSquint, faGrinTongueWink, faGrinWink, faGripHorizontal, faGripLines, faGripLinesVertical, faGripVertical, faGuitar, faHSquare, faH1, faH2, faH3, faH4, faHamburger, faHammer, faHammerWar, faHamsa, faHandHeart, faHandHolding, faHandHoldingBox, faHandHoldingHeart, faHandHoldingMagic, faHandHoldingSeedling, faHandHoldingUsd, faHandHoldingWater, faHandLizard, faHandMiddleFinger, faHandPaper, faHandPeace, faHandPointDown, faHandPointLeft, faHandPointRight, faHandPointUp, faHandPointer, faHandReceiving, faHandRock, faHandScissors, faHandSpock, faHands, faHandsHeart, faHandsHelping, faHandsUsd, faHandshake, faHandshakeAlt, faHanukiah, faHardHat, faHashtag, faHatChef, faHatSanta, faHatWinter, faHatWitch, faHatWizard, faHaykal, faHdd, faHeadSide, faHeadSideBrain, faHeadSideMedical, faHeadVr, faHeading, faHeadphones, faHeadphonesAlt, faHeadset, faHeart, faHeartBroken, faHeartCircle, faHeartRate, faHeartSquare, faHeartbeat, faHelicopter, faHelmetBattle, faHexagon, faHighlighter, faHiking, faHippo, faHistory, faHockeyMask, faHockeyPuck, faHockeySticks, faHollyBerry, faHome, faHomeAlt, faHomeHeart, faHomeLg, faHomeLgAlt, faHoodCloak, faHorizontalRule, faHorse, faHorseHead, faHospital, faHospitalAlt, faHospitalSymbol, faHospitalUser, faHospitals, faHotTub, faHotdog, faHotel, faHourglass, faHourglassEnd, faHourglassHalf, faHourglassStart, faHouseDamage, faHouseFlood, faHryvnia, faHumidity, faHurricane, faICursor, faIceCream, faIceSkate, faIcicles, faIcons, faIconsAlt, faIdBadge, faIdCard, faIdCardAlt, faIgloo, faImage, faImages, faInbox, faInboxIn, faInboxOut, faIndent, faIndustry, faIndustryAlt, faInfinity, faInfo, faInfoCircle, faInfoSquare, faInhaler, faIntegral, faIntersection, faInventory, faIslandTropical, faItalic, faJackOLantern, faJedi, faJoint, faJournalWhills, faKaaba, faKerning, faKey, faKeySkeleton, faKeyboard, faKeynote, faKhanda, faKidneys, faKiss, faKissBeam, faKissWinkHeart, faKite, faKiwiBird, faKnifeKitchen, faLambda, faLamp, faLandmark, faLandmarkAlt, faLanguage, faLaptop, faLaptopCode, faLaptopMedical, faLaugh, faLaughBeam, faLaughSquint, faLaughWink, faLayerGroup, faLayerMinus, faLayerPlus, faLeaf, faLeafHeart, faLeafMaple, faLeafOak, faLemon, faLessThan, faLessThanEqual, faLevelDown, faLevelDownAlt, faLevelUp, faLevelUpAlt, faLifeRing, faLightbulb, faLightbulbDollar, faLightbulbExclamation, faLightbulbOn, faLightbulbSlash, faLightsHoliday, faLineColumns, faLineHeight, faLink, faLips, faLiraSign, faList, faListAlt, faListOl, faListUl, faLocation, faLocationArrow, faLocationCircle, faLocationSlash, faLock, faLockAlt, faLockOpen, faLockOpenAlt, faLongArrowAltDown, faLongArrowAltLeft, faLongArrowAltRight, faLongArrowAltUp, faLongArrowDown, faLongArrowLeft, faLongArrowRight, faLongArrowUp, faLoveseat, faLowVision, faLuchador, faLuggageCart, faLungs, faMace, faMagic, faMagnet, faMailBulk, faMailbox, faMale, faMandolin, faMap, faMapMarked, faMapMarkedAlt, faMapMarker, faMapMarkerAlt, faMapMarkerAltSlash, faMapMarkerCheck, faMapMarkerEdit, faMapMarkerExclamation, faMapMarkerMinus, faMapMarkerPlus, faMapMarkerQuestion, faMapMarkerSlash, faMapMarkerSmile, faMapMarkerTimes, faMapPin, faMapSigns, faMarker, faMars, faMarsDouble, faMarsStroke, faMarsStrokeH, faMarsStrokeV, faMask, faMeat, faMedal, faMedkit, faMegaphone, faMeh, faMehBlank, faMehRollingEyes, faMemory, faMenorah, faMercury, faMeteor, faMicrochip, faMicrophone, faMicrophoneAlt, faMicrophoneAltSlash, faMicrophoneSlash, faMicroscope, faMindShare, faMinus, faMinusCircle, faMinusHexagon, faMinusOctagon, faMinusSquare, faMistletoe, faMitten, faMobile, faMobileAlt, faMobileAndroid, faMobileAndroidAlt, faMoneyBill, faMoneyBillAlt, faMoneyBillWave, faMoneyBillWaveAlt, faMoneyCheck, faMoneyCheckAlt, faMoneyCheckEdit, faMoneyCheckEditAlt, faMonitorHeartRate, faMonkey, faMonument, faMoon, faMoonCloud, faMoonStars, faMortarPestle, faMosque, faMotorcycle, faMountain, faMountains, faMousePointer, faMug, faMugHot, faMugMarshmallows, faMugTea, faMusic, faNarwhal, faNetworkWired, faNeuter, faNewspaper, faNotEqual, faNotesMedical, faObjectGroup, faObjectUngroup, faOctagon, faOilCan, faOilTemp, faOm, faOmega, faOrnament, faOtter, faOutdent, faOverline, faPageBreak, faPager, faPaintBrush, faPaintBrushAlt, faPaintRoller, faPalette, faPallet, faPalletAlt, faPaperPlane, faPaperclip, faParachuteBox, faParagraph, faParagraphRtl, faParking, faParkingCircle, faParkingCircleSlash, faParkingSlash, faPassport, faPastafarianism, faPaste, faPause, faPauseCircle, faPaw, faPawAlt, faPawClaws, faPeace, faPegasus, faPen, faPenAlt, faPenFancy, faPenNib, faPenSquare, faPencil, faPencilAlt, faPencilPaintbrush, faPencilRuler, faPennant, faPeopleCarry, faPepperHot, faPercent, faPercentage, faPersonBooth, faPersonCarry, faPersonDolly, faPersonDollyEmpty, faPersonSign, faPhone, faPhoneAlt, faPhoneLaptop, faPhoneOffice, faPhonePlus, faPhoneSlash, faPhoneSquare, faPhoneSquareAlt, faPhoneVolume, faPhotoVideo, faPi, faPie, faPig, faPiggyBank, faPills, faPizza, faPizzaSlice, faPlaceOfWorship, faPlane, faPlaneAlt, faPlaneArrival, faPlaneDeparture, faPlay, faPlayCircle, faPlug, faPlus, faPlusCircle, faPlusHexagon, faPlusOctagon, faPlusSquare, faPodcast, faPodium, faPodiumStar, faPoll, faPollH, faPollPeople, faPoo, faPooStorm, faPoop, faPopcorn, faPortrait, faPoundSign, faPowerOff, faPray, faPrayingHands, faPrescription, faPrescriptionBottle, faPrescriptionBottleAlt, faPresentation, faPrint, faPrintSearch, faPrintSlash, faProcedures, faProjectDiagram, faPumpkin, faPuzzlePiece, faQrcode, faQuestion, faQuestionCircle, faQuestionSquare, faQuidditch, faQuoteLeft, faQuoteRight, faQuran, faRabbit, faRabbitFast, faRacquet, faRadiation, faRadiationAlt, faRainbow, faRaindrops, faRam, faRampLoading, faRandom, faReceipt, faRectangleLandscape, faRectanglePortrait, faRectangleWide, faRecycle, faRedo, faRedoAlt, faRegistered, faRemoveFormat, faRepeat, faRepeat1, faRepeat1Alt, faRepeatAlt, faReply, faReplyAll, faRepublican, faRestroom, faRetweet, faRetweetAlt, faRibbon, faRing, faRingsWedding, faRoad, faRobot, faRocket, faRoute, faRouteHighway, faRouteInterstate, faRss, faRssSquare, faRubleSign, faRuler, faRulerCombined, faRulerHorizontal, faRulerTriangle, faRulerVertical, faRunning, faRupeeSign, faRv, faSack, faSackDollar, faSadCry, faSadTear, faSalad, faSandwich, faSatellite, faSatelliteDish, faSausage, faSave, faScalpel, faScalpelPath, faScanner, faScannerKeyboard, faScannerTouchscreen, faScarecrow, faScarf, faSchool, faScrewdriver, faScroll, faScrollOld, faScrubber, faScythe, faSdCard, faSearch, faSearchDollar, faSearchLocation, faSearchMinus, faSearchPlus, faSeedling, faSendBack, faSendBackward, faServer, faShapes, faShare, faShareAll, faShareAlt, faShareAltSquare, faShareSquare, faSheep, faShekelSign, faShield, faShieldAlt, faShieldCheck, faShieldCross, faShip, faShippingFast, faShippingTimed, faShishKebab, faShoePrints, faShoppingBag, faShoppingBasket, faShoppingCart, faShovel, faShovelSnow, faShower, faShredder, faShuttleVan, faShuttlecock, faSickle, faSigma, faSign, faSignIn, faSignInAlt, faSignLanguage, faSignOut, faSignOutAlt, faSignal, faSignal1, faSignal2, faSignal3, faSignal4, faSignalAlt, faSignalAlt1, faSignalAlt2, faSignalAlt3, faSignalAltSlash, faSignalSlash, faSignature, faSimCard, faSitemap, faSkating, faSkeleton, faSkiJump, faSkiLift, faSkiing, faSkiingNordic, faSkull, faSkullCrossbones, faSlash, faSledding, faSleigh, faSlidersH, faSlidersHSquare, faSlidersV, faSlidersVSquare, faSmile, faSmileBeam, faSmilePlus, faSmileWink, faSmog, faSmoke, faSmoking, faSmokingBan, faSms, faSnake, faSnooze, faSnowBlowing, faSnowboarding, faSnowflake, faSnowflakes, faSnowman, faSnowmobile, faSnowplow, faSocks, faSolarPanel, faSort, faSortAlphaDown, faSortAlphaDownAlt, faSortAlphaUp, faSortAlphaUpAlt, faSortAlt, faSortAmountDown, faSortAmountDownAlt, faSortAmountUp, faSortAmountUpAlt, faSortDown, faSortNumericDown, faSortNumericDownAlt, faSortNumericUp, faSortNumericUpAlt, faSortShapesDown, faSortShapesDownAlt, faSortShapesUp, faSortShapesUpAlt, faSortSizeDown, faSortSizeDownAlt, faSortSizeUp, faSortSizeUpAlt, faSortUp, faSoup, faSpa, faSpaceShuttle, faSpade, faSparkles, faSpellCheck, faSpider, faSpiderBlackWidow, faSpiderWeb, faSpinner, faSpinnerThird, faSplotch, faSprayCan, faSquare, faSquareFull, faSquareRoot, faSquareRootAlt, faSquirrel, faStaff, faStamp, faStar, faStarAndCrescent, faStarChristmas, faStarExclamation, faStarHalf, faStarHalfAlt, faStarOfDavid, faStarOfLife, faStars, faSteak, faSteeringWheel, faStepBackward, faStepForward, faStethoscope, faStickyNote, faStocking, faStomach, faStop, faStopCircle, faStopwatch, faStore, faStoreAlt, faStream, faStreetView, faStretcher, faStrikethrough, faStroopwafel, faSubscript, faSubway, faSuitcase, faSuitcaseRolling, faSun, faSunCloud, faSunDust, faSunHaze, faSunglasses, faSunrise, faSunset, faSuperscript, faSurprise, faSwatchbook, faSwimmer, faSwimmingPool, faSword, faSwords, faSynagogue, faSync, faSyncAlt, faSyringe, faTable, faTableTennis, faTablet, faTabletAlt, faTabletAndroid, faTabletAndroidAlt, faTabletRugged, faTablets, faTachometer, faTachometerAlt, faTachometerAltAverage, faTachometerAltFast, faTachometerAltFastest, faTachometerAltSlow, faTachometerAltSlowest, faTachometerAverage, faTachometerFast, faTachometerFastest, faTachometerSlow, faTachometerSlowest, faTaco, faTag, faTags, faTally, faTanakh, faTape, faTasks, faTasksAlt, faTaxi, faTeeth, faTeethOpen, faTemperatureFrigid, faTemperatureHigh, faTemperatureHot, faTemperatureLow, faTenge, faTennisBall, faTerminal, faText, faTextHeight, faTextSize, faTextWidth, faTh, faThLarge, faThList, faTheaterMasks, faThermometer, faThermometerEmpty, faThermometerFull, faThermometerHalf, faThermometerQuarter, faThermometerThreeQuarters, faTheta, faThumbsDown, faThumbsUp, faThumbtack, faThunderstorm, faThunderstormMoon, faThunderstormSun, faTicket, faTicketAlt, faTilde, faTimes, faTimesCircle, faTimesHexagon, faTimesOctagon, faTimesSquare, faTint, faTintSlash, faTire, faTireFlat, faTirePressureWarning, faTireRugged, faTired, faToggleOff, faToggleOn, faToilet, faToiletPaper, faToiletPaperAlt, faTombstone, faTombstoneAlt, faToolbox, faTools, faTooth, faToothbrush, faTorah, faToriiGate, faTornado, faTractor, faTrademark, faTrafficCone, faTrafficLight, faTrafficLightGo, faTrafficLightSlow, faTrafficLightStop, faTrain, faTram, faTransgender, faTransgenderAlt, faTrash, faTrashAlt, faTrashRestore, faTrashRestoreAlt, faTrashUndo, faTrashUndoAlt, faTreasureChest, faTree, faTreeAlt, faTreeChristmas, faTreeDecorated, faTreeLarge, faTreePalm, faTrees, faTriangle, faTrophy, faTrophyAlt, faTruck, faTruckContainer, faTruckCouch, faTruckLoading, faTruckMonster, faTruckMoving, faTruckPickup, faTruckPlow, faTruckRamp, faTshirt, faTty, faTurkey, faTurtle, faTv, faTvRetro, faUmbrella, faUmbrellaBeach, faUnderline, faUndo, faUndoAlt, faUnicorn, faUnion, faUniversalAccess, faUniversity, faUnlink, faUnlock, faUnlockAlt, faUpload, faUsdCircle, faUsdSquare, faUser, faUserAlt, faUserAltSlash, faUserAstronaut, faUserChart, faUserCheck, faUserCircle, faUserClock, faUserCog, faUserCrown, faUserEdit, faUserFriends, faUserGraduate, faUserHardHat, faUserHeadset, faUserInjured, faUserLock, faUserMd, faUserMdChat, faUserMinus, faUserNinja, faUserNurse, faUserPlus, faUserSecret, faUserShield, faUserSlash, faUserTag, faUserTie, faUserTimes, faUsers, faUsersClass, faUsersCog, faUsersCrown, faUsersMedical, faUtensilFork, faUtensilKnife, faUtensilSpoon, faUtensils, faUtensilsAlt, faValueAbsolute, faVectorSquare, faVenus, faVenusDouble, faVenusMars, faVial, faVials, faVideo, faVideoPlus, faVideoSlash, faVihara, faVoicemail, faVolcano, faVolleyballBall, faVolume, faVolumeDown, faVolumeMute, faVolumeOff, faVolumeSlash, faVolumeUp, faVoteNay, faVoteYea, faVrCardboard, faWalker, faWalking, faWallet, faWand, faWandMagic, faWarehouse, faWarehouseAlt, faWasher, faWatch, faWatchFitness, faWater, faWaterLower, faWaterRise, faWaveSine, faWaveSquare, faWaveTriangle, faWebcam, faWebcamSlash, faWeight, faWeightHanging, faWhale, faWheat, faWheelchair, faWhistle, faWifi, faWifi1, faWifi2, faWifiSlash, faWind, faWindTurbine, faWindWarning, faWindow, faWindowAlt, faWindowClose, faWindowMaximize, faWindowMinimize, faWindowRestore, faWindsock, faWineBottle, faWineGlass, faWineGlassAlt, faWonSign, faWreath, faWrench, faXRay, faYenSign, faYinYang };
    //     "#;
    //     parse(source).unwrap();
    //   }

    //   #[test]
    //   fn empty_export() {
    //     let source = r#"
    //       export {};
    //     "#;

    //     let analysis = parse(source).unwrap();
    //     assert_eq!(analysis.imports.len(), 0);
    //     assert_eq!(analysis.exports.len(), 0);
    //}
}
