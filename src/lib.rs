mod linting;
mod parsing;
pub use crate::linting::sql_linting::*;
pub use crate::parsing::sql_parsing::*;
use std::fs::File;
use std::io::Read;

pub fn lint_files(args: &[&str]) -> Vec<Violation> {
    for r in args {
        match r.ends_with(".sql") {
            true => match File::open(r) {
                Ok(file) => {
                    let parsed_file = file_tokenize(file);

                    //Is it safe to return from within here? Rather than return the value after we've escaped the match statement?
                    return review_file(parsed_file);
                }
                _ => Default::default(), //TODO: Actual Error Handling here?
            },
            _ => Default::default(), //TODO: Actual Error Handling here?
        };
    }
    Vec::new()
}

fn file_tokenize(file_to_tokenize: File) -> ParsedSqlFile {
    let mut file_to_tokenize = file_to_tokenize; //This is mine now.

    let mut file_buff: Vec<u8> = Vec::new();
    match file_to_tokenize.read_to_end(&mut file_buff) {
        Ok(_) => {}
        Err(read_error) => panic!("\t{:?}", read_error),
    }

    let mut document: Vec<Vec<String>> = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut word: Vec<char> = Vec::new();

    for c in file_buff {
        match c {
            32 => {
                //Space
                line.push(word.into_iter().collect());
                word = Vec::new();
            }
            10 => {
                //New Line
                line.push(word.into_iter().collect());
                document.push(line);
                line = Vec::new();
                word = Vec::new();
            }
            _ => {
                word.push(c as char);
            }
        }
    }
    line.push(word.into_iter().collect());
    document.push(line);

    ParsedSqlFile {
        tokenized_data: document,
    }
}

fn review_file(file_to_review: ParsedSqlFile) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();
    let mut state = FileState::new();

    let mut token_number;

    for (line_number, line) in file_to_review.tokenized_data.into_iter().enumerate() {
        token_number = 0;

        state.line_comment = false;
        let mut line_copy: Vec<String> = Vec::new();
        line_copy.clone_from(&line);

        //State check on a per word basis
        for word in line {
            if word.is_empty() {
                continue;
            } else {
                token_number += 1;
            }

            FileState::finalize_closing_flags(&mut state);
            QueruParser::interpret(&mut state, &word);

            if Rules::no_cursors(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Do not use CURSORS, prefer while loops with counters"),
                ));
            }
            if Rules::must_qualify_tables(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Fully qualify tables"),
                ));
            }
            if Rules::no_select_star(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Do not use * in select list, specify columns"),
                ));
            }
            if Rules::no_delcare_in_tran(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Do not declare variables in transaction"),
                ));
            }
            if Rules::no_function_in_where(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from(
                        "Do not use functions in where clauses, cache functions as variables first",
                    ),
                ));
            }
            if Rules::no_nolock(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Do not use NOLOCK"),
                ));
            }
            if Rules::no_select_in_tran(&state, &word) {
                violations.push(Violation::new(
                    line_number.try_into().unwrap(),
                    token_number,
                    line_copy.clone(),
                    String::from("Do not run select statements in transaction"),
                ));
            }
        }
    }
    if Rules::left_tran_open(&state, "") {
        violations.push(Violation::new(
            0,
            0,
            Vec::new(),
            String::from("Transaction left open at end of file"),
        ));
    }
    violations
}
