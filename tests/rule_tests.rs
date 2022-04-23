extern crate sql_parser;

#[cfg(test)]
mod lints {
    #[test]
    fn no_select_star() {
        let violations = sql_parser::lint_files(&["noselectstar.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Do not use * in select list, specify columns")
    }

    #[test]
    fn no_open_trans_at_end_of_file() {
        let violations = sql_parser::lint_files(&["noopentran.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Transaction left open at end of file")
    }

    #[test]
    fn no_nolock() {
        let violations = sql_parser::lint_files(&["nonolock.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Do not use NOLOCK")
    }

    #[test]
    fn no_select_in_tran() {
        let violations = sql_parser::lint_files(&["noselectintran.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Do not run select statements in transaction")
    }

    #[test]
    fn no_funcs_in_where_clause() {
        let violations = sql_parser::lint_files(&["nofunctionsinwhere.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(
            violations[0].violation_string
                == "Do not use functions in where clauses, cache functions as variables first"
        )
    }

    #[test]
    fn no_declare_in_tran() {
        let violations = sql_parser::lint_files(&["nodeclareintran.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Do not declare variables in transaction")
    }

    #[test]
    fn no_cursors() {
        let violations = sql_parser::lint_files(&["nocursors.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(
            violations[0].violation_string
                == "Do not use CURSORS, prefer while loops with counters"
        );
    }

    #[test]
    fn must_qualify_tables() {
        let violations = sql_parser::lint_files(&["fullyqualifytables.sql"]);
        println!("{:#?}", violations);
        assert!(violations.len() == 1);
        assert!(violations[0].violation_string == "Fully qualify tables");
    }
}
