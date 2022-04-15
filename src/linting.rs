pub mod sql_linting {
    use crate::FileStateflags;
    use std::fmt;

    #[derive(Debug)]
    pub struct Violation {
        pub line: u8,
        pub token_location: u8,
        pub violation_string: String,
        pub offending_code: Vec<String>,
    }
    impl Violation {
        pub fn new(
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
            violation_message: String
        ) -> Violation {
            Violation {
                line,
                token_location,
                violation_string: violation_message,
                offending_code,
            }
        }
    }
    impl fmt::Display for Violation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Line {}, Token {}: {}\n{:?}",
                self.line,
                self.token_location,
                self.violation_string,
                self.offending_code.join(" ")
            )
        }
    }

    pub struct Rules {}
    impl Rules {
        pub fn no_select_star(fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.select && current_token == "*" {
                return true;
            }
            false
        }

        pub fn no_function_in_where(fstat: &FileStateflags, current_token: &str) -> bool {
            if (fstat.where_clause
                || fstat.where_clause_left_assignment
                || fstat.where_clause_operand
                || fstat.where_clause_right_assignment)
                && current_token.ends_with(')')
            {
                return true;
            }
            false
        }

        pub fn no_nolock(fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.select && (current_token.to_uppercase() == "(NOLOCK)" || current_token.to_uppercase() == "NOLOCK") {
                return true;
            }
            false
        }

        pub fn left_tran_open(fstat: &FileStateflags, _current_token: &str) -> bool {
            if fstat.in_transaction {
                return true;
            }
            false
        }

        pub fn left_tran_open_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line,
                token_location,
                violation_string: String::from("Transaction left open at end of file"),
                offending_code,
            }
        }

        pub fn no_select_in_tran(fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.in_transaction && fstat.select && current_token.to_uppercase() == "SELECT" {
                return true;
            }
            false
        }

        pub fn no_delcare_in_tran(fstat: &FileStateflags, _current_token: &str) -> bool {
            if fstat.in_transaction && fstat.declare {
                return true;
            }
            false
        }
    }
}
