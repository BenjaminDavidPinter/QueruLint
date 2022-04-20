pub mod sql_parsing {
    #[derive(Debug, Default)]
    pub struct QueruParser {
    }

    #[derive(Debug, Default)]
    pub struct ParsedSqlFile {
        pub tokenized_data: Vec<Vec<String>>,
    }

    #[derive(Debug, Default)]
    pub struct Variable {
        pub variable_name: String,
        pub variable_type: String,
        pub initial_value: String,
    }
    impl Variable {
        pub fn new(name: String) -> Variable {
            Variable {
                variable_name: name,
                variable_type: String::new(),
                initial_value: String::new(),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct FileState {
        pub line_comment: bool,
        pub block_comment: bool,
        pub closing_block_comment: bool,
        pub closing_select: bool,
        pub select: bool,
        pub begin: bool,
        pub end: bool,
        pub in_transaction: bool,
        pub declare: bool,
        pub check_datatype: bool,
        pub check_var_initial_value: bool,
        pub where_clause: bool,
        pub where_clause_left_assignment: bool,
        pub where_clause_operand: bool,
        pub where_clause_right_assignment: bool,
        pub commit: bool,
        pub vars: Vec<Variable>
    }
    impl FileState{

        //Do I really need to do this?
        pub fn new() -> FileState {
            Default::default()
        }

        pub fn in_comment(flags: &FileState) -> bool {
            flags.line_comment || flags.block_comment || flags.closing_block_comment
        }

        pub fn finalize_closing_flags(flags: FileState) -> FileState { 
            let mut new_flags = FileState { ..flags};

            if flags.closing_block_comment {
                new_flags.closing_block_comment = false;
                new_flags.block_comment = false;
            }

            if flags.closing_select {
                new_flags.closing_select = false;
                new_flags.select = false;
            }

            new_flags

        }

        pub fn close_statement_flags(flags: FileState) -> FileState {
            FileState {
                declare : false,
                check_datatype : false,
                check_var_initial_value : false,
                select : false,
                ..flags
            }
        }

    }

    impl QueruParser {

        fn clean_str(string_to_clean: &str) -> String {
            string_to_clean.replacen(';', "", 1).replacen('\'', "", 2)
        }

        
        pub fn interpret(current_flags: FileState, word: &str) -> FileState {
            let mut current_flags = current_flags;

            if word.starts_with("--") {
                current_flags.line_comment = true;
            }
            if word.contains(';') {
                current_flags.closing_select = true;
                current_flags.declare = false;
            }

            match word.to_uppercase().as_str() {
                "--" => {
                    current_flags.line_comment = true;
                    current_flags.closing_select = true;
                }
                "/*" => {
                    current_flags.block_comment = true;
                }
                "*/" => {
                    current_flags.closing_block_comment = true;
                }
                "SELECT" => {
                    current_flags.where_clause = false;
                    if !FileState::in_comment(&current_flags) {
                        current_flags.select = true;
                    }
                }
                ";" => {
                    current_flags.where_clause = false;
                    current_flags.closing_select = true;
                    if current_flags.check_var_initial_value {
                        current_flags.vars.last_mut().unwrap().initial_value = String::new();
                        current_flags.check_var_initial_value = false;
                    }
                }
                "GO" => {
                    current_flags.where_clause = false;
                    current_flags.closing_select = true;
                    current_flags = FileState::close_statement_flags(current_flags);
                }
                "BEGIN" => {
                    current_flags.begin = true;
                    current_flags.where_clause = false;
                }
                "TRAN" | "TRANSACTION" => {
                    current_flags.where_clause = false;
                    if current_flags.begin {
                        current_flags.begin = false;
                        current_flags.in_transaction = true;
                    } else if current_flags.commit {
                        current_flags.commit = false;
                        current_flags.in_transaction = false;
                    }
                }
                "END" => {
                    current_flags.where_clause = false;
                    current_flags.end = true;
                    current_flags = FileState::close_statement_flags(current_flags);
                },
                "COMMIT" =>{
                    current_flags.commit = true;
                }
                "DECLARE" => {
                    current_flags.where_clause = false;
                    current_flags.select = false;
                    current_flags.declare = true;
                }
                "=" => {
                    //Capture the step over '=' so we can get the value below
                }
                "WHERE" | "OR" | "AND" => {
                    current_flags.where_clause = true;
                }
                &_ => {
                    //Implement in reverse precedent; Initial Value -> Type -> Name etc
                    //This is specifically for variable declarations
                    if current_flags.check_var_initial_value {
                        current_flags.vars.last_mut().unwrap().initial_value = QueruParser::clean_str(word);
                        current_flags.check_var_initial_value = false;
                    }
                    if current_flags.check_datatype {
                        current_flags.vars.last_mut().unwrap().variable_type = QueruParser::clean_str(word);
                        current_flags.check_datatype = false;
                        current_flags.check_var_initial_value = true;
                    }
                    if current_flags.declare {
                        current_flags.vars.push(Variable::new(QueruParser::clean_str(word))); //Just keep a copy of the possible variable name for later
                        current_flags.declare = false;
                        current_flags.check_datatype = true;
                    }

                    //Just update the statuses on the where clause to track location
                    if current_flags.where_clause {
                        current_flags.where_clause = false;
                        current_flags.where_clause_left_assignment = true;
                    } else if current_flags.where_clause_left_assignment {
                        current_flags.where_clause_left_assignment = false;
                        current_flags.where_clause_operand = true;
                    } else if current_flags.where_clause_operand {
                        current_flags.where_clause_operand = false;
                        current_flags.where_clause_right_assignment = true;
                    } else if current_flags.where_clause_right_assignment {
                        current_flags.where_clause_right_assignment = false;
                    }
                } //Leave this here as we implement the entire sql language

            }
            current_flags
        }
    }
}
