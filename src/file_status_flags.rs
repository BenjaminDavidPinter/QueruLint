#[derive(Debug, Default)]
pub struct FileStatusFlags {
    pub line_comment: bool,
    pub block_comment: bool,
    pub closing_block_comment: bool,
    pub closing_select: bool,
    pub select: bool,
    pub begin: bool,
    pub end: bool,
    pub in_transaction : bool
}

impl FileStatusFlags {
    pub fn in_comment(&self) -> bool {
        return self.line_comment || self.block_comment || self.closing_block_comment;
    }

    pub fn finalize_closing_flags(&mut self) {
        //Finish block comment status bit flips
        if self.closing_block_comment {
            self.closing_block_comment = false;
            self.block_comment = false;
        }

        //Finish select status bit flips
        if self.closing_select {
            self.closing_select = false;
            self.select = false;
        }
    }

    pub fn set_flags(&mut self, word: &str) {
        if word.starts_with("--") {
            self.line_comment = true;
        }
        if word.contains(";") {
            self.closing_select = true;
        }

        match word.to_uppercase().as_str() {
            "--" => {
                self.line_comment = true;
                self.closing_select = true;
            }
            "/*" => {
                self.block_comment = true;
            }
            "*/" => {
                self.closing_block_comment = true;
            }
            "SELECT" => {
                if !self.in_comment() {
                    self.select = true;
                }
            }
            ";" => {
                self.closing_select = true;
            }
            "GO" => {
                self.closing_select = true;
            },
            "BEGIN" => {
                self.begin = true;
            },
            "TRAN" | "TRANSACTION" => {
                if self.begin {
                    self.begin = false;
                    self.in_transaction = true;
                }
                else if self.end {
                    self.end = false;
                    self.in_transaction = false;
                }
            },
            "END" => {
                self.end = true;
            }
            &_ => {} //Leave this here as we implement the entire sql language
        }
    }
}