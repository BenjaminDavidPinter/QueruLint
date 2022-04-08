#[derive(Debug, Default)]
pub struct QueruParser{
    pub flags: FileStateflags,
    pub vars: Vec<Variable>
}

#[derive(Debug, Default)]
pub struct Variable {
    pub variable_name: String,
    pub variable_type: String,
    pub initial_value: String
}
impl Variable{
    pub fn new(name: String) -> Variable{
        Variable { variable_name: name, 
            variable_type: String::new(),
            initial_value: String::new() }
    }
}

#[derive(Debug, Default)]
pub struct FileStateflags {
    pub line_comment: bool,
    pub block_comment: bool,
    pub closing_block_comment: bool,
    pub closing_select: bool,
    pub select: bool,
    pub begin: bool,
    pub end: bool,
    pub in_transaction : bool,
    pub declare: bool,
    pub check_datatype: bool,
    pub check_var_initial_value: bool,
    pub where_clause: bool,
    pub where_clause_left_assignment: bool,
    pub where_clause_operand: bool,
    pub where_clause_right_assignment: bool,
}

impl QueruParser{
    pub fn in_comment(&self) -> bool {
        return self.flags.line_comment || self.flags.block_comment || self.flags.closing_block_comment;
    }

    fn clean_str(string_to_clean: &str) -> String{
        return string_to_clean
        .replacen(';', "", 1)
        .replacen("'", "", 2);
    }

    pub fn finalize_closing_flags(&mut self) {
        //Finish block comment status bit flips
        if self.flags.closing_block_comment {
            self.flags.closing_block_comment = false;
            self.flags.block_comment = false;
        }

        //Finish select status bit flips
        if self.flags.closing_select {
            self.flags.closing_select = false;
            self.flags.select = false;
        }
    }

    pub fn close_statement_flags(&mut self){
        self.flags.declare = false;
        self.flags.check_datatype = false;
        self.flags.check_var_initial_value = false;
        self.flags.select = false;
    }

    pub fn set_flags(&mut self, word: &str) {
        if word.starts_with("--") {
            self.flags.line_comment = true;
        }
        if word.contains(";") {
            self.flags.closing_select = true;
            self.flags.declare = false;
        }

        match word.to_uppercase().as_str() {
            "--" => {
                self.flags.line_comment = true;
                self.flags.closing_select = true;
            }
            "/*" => {
                self.flags.block_comment = true;
            }
            "*/" => {
                self.flags.closing_block_comment = true;
            }
            "SELECT" => {
                self.flags.where_clause = false;
                if !self.in_comment() {
                    self.flags.select = true;
                }
            }
            ";" => {
                self.flags.where_clause = false;
                self.flags.closing_select = true;
                if self.flags.check_var_initial_value {
                    self.vars.last_mut().unwrap().initial_value = String::new();
                    self.flags.check_var_initial_value = false;
                }
            }
            "GO" => {
                self.flags.where_clause = false;
                self.flags.closing_select = true;
                self.close_statement_flags();
            },
            "BEGIN" => {
                self.flags.begin = true;
                self.flags.where_clause = false;
            },
            "TRAN" | "TRANSACTION" => {
                self.flags.where_clause = false;
                if self.flags.begin {
                    self.flags.begin = false;
                    self.flags.in_transaction = true;
                }
                else if self.flags.end {
                    self.flags.end = false;
                    self.flags.in_transaction = false;
                }
            },
            "END" => {
                self.flags.where_clause = false;
                self.flags.end = true;
                self.close_statement_flags();
            },
            "DECLARE" => {
                self.flags.where_clause = false;
                self.flags.select = false;
                self.flags.declare = true;
            },
            "=" => {
                //Capture the step over '=' so we can get the value below
            },
            "WHERE" | "OR" | "AND" => {
                self.flags.where_clause = true;
            }
            &_ => {
                //Implement in reverse precedent; Initial Value -> Type -> Name etc
                //This is specifically for variable declarations
                if self.flags.check_var_initial_value {
                    self.vars.last_mut().unwrap().initial_value = QueruParser::clean_str(word);
                    self.flags.check_var_initial_value = false;
                }
                if self.flags.check_datatype  {
                    self.vars.last_mut().unwrap().variable_type = QueruParser::clean_str(word);
                    self.flags.check_datatype = false;
                    self.flags.check_var_initial_value = true;
                }
                if self.flags.declare {
                    self.vars.push(
                        Variable::new(QueruParser::clean_str(word))
                    ); //Just keep a copy of the possible variable name for later
                    self.flags.declare = false;
                    self.flags.check_datatype = true;
                }

                //Just update the statuses on the where clause to track location
                if self.flags.where_clause {
                    println!("where_clause; {}", word);
                    self.flags.where_clause = false;
                    self.flags.where_clause_left_assignment = true;
                } else if self.flags.where_clause_left_assignment {
                    println!("where_clause_left_assignments; {}", word);
                    self.flags.where_clause_left_assignment = false;
                    self.flags.where_clause_operand = true;
                } else if self.flags.where_clause_operand {
                    println!("where_clause_operand; {}", word);
                    self.flags.where_clause_operand = false;
                    self.flags.where_clause_right_assignment = true;
                } else if self.flags.where_clause_right_assignment {
                    println!("where_clause_right_assignment; {}", word);
                    self.flags.where_clause_right_assignment = false;
                }
            } //Leave this here as we implement the entire sql language
        }
    }
}