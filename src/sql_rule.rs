use crate::FileStatusFlags;
use crate::Violation;

pub trait SqlRule {
    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation;
    fn check(&self, fstat: &FileStatusFlags, current_token: &str) -> bool;
}