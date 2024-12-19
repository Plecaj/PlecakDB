#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Float(f64),
    Number(i64),
    StringLiteral(String),
    Operator(String),
    Delimiter(char),
}

pub struct Tokenizer<'a> {
    input: &'a str,
    current_position: usize,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            current_position: 0,
            tokens: Vec::new(),
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.current_position..)?.chars().next()
    }

    fn advance(&mut self) {
        self.current_position += 1;
    }

    fn handle_literals(&mut self) -> Result<(), String> {
        let quote_char = match self.current_char() {
            Some(c) => c,
            None => return Err("Expected a quote character but found end of input".to_string()),
        };
        self.advance();

        let start = self.current_position;
        while let Some(c) = self.current_char() {
            if c == quote_char {
                let literal = self.input[start..self.current_position].to_string();
                self.tokens.push(Token::StringLiteral(literal));
                self.advance();
                return Ok(());
            }
            self.advance();
        }
        Err("Unterminated string literal".to_string())
    }

    fn handle_alphabetic(&mut self) -> Result<(), String> {
        let start = self.current_position;
        while let Some(c) = self.current_char() {
            if c.is_alphabetic() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let keywords: [&str; 11] = ["SELECT", "FROM", "WHERE", "ORDER", "GROUP", "DELETE", "UPDATE", "SET", "INSERT", "INTO", "VALUES"];
        let phrase = self.input[start..self.current_position].to_string();
        let upper_phrase = phrase.to_uppercase();
        if keywords.contains(&upper_phrase.as_str()) {
            self.tokens.push(Token::Keyword(upper_phrase));
        } else {
            self.tokens.push(Token::Identifier(phrase));
        }
        Ok(())
    }

    fn handle_numeric(&mut self) -> Result<(), String> {
        let start = self.current_position;
        let mut has_dot = false;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.advance();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        if has_dot {
            let number: Result<f64, _> = self.input[start..self.current_position].parse();
            match number {
                Ok(value) => self.tokens.push(Token::Float(value)),
                Err(_) => return Err("Failed to parse float".to_string()),
            }
        } else {
            let number: Result<i64, _> = self.input[start..self.current_position].parse();
            match number {
                Ok(value) => self.tokens.push(Token::Number(value)),
                Err(_) => return Err("Failed to parse integer".to_string()),
            }
        }
        Ok(())
    }

    fn handle_operator(&mut self, initial_char: char) -> Result<(), String> {
        let mut operator = initial_char.to_string();
        self.advance();

        if let Some(next_char) = self.current_char() {
            if next_char == '=' {
                operator.push(next_char);
                self.advance();
            } else if initial_char == '!' && next_char == '=' {
                operator.push(next_char);
                self.advance();
            }
        }
        self.tokens.push(Token::Operator(operator));
        Ok(())
    }

    fn handle_logical_operator(&mut self, initial_char: char) -> Result<(), String> {
        let mut operator = initial_char.to_string();
        self.advance();

        if let Some(next_char) = self.current_char() {
            if next_char == initial_char {
                operator.push(next_char);
                self.advance();
            }
        }
        self.tokens.push(Token::Operator(operator));
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, String> {
        while self.current_position < self.input.len() {
            match self.current_char() {
                Some(c) if c.is_whitespace() => self.advance(),
                Some(c) if c == '"' || c == '\'' => self.handle_literals()?,
                Some(c) if c == ';' || c == ',' || c == '(' || c == ')' => {
                    self.tokens.push(Token::Delimiter(c));
                    self.advance();
                }
                Some(c) if "+-*/=".contains(c) => {
                    self.tokens.push(Token::Operator(c.to_string()));
                    self.advance();
                }
                Some(c) if c == '<' || c == '>' || c == '!' => self.handle_operator(c)?,
                Some(c) if c == '&' || c == '|' => self.handle_logical_operator(c)?,
                Some(c) if c.is_alphabetic() => self.handle_alphabetic()?,
                Some(c) if c.is_digit(10) => self.handle_numeric()?,
                Some(_) => {
                    return Err(format!("Unrecognized token at position {}", self.current_position));
                }
                None => return Ok(&self.tokens),
            }
        }
        Ok(&self.tokens)
    }
}
