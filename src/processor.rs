use std::collections::HashMap;

pub struct MacroProcessor {
    tokens: Vec<Vec<String>>,
    macros: HashMap<String, Macro>,
    i: usize,
}

#[derive(Debug, Clone)]
struct Macro {
    args: Vec<String>,
    body: Vec<Vec<String>>,
}

impl MacroProcessor {
    pub fn new(tokens: Vec<Vec<String>>) -> Self {
        Self {
            tokens,
            macros: HashMap::new(),
            i: 0,
        }
    }

    fn with_macros(&self, macros: &HashMap<String, Macro>) -> Self {
        Self {
            tokens: self.tokens.clone(),
            macros: macros.clone(),
            i: self.i,
        }
    }

    pub fn run(&mut self) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        while self.i < self.tokens.len() {
            let line = self.tokens[self.i].clone();
            self.i += 1;
            match line[0].as_str() {
                "$begin" | "$end" => continue,
                s if s.starts_with('!') && s.chars().all(|c| c.is_alphanumeric() || c == '!') => {
                    self.define_macro(&line);
                }
                s if s.ends_with('!') && s.chars().all(|c| c.is_alphanumeric() || c == '!') => {
                    result.extend(self.process_macro(&line));
                }
                _ => result.push(line),
            }
        }
        result
    }

    fn define_macro(&mut self, line: &[String]) {
        let name = &line[0][1..];
        let args = line[1..].to_vec();
        self.macros.insert(
            name.to_string(),
            Macro {
                args,
                body: Vec::new(),
            },
        );
        while self.tokens[self.i][0] != "!!" {
            self.macros
                .get_mut(name)
                .expect("Failed to get existing macro to add an argument.")
                .body
                .push(self.tokens[self.i].clone());
            self.i += 1;
        }
        self.i += 1;
    }

    fn process_macro(&mut self, line: &[String]) -> Vec<Vec<String>> {
        let name = &line[0][..line[0].len() - 1];
        // dbg!(&self.macros);
        let macro_def = self
            .macros
            .get(name)
            .unwrap_or_else(|| panic!("Macro with name '{name}' is not found."))
            .clone();
        let args: HashMap<String, String> = macro_def
            .args
            .iter()
            .zip(line[1..].iter())
            .filter(|(p, _)| !p.starts_with('&'))
            .map(|(p, a)| (p.clone(), a.clone()))
            .collect();
        let block_args: HashMap<String, Vec<Vec<String>>> = macro_def
            .args
            .iter()
            .filter(|p| p.starts_with('&'))
            .map(|p| (p.clone(), self.parse_block()))
            .collect();

        let mut body_with_macros = Vec::new();
        for line in &macro_def.body {
            let lines = if line.len() == 1 {
                // dbg!(&line);
                if let Some(key) = line[0].strip_suffix("!:") {
                    if let Some(arg) = args.get(key) {
                        vec![vec![format!("{}:", arg)]]
                    } else {
                        vec![line.clone()]
                    }
                } else if line[0].starts_with('&') {
                    // dbg!(&block_args);
                    if let Some(block) = block_args.get(&line[0]) {
                        block.clone()
                    } else {
                        vec![line.clone()]
                    }
                } else {
                    vec![line.clone()]
                }
            } else {
                let mut new_line = Vec::new();
                for token in line {
                    if token.ends_with('!') {
                        if let Some(arg) = args.get(&token[..token.len() - 1]) {
                            new_line.push(arg.clone());
                        } else {
                            new_line.push(token.clone());
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
            .run()
    }

    fn parse_block(&mut self) -> Vec<Vec<String>> {
        let mut block = Vec::new();
        self.i += 1;
        while self.tokens[self.i][0] != "$end" {
            block.push(self.tokens[self.i].clone());
            self.i += 1;
        }
        self.i += 1;
        block
    }
}

pub fn stringify_tokens(tokens: Vec<Vec<String>>) -> String {
    tokens
        .into_iter()
        .map(|s| s.join(" "))
        .collect::<Vec<String>>()
        .join("\n")
}
