use crate::ast::*;
use crate::lexer::{Lexer, Token};
use anyhow::{Result, anyhow};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.lexer.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.lexer.peek() {
            Some(Token::Ye) => self.parse_var_declaration(),
            Some(Token::Func) => self.parse_func_declaration(),
            Some(Token::Agar) => self.parse_if_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::Wapas) => self.parse_wapas_kar_statement(),
            Some(Token::Jabtak) => self.parse_while_statement(),
            Some(Token::Har) => self.parse_for_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement> {
        self.consume(Token::Ye, "Expected 'ye'")?;

        let name = match self.lexer.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(anyhow!("Expected variable name")),
        };

        self.consume(Token::Colon, "Expected ':' after variable name")?;

        let type_annotation = self.parse_type()?;

        self.consume(Token::Assign, "Expected '=' after type")?;

        let initializer = self.parse_expression()?;

        self.consume(Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Statement::VarDecl {
            name,
            type_annotation,
            initializer,
        })
    }

    fn parse_func_declaration(&mut self) -> Result<Statement> {
        self.consume(Token::Func, "Expected 'func'")?;

        let name = match self.lexer.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(anyhow!("Expected function name")),
        };

        self.consume(Token::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();

        if !matches!(self.lexer.peek(), Some(Token::RightParen)) {
            loop {
                let param_name = match self.lexer.advance() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => return Err(anyhow!("Expected parameter name")),
                };

                let param_type = self.parse_type()?;
                params.push((param_name, param_type));

                if matches!(self.lexer.peek(), Some(Token::Comma)) {
                    self.lexer.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after parameters")?;

        let return_type = self.parse_type()?;

        self.consume(Token::LeftBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();
        while !matches!(self.lexer.peek(), Some(Token::RightBrace)) && !self.lexer.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after function body")?;

        Ok(Statement::FuncDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.consume(Token::Agar, "Expected 'agar'")?;

        let condition = self.parse_expression()?;

        self.consume(Token::LeftBrace, "Expected '{' after if condition")?;

        let mut then_branch = Vec::new();
        while !matches!(self.lexer.peek(), Some(Token::RightBrace)) && !self.lexer.is_at_end() {
            then_branch.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after if body")?;

        let else_branch = if matches!(self.lexer.peek(), Some(Token::Varna)) {
            self.lexer.advance();
            self.consume(Token::LeftBrace, "Expected '{' after 'varna'")?;

            let mut else_stmts = Vec::new();
            while !matches!(self.lexer.peek(), Some(Token::RightBrace)) && !self.lexer.is_at_end() {
                else_stmts.push(self.parse_statement()?);
            }

            self.consume(Token::RightBrace, "Expected '}' after else body")?;
            Some(else_stmts)
        } else {
            None
        };

        Ok(Statement::IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.consume(Token::Return, "Expected 'return'")?;

        let value = if matches!(self.lexer.peek(), Some(Token::Semicolon)) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Token::Semicolon, "Expected ';' after return statement")?;

        Ok(Statement::ReturnStmt { value })
    }

    fn parse_wapas_kar_statement(&mut self) -> Result<Statement> {
        self.consume(Token::Wapas, "Expected 'wapas'")?;
        self.consume(Token::Kar, "Expected 'kar' after 'wapas'")?;

        let value = if matches!(self.lexer.peek(), Some(Token::Semicolon)) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume(Token::Semicolon, "Expected ';' after wapas kar statement")?;

        Ok(Statement::ReturnStmt { value })
    }

    fn parse_while_statement(&mut self) -> Result<Statement> {
        self.consume(Token::Jabtak, "Expected 'jabtak'")?;

        let condition = self.parse_expression()?;

        self.consume(Token::LeftBrace, "Expected '{' after while condition")?;

        let mut body = Vec::new();
        while !matches!(self.lexer.peek(), Some(Token::RightBrace)) && !self.lexer.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after while body")?;

        Ok(Statement::WhileStmt { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement> {
        self.consume(Token::Har, "Expected 'har'")?;

        let variable = match self.lexer.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(anyhow!("Expected variable name after 'har'")),
        };

        self.consume(Token::Mein, "Expected 'mein' after variable")?;

        let iterable = self.parse_expression()?;

        self.consume(Token::LeftBrace, "Expected '{' after for expression")?;

        let mut body = Vec::new();
        while !matches!(self.lexer.peek(), Some(Token::RightBrace)) && !self.lexer.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::RightBrace, "Expected '}' after for body")?;

        Ok(Statement::ForStmt {
            variable,
            iterable,
            body,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after expression")?;
        Ok(Statement::ExprStmt { expression: expr })
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut expr = self.parse_comparison()?;

        while let Some(token) = self.lexer.peek() {
            let operator = match token {
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.lexer.peek() {
            let operator = match token {
                Token::Greater => BinaryOperator::Greater,
                Token::GreaterEqual => BinaryOperator::GreaterEqual,
                Token::Less => BinaryOperator::Less,
                Token::LessEqual => BinaryOperator::LessEqual,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.lexer.peek() {
            let operator = match token {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        let mut expr = self.parse_unary()?;

        while let Some(token) = self.lexer.peek() {
            let operator = match token {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                _ => break,
            };

            self.lexer.advance();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        match self.lexer.peek() {
            Some(Token::Bang) => {
                self.lexer.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Minus) => {
                self.lexer.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.lexer.peek() {
                Some(Token::LeftParen) => {
                    self.lexer.advance();
                    let mut arguments = Vec::new();

                    if !matches!(self.lexer.peek(), Some(Token::RightParen)) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if matches!(self.lexer.peek(), Some(Token::Comma)) {
                                self.lexer.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.consume(Token::RightParen, "Expected ')' after arguments")?;

                    expr = Expression::Call {
                        callee: Box::new(expr),
                        arguments,
                    };
                }
                Some(Token::Dot) => {
                    self.lexer.advance();
                    let method = match self.lexer.advance() {
                        Some(Token::Identifier(name)) => name.clone(),
                        Some(Token::Bol) => "bol".to_string(), // Special case for paneer.bol
                        _ => return Err(anyhow!("Expected method name after '.'")),
                    };

                    self.consume(Token::LeftParen, "Expected '(' after method name")?;

                    let mut arguments = Vec::new();
                    if !matches!(self.lexer.peek(), Some(Token::RightParen)) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if matches!(self.lexer.peek(), Some(Token::Comma)) {
                                self.lexer.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.consume(Token::RightParen, "Expected ')' after method arguments")?;

                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method,
                        arguments,
                    };
                }
                Some(Token::LeftBracket) => {
                    self.lexer.advance();
                    let index = self.parse_expression()?;
                    self.consume(Token::RightBracket, "Expected ']' after array index")?;

                    expr = Expression::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match self.lexer.advance() {
            Some(Token::True) => Ok(Expression::Literal {
                value: LiteralValue::Bool(true),
            }),
            Some(Token::False) => Ok(Expression::Literal {
                value: LiteralValue::Bool(false),
            }),
            Some(Token::IntLiteral(value)) => Ok(Expression::Literal {
                value: LiteralValue::Int(*value),
            }),
            Some(Token::FloatLiteral(value)) => Ok(Expression::Literal {
                value: LiteralValue::Float(*value),
            }),
            Some(Token::StringLiteral(value)) => Ok(Expression::Literal {
                value: LiteralValue::String(value.clone()),
            }),
            Some(Token::Identifier(name)) => Ok(Expression::Variable { name: name.clone() }),
            Some(Token::Paneer) => Ok(Expression::Variable {
                name: "paneer".to_string(),
            }),
            Some(Token::LeftParen) => {
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            Some(Token::LeftBracket) => {
                let mut elements = Vec::new();

                if !matches!(self.lexer.peek(), Some(Token::RightBracket)) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if matches!(self.lexer.peek(), Some(Token::Comma)) {
                            self.lexer.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.consume(Token::RightBracket, "Expected ']' after array elements")?;

                Ok(Expression::ArrayLiteral { elements })
            }
            _ => Err(anyhow!("Expected expression")),
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        match self.lexer.advance() {
            Some(Token::IntType) => Ok(Type::Int),
            Some(Token::FloatType) => Ok(Type::Float),
            Some(Token::StringType) => Ok(Type::String),
            Some(Token::BoolType) => Ok(Type::Bool),
            Some(Token::ArrayType) => {
                self.consume(Token::Less, "Expected '<' after 'array'")?;
                let inner_type = self.parse_type()?;
                self.consume(Token::Greater, "Expected '>' after array element type")?;
                Ok(Type::Array(Box::new(inner_type)))
            }
            _ => Err(anyhow!("Expected type annotation")),
        }
    }

    fn consume(&mut self, expected: Token, message: &str) -> Result<()> {
        match self.lexer.peek() {
            Some(token) if std::mem::discriminant(token) == std::mem::discriminant(&expected) => {
                self.lexer.advance();
                Ok(())
            }
            _ => Err(anyhow!("{}", message)),
        }
    }
}
