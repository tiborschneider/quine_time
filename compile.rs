use std::env;
use std::io::prelude::*;

fn main() {
    let name = env::args().skip(1).next().unwrap();
    let width = env::args()
        .skip(2)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let input = format!("{name}.rs");
    let output = format!("{name}_quine.rs");

    let prog = std::fs::read_to_string(input).unwrap();
    let prog = format_prog(&prog, width);
    let prog = quine_prog(&prog);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .unwrap();
    file.write_all(prog.as_bytes()).unwrap();
}

fn special(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '!' | '\'')
}

fn format_prog(prog: &str, max_width: usize) -> String {
    let mut prev_char = ';';
    let mut out = String::new();
    let mut width = 0;
    for part in prog.split_whitespace() {
        if (width > 0 && width + part.len() > max_width) || part == "\"?\";" {
            out.push('\n');
            width = 0;
            prev_char = ';';
        }
        // check if we need to put a space before
        let first_char = part.chars().next().unwrap();
        let last_char = part.chars().last().unwrap();

        if special(prev_char) && special(first_char) {
            out.push(' ');
            width += 1;
        }
        out.push_str(part);
        width += part.len();
        prev_char = last_char;
    }
    out
}

fn quine_prog(prog: &str) -> String {
    let escaped_prog = prog.replace('\\', "\\\\");
    let escaped_prog = escaped_prog.as_str().replace('"', "\\\"");
    prog.replacen('?', &escaped_prog.trim(), 1)
}
