//! Generates large snippets of Rust code for usage in the benchmarks.

use std::fs;

use stdx::format_to;

use crate::project_dir;

pub fn big_struct() -> String {
    let n = 1_000;

    let mut buf = "pub struct RegisterBlock {".to_string();
    for i in 0..n {
        format_to!(buf, "  /// Doc comment for {}.\n", i);
        format_to!(buf, "  pub s{}: S{},\n", i, i);
    }
    buf.push_str("}\n\n");
    for i in 0..n {
        format_to!(
            buf,
            "

#[repr(transparent)]
struct S{} {{
    field: u32,
}}",
            i
        );
    }

    buf
}

pub fn glorious_old_parser() -> String {
    let path = project_dir().join("bench_data/glorious_old_parser");
    fs::read_to_string(&path).unwrap()
}