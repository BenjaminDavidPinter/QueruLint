mod parsing;
mod linting;
pub use crate::parsing::sql_parsing::*;
pub use crate::linting::sql_linting::*;
use std::fs::File;
use std::io::Read;

pub fn lint_files(args: &[&str]) -> Vec<Violation>{
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

    //Rules for token-by-token parsing
    let token_rules: Vec<Box<dyn SqlRule>> = vec![
        Box::new(NoNoLock {}),
        Box::new(NoSelectStar {}),
        Box::new(NoSelectInTran {}),
        Box::new(NoDeclareInTran {}),
        Box::new(NoFunctionsInWhere {}),
    ];

    //Rules for end of file checks
    let post_rules: Vec<Box<dyn SqlRule>> = vec![Box::new(LeftOpenTran {})];

    let mut parser: QueruParser = Default::default();

    let mut line_number = 0;
    let mut token_number;

    for line in file_to_review.tokenized_data {
        line_number += 1;
        token_number = 0;

        parser.flags.line_comment = false;
        let mut line_copy: Vec<String> = Vec::new();
        line_copy.clone_from(&line);

        //State check on a per word basis
        for word in line {
            if word.is_empty() {
                continue;
            }
            token_number += 1;

            parser.finalize_closing_flags();
            parser.set_flags(&word);

            for rule in &token_rules {
                if rule.check(&parser.flags, &word) {
                    violations.push(rule.get_violation(
                        line_number,
                        token_number,
                        line_copy.clone(),
                    ))
                }
            }
        }
    }

    for rule in &post_rules {
        if rule.check(&parser.flags, "") {
            violations.push(rule.get_violation(0, 0, Vec::new()))
        }
    }
    violations
}
