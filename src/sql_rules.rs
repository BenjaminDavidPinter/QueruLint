use std::thread::current;

use crate::{SqlRule, violation::Violation};

pub struct NoSelectStar {}
impl SqlRule for NoSelectStar {
    fn check(&self, fstat: &crate::file_status_flags::FileStatusFlags, current_token: &str) -> bool {
        if fstat.select && current_token == "*" {
            return true;
        }
        return false;
    }
    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation {
        Violation { 
            line: line, 
            token_location: token_location, 
            violation_string: String::from("Do not use * in select list, specify columns"), 
            offending_code: offending_code 
        }
    }
}

pub struct NoNoLock {}
impl SqlRule for NoNoLock {
    fn check(&self, fstat: &crate::file_status_flags::FileStatusFlags, current_token: &str) -> bool {
        if fstat.select && (current_token == "(NOLOCK)" || current_token == "NOLOCK") {
            return true;
        }
        return false;
    }

    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation {
        Violation { 
            line: line, 
            token_location: token_location, 
            violation_string: String::from("Do not use NOLOCK"), 
            offending_code: offending_code 
        }
    }
}

pub struct LeftOpenTran {}
impl SqlRule for LeftOpenTran {
    fn check(&self ,fstat: &crate::file_status_flags::FileStatusFlags, current_token: &str) -> bool {
        if fstat.in_transaction {
            return true;
        }
        return false;
    }

    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation {
        Violation { 
            line: line, 
            token_location: token_location, 
            violation_string: String::from("Transaction left open at end of file"), 
            offending_code: offending_code 
        }
    }
}

pub struct NoSelectInTran {}
impl SqlRule for NoSelectInTran {
    fn check(&self ,fstat: &crate::file_status_flags::FileStatusFlags, current_token: &str) -> bool {
        if fstat.in_transaction && fstat.select && current_token == "SELECT" {
            return true;
        }
        return false;
    }

    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation {
        Violation { 
            line: line, 
            token_location: token_location, 
            violation_string: String::from("Do not run select statements in transaction"), 
            offending_code: offending_code 
        }
    }
}