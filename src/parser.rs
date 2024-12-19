use crate::tokenizer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Query {
    Select(SelectQuery),
    Insert(InsertQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
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
pub struct InsertQuery{
    table_name: Table,
    columns: Vec<Column>,
    values: Vec<Value>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeleteQuery{
    table_name: Table,
    where_clause: Option<Condition>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UpdateQuery{
    table_name: Table,
    changes: Vec<UpdateSet>,
    where_clause: Option<Condition>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UpdateSet{
    column: Column,
    value: Value,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Column {
    name: String,
}

#[derive(Debug, Clone)]
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
#[allow(dead_code)]
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
                "INSERT" => Ok(Query::Insert(self.handle_insert()?)),
                "UPDATE" => Ok(Query::Update(self.handle_update()?)),
                "DELETE" => Ok(Query::Delete(self.handle_delete()?)),
                _ => Err("Invalid query type".to_string()),
            },
            _ => Err("Expected keyword token at the beginning!".to_string()),
        }
    }

    fn handle_select(&mut self) -> Result<SelectQuery, String> {
        let columns = self.parse_column_list()?;

        self.consume_token(Token::Keyword("FROM".to_string()))?;
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

    fn handle_insert(&mut self) -> Result<InsertQuery, String>{
        self.consume_token(Token::Keyword("INTO".to_string()))?;
        let table = self.parse_table()?;

        self.consume_token(Token::Delimiter('('))?;
        let columns = self.parse_column_list()?;
        self.consume_token(Token::Delimiter(')'))?;

        self.consume_token(Token::Keyword("VALUES".to_string()))?;
        self.consume_token(Token::Delimiter('('))?;
        let values = self.parse_value_list()?;
        self.consume_token(Token::Delimiter(')'))?;

        Ok(InsertQuery {
            table_name: table,
            columns: columns, 
            values: values
        })
    }

    fn handle_update(&mut self) -> Result<UpdateQuery, String>{
        let table = self.parse_table()?;

        self.consume_token(Token::Keyword("SET".to_string()))?;
        let update_changes = self.parse_set_list()?;

        let where_clause = if self.check_keyword("WHERE") {
            self.advance();
            Some(self.parse_condition()?)
        } else {
            None
        };
    
        Ok(UpdateQuery {
            table_name: table,
            changes: update_changes,
            where_clause,
        })
    }

    fn handle_delete(&mut self) -> Result<DeleteQuery, String> {
        self.consume_token(Token::Keyword("FROM".to_string()))?;
        let table = self.parse_table()?;
        
        let where_clause = if self.check_keyword("WHERE") {
            self.advance();
            Some(self.parse_condition()?)
        } else {
            None
        };
        
        Ok(DeleteQuery {
            table_name: table,
            where_clause,
        })
    }

    fn parse_set_list(&mut self) -> Result<Vec<UpdateSet>, String>{
        let mut changes = Vec::new();

        loop {
            changes.push(self.parse_set()?);
            if self.peek() == &Token::Delimiter(','){
                self.advance();
            } else {
                break;
            }
        }
        Ok(changes)
    }

    fn parse_set(&mut self) -> Result<UpdateSet, String>{
        let column = self.parse_column()?;

        if let Token::Operator(op) = self.advance() {
            if op != "=" {
                return Err("Expected '=' in SET clause".to_string());
            }
        } else {
            return Err("Expected '=' operator in SET clause".to_string());
        }

        let value = self.parse_value()?;

        Ok(UpdateSet{
            column: column,
            value: value,
        })      
    }

    fn parse_value_list(&mut self) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();

        loop {
            values.push(self.parse_value()?);
            if self.peek() == &Token::Delimiter(',') {
                self.advance(); 
            } else {
                break;
            }
        }

        Ok(values)
    }

    fn parse_value(&mut self) -> Result<Value, String>{
        let token = self.advance();
        match token{
            Token::Float(value) => return Ok(Value::Float(value)),
            Token::Number(value) => return Ok(Value::Integer(value)),
            Token::StringLiteral(text) =>  return Ok(Value::Text(text)),
            _ => return Err("Expected value".to_string()),
        }
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

    fn consume_token(&mut self, keyword: Token) -> Result<(), String> {
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
