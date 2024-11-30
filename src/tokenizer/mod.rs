use core::panic;

#[derive(Debug)]
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
        return Tokenizer {
            input,
            current_position: 0,
            tokens: Vec::new(),
        };
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.current_position..)?.chars().next()
    }

    fn advance(&mut self) {
        self.current_position += 1;
    }

    fn handle_literals(&mut self) {
        let quote_char = self.current_char().unwrap();
        self.advance();

        let start = self.current_position;
        while let Some(c) = self.current_char() {
            if c == quote_char {
                let literal = self.input[start..self.current_position].to_string();
                self.tokens.push(Token::StringLiteral(literal));
                self.advance();
                return;
            }
            self.advance();
        }
        panic!("Unterminated string literal!");
    }

    fn handle_aplhabetic(&mut self) {
        let start = self.current_position;
        while let Some(c) = self.current_char() {
            if c.is_alphabetic() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let keywords: [&str; 6] = ["SELECT", "FROM", "WHERE", "UPDATE", "DELETE", "INSERT"];
        let phrase = self.input[start..self.current_position].to_string();
        let upper_phrase = self.input[start..self.current_position].to_uppercase();
        if keywords.contains(&upper_phrase.as_str()) {
            self.tokens.push(Token::Keyword(upper_phrase));
        } else {
            self.tokens.push(Token::Identifier(phrase));
        }
    }

    fn handle_numeric(&mut self) {
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
                Err(_) => panic!("Falied to parse float in tokenizer!"),
            }
        } else {
            let number: Result<i64, _> = self.input[start..self.current_position].parse();
            match number {
                Ok(value) => self.tokens.push(Token::Number(value)),
                Err(_) => panic!("Failed to parse int in tokenizer!"),
            }
        }
    }

    fn handle_operator(&mut self, initial_char: char) {
        let mut operator = initial_char.to_string();
        self.advance();

        if let Some(next_char) = self.current_char() {
            if next_char == '=' {
                operator.push(next_char);
                self.advance();
            }
        }
        self.tokens.push(Token::Operator(operator));
    }

    fn handle_logical_operator(&mut self, initial_char: char) {
        let mut operator = initial_char.to_string();
        self.advance();

        if let Some(next_char) = self.current_char() {
            if next_char == initial_char {
                operator.push(next_char);
                self.advance();
            }
        }
        self.tokens.push(Token::Operator(operator));
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while self.current_position < self.input.len() {
            match self.current_char() {
                Some(c) if c.is_whitespace() => self.advance(),
                Some(c) if c == '"' || c == '\'' => self.handle_literals(),
                Some(c) if c == ';' || c == ',' || c == '(' || c == ')' => {
                    self.advance();
                    self.tokens.push(Token::Delimiter(c));
                }
                Some(c) if "+-*/=".contains(c) => {
                    self.advance();
                    self.tokens.push(Token::Operator(c.to_string()));
                }
                Some(c) if c == '<' || c == '>' => self.handle_operator(c),
                Some(c) if c == '&' || c == '|' => self.handle_logical_operator(c),
                Some(c) if c.is_alphabetic() => self.handle_aplhabetic(),
                Some(c) if c.is_digit(10) => self.handle_numeric(),
                Some(_) => panic!("Unrecognised Token!"),
                None => return &self.tokens,
            }
        }
        &self.tokens
    }
}
