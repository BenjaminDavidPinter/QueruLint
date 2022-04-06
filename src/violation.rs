use std::fmt;

#[derive(Debug)]
pub struct Violation {
    pub line : u8,
    pub token_location: u8,
    pub violation_string: String,
    pub offending_code: Vec<String>
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Line {}, Token {}: {}\n{:?}", 
        self.line, self.token_location, self.violation_string, self.offending_code.join(" "))
    }
}