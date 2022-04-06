Implement rules by trait, then consume the rules in the relevant section of the file crawler.

```Rust
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
```

