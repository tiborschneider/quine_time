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
    for part in prog
        .split_whitespace()
        .flat_map(|p| Splitter::new(p))
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
    {
        if (width > 0 && width + part.len() > max_width) || part == "\"?\"" {
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

struct Splitter<'a> {
    d: &'a str,
    sep: Option<&'a str>,
    pat: &'static [&'static str],
}

impl<'a> Splitter<'a> {
    fn new(d: &'a str) -> Self {
        Self {
            d,
            sep: None,
            pat: &["::", ":", ",", "(", ")", "..", ".", "|", ";", "?"],
        }
    }
}

impl<'a> Iterator for Splitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sep) = self.sep.take() {
            return Some(sep);
        }
        if let Some((pat, _)) = self
            .pat
            .iter()
            .filter_map(|p| self.d.find(p).map(|x| (p, x)))
            .min_by(|(ap, al), (bp, bl)| al.cmp(bl).then(bp.len().cmp(&ap.len())))
        {
            let Some((left, right)) = self.d.split_once(*pat) else {
                unreachable!()
            };
            self.sep = Some(pat);
            self.d = right;
            return Some(left);
        }
        // if we reach this point, but d is non-empty, then no pattern has matched.
        if !self.d.is_empty() {
            let d = self.d;
            self.d = "";
            return Some(d);
        }
        None
    }
}

fn quine_prog(prog: &str) -> String {
    let escaped_prog = prog.replace('\\', "\\\\");
    let escaped_prog = escaped_prog.as_str().replace('"', "\\\"");
    prog.replacen('~', &escaped_prog.trim(), 1)
}
