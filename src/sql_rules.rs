use crate::FileStatusFlags;

pub struct SqlRules {
}

impl SqlRules {
    pub fn no_select_star(fstat: &FileStatusFlags, current_token: &str) -> bool{
        if fstat.select && current_token == "*"{
            return true;
        }else {
            return false;
        }
    }

    pub fn no_nolock(fstat: &FileStatusFlags, current_token: &str) -> bool{
        if current_token == "NOLOCK" || current_token == "(NOLOCK)" {
            return true;
        }else {
            return false;
        }
    }
}