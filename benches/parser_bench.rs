extern crate sql_parser;

use criterion::{criterion_group, criterion_main, Criterion};

fn no_select_star() {
    let _violations = sql_parser::lint_files(&["noselectstar.sql"]);
}
pub fn bench_no_select_star(c: &mut Criterion) {
    c.bench_function("No SELECT *", |b| b.iter(no_select_star));
}

fn no_open_trans_at_end_of_file() {
    let _violations = sql_parser::lint_files(&["noopentran.sql"]);
}
pub fn bench_no_open_trans_at_end_of_file(c: &mut Criterion) {
    c.bench_function("Unclosed TRAN", |b| b.iter(no_open_trans_at_end_of_file));
}

fn no_nolock() {
    let _violations = sql_parser::lint_files(&["nonolock.sql"]);
}
pub fn bench_no_nolock(c: &mut Criterion) {
    c.bench_function("No NOLOCK", |b| b.iter(no_nolock));
}

fn no_select_in_tran() {
    let _violations = sql_parser::lint_files(&["noselectintran.sql"]);
}
pub fn bench_no_select_in_tran(c: &mut Criterion) {
    c.bench_function("No SELECT in TRAN", |b| b.iter(no_select_in_tran));
}

fn no_funcs_in_where_clause() {
    let _violations = sql_parser::lint_files(&["nofunctionsinwhere.sql"]);
}
pub fn bench_no_funcs_in_where_clause(c: &mut Criterion) {
    c.bench_function("No f() in WHERE", |b| b.iter(no_funcs_in_where_clause));
}

fn no_declare_in_tran() {
    let _violations = sql_parser::lint_files(&["nodeclareintran.sql"]);
}
pub fn bench_no_declare_in_tran(c: &mut Criterion) {
    c.bench_function("No DECLARE in TRAN", |b| b.iter(no_declare_in_tran));
}



criterion_group!(benches, 
    bench_no_select_star, 
    bench_no_open_trans_at_end_of_file,
    bench_no_nolock,
    bench_no_select_in_tran,
    bench_no_funcs_in_where_clause,
    bench_no_declare_in_tran);
criterion_main!(benches);
