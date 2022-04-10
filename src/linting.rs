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

    pub trait SqlRule {
        fn get_violation(
            &self,
            line: u8,
            toke_location: u8,
            offending_code: Vec<String>,
        ) -> Violation;
        fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool;
    }

    pub struct NoSelectStar {}
    impl SqlRule for NoSelectStar {
        fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.select && current_token == "*" {
                return true;
            }
            false
        }
        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from("Do not use * in select list, specify columns"),
                offending_code: offending_code,
            }
        }
    }

    pub struct NoFunctionsInWhere {}
    impl SqlRule for NoFunctionsInWhere {
        fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool {
            if (fstat.where_clause
                || fstat.where_clause_left_assignment
                || fstat.where_clause_operand
                || fstat.where_clause_right_assignment)
                && current_token.ends_with(")")
            {
                return true;
            }
            false
        }
        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from(
                    "Do not use functions in where clauses, cache functions as variables first",
                ),
                offending_code: offending_code,
            }
        }
    }

    pub struct NoNoLock {}
    impl SqlRule for NoNoLock {
        fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.select && (current_token.to_uppercase() == "(NOLOCK)" || current_token.to_uppercase() == "NOLOCK") {
                return true;
            }
            false
        }

        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from("Do not use NOLOCK"),
                offending_code: offending_code,
            }
        }
    }

    pub struct LeftOpenTran {}
    impl SqlRule for LeftOpenTran {
        fn check(&self, fstat: &FileStateflags, _current_token: &str) -> bool {
            if fstat.in_transaction {
                return true;
            }
            false
        }

        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from("Transaction left open at end of file"),
                offending_code: offending_code,
            }
        }
    }

    pub struct NoSelectInTran {}
    impl SqlRule for NoSelectInTran {
        fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool {
            if fstat.in_transaction && fstat.select && current_token.to_uppercase() == "SELECT" {
                return true;
            }
            false
        }

        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from("Do not run select statements in transaction"),
                offending_code: offending_code,
            }
        }
    }

    pub struct NoDeclareInTran {}
    impl SqlRule for NoDeclareInTran {
        fn check(&self, fstat: &FileStateflags, _current_token: &str) -> bool {
            if fstat.in_transaction && fstat.declare {
                return true;
            }
            false
        }

        fn get_violation(
            &self,
            line: u8,
            token_location: u8,
            offending_code: Vec<String>,
        ) -> Violation {
            Violation {
                line: line,
                token_location: token_location,
                violation_string: String::from("Do not declare variables in transaction"),
                offending_code: offending_code,
            }
        }
    }
}
