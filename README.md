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

Output supports line/token location & offending code
```
Line 2, Token 2: Do not use * in select list, specify columns
"SELECT * FROM DBO.TABLE AS T1"

Line 3, Token 7: Do not use NOLOCK
"    INNER JOIN DBO.TABLE2 AS T2 WITH (NOLOCK)"
```