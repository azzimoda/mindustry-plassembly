use std::collections::HashMap;

use crate::lexer::Token;

pub struct MacroProcessor {
    tokens: Vec<Vec<Token>>,
    macros: HashMap<String, Macro>,
    gen_ident_count: usize,
    i: usize,
}

#[derive(Debug, Clone)]
struct Macro {
    args: Vec<Token>,
    body: Vec<Vec<Token>>,
}

impl MacroProcessor {
    pub fn new(tokens: Vec<Vec<Token>>) -> Self {
        Self {
            tokens,
            macros: HashMap::new(),
            gen_ident_count: 0,
            i: 0,
        }
    }

    fn with_macros(&mut self, macros: &HashMap<String, Macro>) -> &mut Self {
        self.macros.extend(macros.clone());
        return self;
    }

    fn with_ident_count(&mut self, n: usize) -> &mut Self {
        self.gen_ident_count = n;
        self
    }

    pub fn run(&mut self) -> Vec<Vec<Token>> {
        let mut result = Vec::new();
        while self.i < self.tokens.len() {
            let line = self.tokens[self.i].clone();
            self.i += 1;

            match &line[0] {
                Token::MacroDef(_) => self.define_macro(&line),
                Token::MacroExpand(_) => result.extend(self.expand_macro(&line)),
                Token::MacroDefEnd
                | Token::MacroExpandLabel(_)
                | Token::BlockParam(_)
                | Token::GenericIdentifier(_)
                | Token::GenericLabel(_) => continue,
                Token::Keyword(kw) if matches!(kw.as_str(), "begin" | "end") => continue,
                _ => result.push(line),
            }
        }
        result
    }

    fn define_macro(&mut self, line: &[Token]) {
        let name = match &line[0] {
            Token::MacroDef(name) => name.clone(),
            _ => unreachable!("Defining macro not on Token::MacroDef"),
        };
        let args = line[1..].to_vec();
        self.macros.insert(
            name.to_string(),
            Macro {
                args,
                body: Vec::new(),
            },
        );
        while !matches!(self.tokens[self.i][0], Token::MacroDefEnd) {
            self.macros
                .get_mut(&name)
                .expect("Failed to get existing macro to add an argument.")
                .body
                .push(self.tokens[self.i].clone());
            self.i += 1;
        }
        self.i += 1;
    }

    fn expand_macro(&mut self, line: &[Token]) -> Vec<Vec<Token>> {
        let name = match &line[0] {
            Token::MacroExpand(name) => name.clone(),
            token => unreachable!("Expanding macro not on Token::MacroExpand(...): {token:?}"),
        };
        // dbg!(&self.macros);
        let macro_def = self
            .macros
            .get(&name)
            .unwrap_or_else(|| panic!("Macro with name '{name}' is not found."))
            .clone();
        let args: HashMap<String, Token> = macro_def
            .args
            .iter()
            .zip(line[1..].iter())
            .filter(|(p, _)| !matches!(p, Token::BlockParam(..)))
            .map(|(p, a)| (p.to_string(), a.clone()))
            .collect();
        let block_args: HashMap<String, Vec<Vec<Token>>> = macro_def
            .args
            .iter()
            .filter(|p| matches!(p, Token::BlockParam(..)))
            .map(|p| {
                let key = match p {
                    Token::BlockParam(param) => param.clone(),
                    _ => unreachable!(),
                };
                (key, self.parse_block())
            })
            .collect();

        let mut body_with_macros = Vec::new();
        let mut local_gen_idents: HashMap<String, String> = HashMap::new();
        for line in &macro_def.body {
            let lines = if line.len() == 1 {
                // dbg!(&line);
                if let Token::MacroExpandLabel(param) = &line[0] {
                    if let Some(arg) = args.get(param) {
                        vec![vec![Token::Label(arg.to_string())]]
                    } else {
                        vec![line.clone()]
                    }
                } else if let Token::BlockParam(param) = &line[0] {
                    // dbg!(&block_args);
                    if let Some(block) = block_args.get(param) {
                        block.clone()
                    } else {
                        vec![line.clone()]
                    }
                } else if let Token::GenericLabel(local_ident) = &line[0] {
                    if let Some(label) = local_gen_idents.get(local_ident) {
                        vec![vec![Token::Label(label.clone())]]
                    } else {
                        let label = self.generate_ident(&local_ident);
                        local_gen_idents.insert(local_ident.clone(), label.clone());
                        vec![vec![Token::Label(label)]]
                    }
                } else {
                    vec![line.clone()]
                }
            } else {
                let mut new_line = Vec::new();
                for token in line {
                    if let Token::MacroExpand(param) = token {
                        if let Some(arg) = args.get(param) {
                            new_line.push(arg.clone());
                        } else {
                            new_line.push(token.clone());
                        }
                    } else if let Token::GenericIdentifier(local_ident) = token {
                        if let Some(ident) = local_gen_idents.get(local_ident) {
                            new_line.push(Token::Identifier(ident.clone()))
                        } else {
                            let ident = self.generate_ident(&local_ident);
                            local_gen_idents.insert(local_ident.clone(), ident.clone());
                            new_line.push(Token::Identifier(ident))
                        }
                    } else {
                        new_line.push(token.clone());
                    }
                }
                vec![new_line]
            };
            body_with_macros.extend(lines);
        }

        MacroProcessor::new(body_with_macros)
            .with_macros(&self.macros)
            .with_ident_count(self.gen_ident_count)
            .run()
    }

    fn parse_block(&mut self) -> Vec<Vec<Token>> {
        let mut block = Vec::new();
        self.i += 1;
        let mut depth = 0;
        loop {
            match &self.tokens[self.i][0] {
                Token::Keyword(kw) if kw == "begin" => depth += 1,
                Token::Keyword(kw) if kw == "end" && depth == 0 => break,
                Token::Keyword(kw) if kw == "end" => depth -= 1,
                _ => (),
            }
            block.push(self.tokens[self.i].clone());
            self.i += 1;
        }
        self.i += 1;
        block
    }

    fn generate_ident(&mut self, s: &str) -> String {
        let i = self.gen_ident_count;
        self.gen_ident_count += 1;
        format!("__GI_{i}_{s}")
    }
}

pub fn stringify_tokens(tokens: Vec<Vec<Token>>) -> String {
    tokens
        .into_iter()
        .map(|s| {
            s.iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
