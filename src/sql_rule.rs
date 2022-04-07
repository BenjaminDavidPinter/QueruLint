use crate::QueruParser;
use crate::Violation;
use crate::queru_parser::FileStateflags;

pub trait SqlRule {
    fn get_violation(&self, line:u8, token_location:u8, offending_code: Vec<String>) -> Violation;
    fn check(&self, fstat: &FileStateflags, current_token: &str) -> bool;
}