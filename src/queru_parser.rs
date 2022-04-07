#[derive(Debug, Default)]
pub struct QueruParser{
    pub flags: FileStateflags,
    pub vars: Vec<String>
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
    pub declare: bool
}

impl QueruParser{
    pub fn in_comment(&self) -> bool {
        return self.flags.line_comment || self.flags.block_comment || self.flags.closing_block_comment;
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
                if !self.in_comment() {
                    self.flags.select = true;
                }
            }
            ";" => {
                self.flags.closing_select = true;
            }
            "GO" => {
                self.flags.closing_select = true;
            },
            "BEGIN" => {
                self.flags.begin = true;
            },
            "TRAN" | "TRANSACTION" => {
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
                self.flags.end = true;
            },
            "DECLARE" => {
                self.flags.select = false;
                self.flags.declare = true;
            }
            &_ => {
                if self.flags.declare {
                    self.vars.push(String::from(word)); //Just keep a copy of the possible variable name for later
                    self.flags.declare = false;
                }  
            } //Leave this here as we implement the entire sql language
        }
    }
}