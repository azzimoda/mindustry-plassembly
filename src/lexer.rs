use regex::Regex;

pub struct Lexer<'a> {
    lines: Vec<&'a str>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        Self { lines, line: 0 }
    }

    pub fn all_lines(&mut self) -> Vec<Vec<String>> {
        let mut lines = Vec::new();
        while self.line < self.lines.len() {
            lines.push(self.next_line());
        }
        lines.into_iter().filter(|e| !e.is_empty()).collect()
    }

    fn next_line(&mut self) -> Vec<String> {
        let token_re =
            Regex::new(r#"!!|-?\d+\.\d+|-?\d+|\$(begin|end)|[&@!]\w+|\w+!:|\w+[:!]?|"[^"]*?""#)
                .unwrap();
        let tokens: Vec<String> = token_re
            .find_iter(self.lines[self.line])
            .map(|m| {
                let s = m.as_str().to_string();
                if Regex::new(r#" "" "#).unwrap().is_match(&s) {
                    dbg!(&s);
                }
                s
            })
            .collect();
        self.line += 1;
        tokens
    }
}
