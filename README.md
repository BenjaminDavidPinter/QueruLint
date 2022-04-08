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

I have no idea how to build a parser, but right now, it's flag based.
The flags help me figure out what state I'm in.

The entire parser/compiler is serialized to json and pasted into the output for debugging purposes
```
QueruParser {
    flags: FileStateflags {
        line_comment: false,
        block_comment: false,
        closing_block_comment: false,
        closing_select: false,
        select: false,
        begin: false,
        end: false,
        in_transaction: false,
        declare: false,
        check_datatype: false,
        check_var_initial_value: false,
        where_clause: false,
        where_clause_left_assignment: false,
        where_clause_operand: false,
        where_clause_right_assignment: false,
    },
    vars: [
        Variable {
            variable_name: "@datetime",
            variable_type: "datetime",
            initial_value: "1/1/2021",
        },
        Variable {
            variable_name: "@whatever",
            variable_type: "datetime",
            initial_value: "2/4/2021",
        },
        Variable {
            variable_name: "@badidea",
            variable_type: "int",
            initial_value: "0",
        },
        Variable {
            variable_name: "@anotherBadIdea",
            variable_type: "int",
            initial_value: "",
        },
    ],
}

Line 5, Token 1: Do not run select statements in transaction
"SELECT * FROM DBO.TABLE AS T1"

Line 5, Token 2: Do not use * in select list, specify columns
"SELECT * FROM DBO.TABLE AS T1"

Line 6, Token 7: Do not use NOLOCK
"    INNER JOIN DBO.TABLE2 AS T2 WITH (NOLOCK)"

Line 8, Token 4: Do not use functions in where clauses, cache functions as variables first
"where T1.createddt > GetDate()"

Line 9, Token 2: Do not use functions in where clauses, cache functions as variables first
"or somefunc() = 'ur mum'"

Line 11, Token 1: Do not declare variables in transaction
"declare @badidea int = 0;"

Line 13, Token 1: Do not declare variables in transaction
"declare @anotherBadIdea int"

```