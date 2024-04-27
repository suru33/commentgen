use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct CommentSyntax {
    pub start: String,
    pub end: String,
}

impl CommentSyntax {
    pub fn new(start: &str, end: &str) -> Self {
        CommentSyntax { start: start.to_string(), end: end.to_string() }
    }

    pub fn get(language: &str) -> CommentSyntax {
        // https://en.wikipedia.org/wiki/Comparison_of_programming_languages_(syntax)
        let style_c = CommentSyntax::new("/*", "*/");
        let style_markup = CommentSyntax::new("<!-", "->");
        let style_shell = CommentSyntax::new("#", "#");
        let style_erlang = CommentSyntax::new("%", "%");
        let style_curl = CommentSyntax::new("|#", "#|");
        let style_julia = CommentSyntax::new("#=", "=#");
        let style_haskell = CommentSyntax::new("{-", "-}");
        let style_clojure = CommentSyntax::new(";;", ";;");
        let style_sql = CommentSyntax::new("--", "--");
        let style_ps = CommentSyntax::new("<#", "#>");
        let style_lisp = CommentSyntax::new("#|", "|#");
        let style_pascal = CommentSyntax::new("{", "}");
        let style_jsp = CommentSyntax::new("<%--", "--%>");
        let style_ocaml = CommentSyntax::new("(*", "*)");
        let style_assembly = CommentSyntax::new(";", ";");

        let lang_map: HashMap<&str, &CommentSyntax> = HashMap::from([
            ("shell", &style_shell),
            ("java", &style_c),
            ("scala", &style_c),
            ("kotlin", &style_c),
            ("rust", &style_c),
            ("r", &style_shell),
            ("objective-c", &style_c),
            ("erlang", &style_erlang),
            ("tex", &style_erlang),
            ("latex", &style_erlang),
            ("matlab", &style_erlang),
            ("ocaml", &style_ocaml),
            ("perl", &style_shell),
            ("ruby", &style_shell),
            ("elixir", &style_shell),
            ("swift", &style_c),
            ("go", &style_c),
            ("curl", &style_curl),
            ("julia", &style_julia),
            ("haskell", &style_haskell),
            ("clojure", &style_clojure),
            ("python", &style_shell),
            ("js", &style_c),
            ("ts", &style_c),
            ("sql", &style_sql),
            ("c", &style_c),
            ("cpp", &style_c),
            ("csharp", &style_c),
            ("html", &style_markup),
            ("xml", &style_markup),
            ("php", &style_c),
            ("lua", &style_sql),
            ("ps", &style_ps),
            ("lisp", &style_lisp),
            ("pascal", &style_pascal),
            ("jsp", &style_jsp),
            ("assembly", &style_assembly),
            ("obj", &style_assembly),
        ]);

        if language == "_" {
            for (k, _) in &lang_map {
                println!("{}", k);
            }
            std::process::exit(0);
        }

        return match lang_map.get(language) {
            None => {
                eprintln!("language `{}` not found, defaults to shell style", language);
                style_shell
            }
            Some(language) => (*language).clone()
        };
    }
}

impl Clone for CommentSyntax {
    fn clone(&self) -> Self {
        Self::new(&self.start, &self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_syntax() {
        let given = CommentSyntax::get("");
        let expected = CommentSyntax::new("#", "#");
        assert_eq!(given, expected);
    }
}
