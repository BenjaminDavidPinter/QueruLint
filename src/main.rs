use core::default::Default;
use std::fs::File;
use std::io::Read;

mod violation;
use violation::*;

mod sql_rules;
use sql_rules::*;

mod file_status_flags;
use file_status_flags::*;

#[derive(Debug, Default)]
pub struct ParsedSqlFile {
    tokenized_data: Vec<Vec<String>>,
    lines: u8,
    tokens: u8,
    characters: u8,
}



fn main() {
    for r in std::env::args() {
        match r.ends_with(".sql") {
            true => match File::open(r) {
                Ok(file) => {
                    let parsed_file = file_tokenize(file);
                    println!("{:#?}", parsed_file);
                    let violations = get_file_violations(parsed_file);
                    
                    println!("");
                    for violation in violations {
                        println!("{}\n", violation);
                    }
                },
                _ => Default::default(), //TODO: Actual Error Handling here?
            },
            _ => Default::default(), //TODO: Actual Error Handling here?
        };
    }
}

fn file_tokenize(file_to_tokenize: File) -> ParsedSqlFile {
    let mut file_to_tokenize = file_to_tokenize; //This is mine now.

    println!("Tokenizing...");
    println!("\t{:?}", file_to_tokenize);

    let char_count;

    let mut file_buff: Vec<u8> = Vec::new();
    match file_to_tokenize.read_to_end(&mut file_buff) {
        Ok(bytes_read) => {
            char_count = bytes_read as u8;
            println!("\t{} Bytes Read", bytes_read)
        }
        Err(read_error) => panic!("\t{:?}", read_error),
    }

    let mut document: Vec<Vec<String>> = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut word: Vec<char> = Vec::new();

    let mut token_count = 0;
    let mut line_count = 0;

    for c in file_buff {
        println!("{:#?}", c as char);
        match c {
            32 => {
                //Space
                token_count += 1;
                line.push(word.into_iter().collect());
                word = Vec::new();
            }
            10 => {
                //New Line
                line_count += 1;
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

    ParsedSqlFile {tokenized_data: document,
        lines: line_count,
        tokens: token_count,
        characters: char_count,
    }
}

fn get_file_violations(file_to_review: ParsedSqlFile) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();

    let mut fstat: FileStatusFlags = Default::default();

    let mut line_number = 0;
    let mut token_number;

    for line in file_to_review.tokenized_data {
        line_number += 1;
        token_number = 0;
        fstat.line_comment = false;
        let mut line_copy: Vec<String> = Vec::new();
        line_copy.clone_from(&line);
        
        for word in line {
            token_number += 1;

            fstat.finalize_closing_flags();
            fstat.set_flags(&word);

            //Rules
            if SqlRules::no_select_star(&fstat, &word) { 
                violations.push(Violation {
                    violation_string: String::from("Do not use * in Select Statements"),
                    line: line_number,
                    token_location: token_number,
                    offending_code: line_copy.clone()
                });
            }

            if SqlRules::no_nolock(&fstat, &word){
                violations.push(Violation {
                    violation_string: String::from("Do not use NOLOCK"),
                    line: line_number,
                    token_location: token_number,
                    offending_code: line_copy.clone()
                });
            }
        }
    }
    violations
}
