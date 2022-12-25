use std::env;

use getopts::Options;

use language::CommentSyntax;

mod language;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("usage: {} [options] [COMMENT]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("l", "language", "Programing language (default: shell)", "");
    opts.optopt("L", "length", "Comment length (default: 80)", "");
    opts.optflag("h", "help", "Print help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let language = match matches.opt_str("language") {
        None => String::from("shell"),
        Some(value) => value
    };

    let length: usize = match matches.opt_str("length") {
        None => 80,
        Some(value) => value.parse().expect(&format!("invalid length: {}", value))
    };

    let comment = match matches.free.as_slice() {
        [x, ..] => x,
        [] => ""
    };

    print!("{}", get_comment(CommentSyntax::get(&language), length, comment));
}

fn get_comment(syntax: CommentSyntax, length: usize, comment: &str) -> String {
    let comment = if comment.trim().len() == 0 { "".to_string() } else { format!(" {} ", comment.trim()) };
    let empty_space_len = syntax.start.len() + syntax.end.len() + 2;

    if empty_space_len + comment.len() >= length {
        format!("{}{}{}", syntax.start, comment, syntax.end)
    } else {
        let comment = format!("{:-^1$}", comment, length - empty_space_len);
        format!("{} {} {}", syntax.start, comment, syntax.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comment_default_to_shell() {
        let given = get_comment(CommentSyntax::get(""), 80, "some comment");
        let expected = String::from("# ------------------------------- some comment ------------------------------- #");
        assert_eq!(given, expected);
    }

    #[test]
    fn test_get_comment() {
        let given = get_comment(CommentSyntax::get("c"), 80, "some comment");
        let expected = String::from("/* ------------------------------ some comment ------------------------------ */");
        assert_eq!(given, expected);
    }

    #[test]
    fn test_get_comment_empty() {
        let given = get_comment(CommentSyntax::get("html"), 80, "");
        let expected = String::from("<!- ------------------------------------------------------------------------- ->");
        assert_eq!(given, expected);
    }
}
