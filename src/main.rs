use std::env;
use std::process::exit;

use getopts::Options;

use language::CommentSyntax;

mod language;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("usage: {program} [options] [COMMENT]");
    eprint!("{}\n     ls \t\tList all supported languages\n", opts.usage(&brief));
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
            eprintln!("{}", f.to_string());
            print_usage(&program, opts);
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let language = matches.opt_str("language").unwrap_or_else(|| String::from("shell"));

    let length: usize = match matches.opt_str("length") {
        None => 80,
        Some(value) => match value.parse() {
            Ok(number) => number,
            Err(_) => {
                eprintln!("invalid length: {}", value);
                exit(1);
            }
        }
    };

    let comment = match matches.free.as_slice() {
        [x, ..] => x,
        [] => ""
    };

    if comment == "ls" && *&args.len() == 2 {
        CommentSyntax::get("_");
    }

    print!("{}", get_comment(CommentSyntax::get(&language), length, comment));
}

fn get_comment(syntax: CommentSyntax, length: usize, comment: &str) -> String {
    let comment = if comment.trim().len() == 0 { "".to_string() } else { format!(" {} ", comment.trim()) };
    let empty_space_len = syntax.start.len() + syntax.end.len() + 2;

    if empty_space_len + comment.len() >= length {
        format!("{}{comment}{}", syntax.start, syntax.end)
    } else {
        let comment = format!("{:-^1$}", comment, length - empty_space_len);
        format!("{} {comment} {}", syntax.start, syntax.end)
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
