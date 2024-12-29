use regex::Regex;

pub struct Lexer<'a> {
    lines: Vec<&'a str>,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Number(String),
    String(String),
    Identifier(String),
    Label(String),
    MacroDef(String),
    MacroExpand(String),
    MacroDefEnd,
    MacroExpandLabel(String),
    BlockParam(String),
    Keyword(String),
    GenericIdentifier(String),
    GenericLabel(String),
    Unknown(String),
}

impl<'a> Lexer<'a> {
    const TOKEN_RE: &'static str =
        r#"!!|-?\d+(\.\d+)?|\$(begin|end|include)|#\w+:?|[&@!]\w+|\w+!:|\w+[:!]?|"[^"]*?""#;

    pub fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        Self { lines, line: 0 }
    }

    pub fn all_lines(&mut self) -> Vec<Vec<Token>> {
        let mut lines = Vec::new();
        while self.line < self.lines.len() {
            lines.push(self.next_line());
        }
        lines.into_iter().filter(|e| !e.is_empty()).collect()
    }

    fn next_line(&mut self) -> Vec<Token> {
        while Regex::new(r"^\\.*$").unwrap().is_match(self.lines[self.line]) {
            self.line += 1;
        }
        let token_re = Regex::new(Self::TOKEN_RE).unwrap();
        let tokens: Vec<Token> = token_re
            .find_iter(self.lines[self.line])
            .map(|m| {
                // if Regex::new(r#" "" "#).unwrap().is_match(&s) {
                //     dbg!(&s);
                // }
                let s = m.as_str().to_string();
                if Regex::new(r"!!").unwrap().is_match(&s) {
                    Token::MacroDefEnd
                } else if Regex::new(r"^-?\d+(\.\d+)?$").unwrap().is_match(&s) {
                    Token::Number(s)
                } else if Regex::new(r#"^"[^"]*?"$"#).unwrap().is_match(&s) {
                    Token::String(s)
                } else if Regex::new(r"^\w+$").unwrap().is_match(&s) {
                    Token::Identifier(s)
                } else if Regex::new(r"^\w+:$").unwrap().is_match(&s) {
                    Token::Label(s.strip_suffix(':').unwrap().to_string())
                } else if Regex::new(r"^!\w+$").unwrap().is_match(&s) {
                    Token::MacroDef(s.strip_prefix('!').unwrap().to_string())
                } else if Regex::new(r"^\w+!$").unwrap().is_match(&s) {
                    Token::MacroExpand(s.strip_suffix('!').unwrap().to_string())
                } else if Regex::new(r"^\w+!:$").unwrap().is_match(&s) {
                    Token::MacroExpandLabel(s.strip_suffix("!:").unwrap().to_string())
                } else if Regex::new(r"^\$\w+$").unwrap().is_match(&s) {
                    Token::Keyword(s.strip_prefix('$').unwrap().to_string())
                } else if Regex::new(r"^&\w+$").unwrap().is_match(&s) {
                    Token::BlockParam(s.strip_prefix('&').unwrap().to_string())
                } else if Regex::new(r"^#\w+$").unwrap().is_match(&s) {
                    Token::GenericIdentifier(s.strip_prefix('#').unwrap().to_string())
                } else if Regex::new(r"^#\w+:$").unwrap().is_match(&s) {
                    Token::GenericLabel(
                        s.strip_prefix('#')
                            .unwrap()
                            .strip_suffix(':')
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    Token::Unknown(s)
                }
            })
            .collect();
        self.line += 1;
        tokens
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(val) | Token::String(val) | Token::Identifier(val) => val.clone(),
            Token::Label(val) => format!("{val}:"),
            Token::MacroDef(val) => format!("!{val}"),
            Token::MacroExpand(val) => format!("{val}!"),
            Token::MacroDefEnd => "!!".into(),
            Token::MacroExpandLabel(val) => format!("{val}!:"),
            Token::BlockParam(val) => format!("&{val}"),
            Token::Keyword(val) => format!("${val}"),
            Token::GenericIdentifier(val) => format!("#{val}"),
            Token::GenericLabel(val) => format!("#{val}:"),
            Token::Unknown(val) => val.clone(),
        }
    }
}
