use crate::tokenizer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Query {
    Select(SelectQuery),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SelectQuery {
    selected_columns: Vec<Column>,
    table_name: Table,
    where_clause: Option<Condition>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Column {
    name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Table {
    name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Condition {
    left: ConditionEnum,
    operator: Operator,
    right: ConditionEnum,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConditionEnum {
    Field(Column),
    Value(Value),
}

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    position: usize,
}

impl <'a>Parser<'a>{
    pub fn new(token_stream: &'a Vec<Token>) -> Self {
        Parser {
            tokens: token_stream,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        let start = self.advance();
        match start {
            Token::Keyword(ref keyword) => match keyword.as_str() {
                "SELECT" => Ok(Query::Select(self.handle_select()?)),
                _ => Err("Invalid query type".to_string()),
            },
            _ => Err("Expected keyword token at the beginning!".to_string()),
        }
    }

    fn handle_select(&mut self) -> Result<SelectQuery, String> {
        let columns = self.parse_column_list()?;

        self.consume_keyword(Token::Keyword("FROM".to_string()))?;
        let table = self.parse_table()?;

        let where_clause = if self.check_keyword("WHERE") {
            self.advance(); 
            Some(self.parse_condition()?)
        } else {
            None
        };

        Ok(SelectQuery {
            selected_columns: columns,
            table_name: table,
            where_clause,
        })
    }

    fn parse_column_list(&mut self) -> Result<Vec<Column>, String> {
        let mut columns = Vec::new();

        loop {
            columns.push(self.parse_column()?);
            if self.peek() == &Token::Delimiter(',') {
                self.advance(); 
            } else {
                break;
            }
        }

        Ok(columns)
    }

    fn parse_column(&mut self) -> Result<Column, String> {
        if let Token::Identifier(name) = self.advance() {
            Ok(Column { name })
        } else {
            Err("Expected column name".to_string())
        }
    }

    fn parse_table(&mut self) -> Result<Table, String> {
        if let Token::Identifier(name) = self.advance() {
            Ok(Table { name })
        } else {
            Err("Expected table name".to_string())
        }
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        let left = self.parse_expression()?;
        let operator = self.parse_operator()?;
        let right = self.parse_expression()?;

        Ok(Condition {
            left,
            operator,
            right,
        })
    }

    fn parse_expression(&mut self) -> Result<ConditionEnum, String> {
        match self.advance() {
            Token::Identifier(name) => Ok(ConditionEnum::Field(Column { name })),
            Token::StringLiteral(text) => Ok(ConditionEnum::Value(Value::Text(text))),
            Token::Float(float) => Ok(ConditionEnum::Value(Value::Float(float))),
            Token::Number(integer) => Ok(ConditionEnum::Value(Value::Integer(integer))),
            _ => Err("Expected expression".to_string()),
        }
    }

    fn parse_operator(&mut self) -> Result<Operator, String> {
        if let Token::Operator(op) = self.advance() {
            match op.as_str() {
                "=" => Ok(Operator::Equal),
                "!=" => Ok(Operator::NotEqual),
                ">" => Ok(Operator::GreaterThan),
                "<" => Ok(Operator::LessThan),
                ">=" => Ok(Operator::GreaterOrEqual),
                "<=" => Ok(Operator::LessOrEqual),
                _ => Err(format!("Unknown operator: {}", op)),
            }
        } else {
            Err("Expected operator!".to_string())
        }
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        if let Token::Keyword(ref k) = self.peek() {
            k == keyword
        } else {
            false
        }
    }

    fn consume_keyword(&mut self, keyword: Token) -> Result<(), String> {
        if self.check(&keyword) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", keyword, self.peek()))
        }
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }
}
