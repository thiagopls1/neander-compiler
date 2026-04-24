use super::{NdrError, Token, TokenKind};

//
// =======================
// AST
// =======================
//

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VarDeclare { name: String, value: Expr },
    VarAssign { name: String, value: Expr },
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Operation {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, NdrError> {
        self.parse_program()
    }

    // ==============
    // ==== CORE ====
    // ==============

    fn parse_program(&mut self) -> Result<Program, NdrError> {
        self.skip_until(TokenKind::ProgramStart)?;
        self.consume(TokenKind::ProgramStart)?;
        self.consume(TokenKind::NewLine)?;

        let mut statements = vec![];

        while !self.check(&TokenKind::ProgramEnd) {
            if self.check(&TokenKind::NewLine) {
                self.advance();
                continue;
            }

            statements.push(self.parse_statement()?);

            self.consume(TokenKind::NewLine)?;
        }

        self.consume(TokenKind::ProgramEnd)?;

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, NdrError> {
        match self.peek() {
            Some(t) => match t.kind {
                TokenKind::DeclareVariable => self.parse_var_decl(),
                TokenKind::Variable => self.parse_var_assign(),
                _ => Err(NdrError::UnexpectedToken { token: t.clone() }),
            },
            None => Err(NdrError::UnexpectedEOF),
        }
    }

    fn parse_var_decl(&mut self) -> Result<Statement, NdrError> {
        self.consume(TokenKind::DeclareVariable)?;

        let name = self.consume(TokenKind::Variable)?.value;

        self.consume(TokenKind::AssignVariable)?;

        let expr = self.parse_expr()?;

        Ok(Statement::VarDeclare { name, value: expr })
    }

    fn parse_var_assign(&mut self) -> Result<Statement, NdrError> {
        let name = self.consume(TokenKind::Variable)?.value;

        self.consume(TokenKind::AssignVariable)?;

        let expr = self.parse_expr()?;

        Ok(Statement::VarAssign { name, value: expr })
    }

    // ======================
    // ==== EXPRESSIONS =====
    // ======================

    fn parse_expr(&mut self) -> Result<Expr, NdrError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.peek() {
            let op = match token.kind {
                TokenKind::Sum => Operator::Add,
                TokenKind::Minus => Operator::Sub,
                _ => break,
            };

            self.advance();

            let right = self.parse_term()?;

            left = Expr::Operation {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, NdrError> {
        match self.peek() {
            Some(t) => match t.kind {
                TokenKind::Number => {
                    let value = self.advance().unwrap().value.parse().unwrap();
                    Ok(Expr::Number(value))
                }
                TokenKind::Variable => {
                    let name = self.advance().unwrap().value.clone();
                    Ok(Expr::Variable(name))
                }
                _ => Err(NdrError::UnexpectedToken { token: t.clone() }),
            },
            None => Err(NdrError::UnexpectedEOF),
        }
    }

    // =================
    // ==== HELPERS ====
    // =================

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.current);
        self.current += 1;
        tok
    }

    fn check(&self, kind: &TokenKind) -> bool {
        match self.peek() {
            Some(t) => std::mem::discriminant(&t.kind) == std::mem::discriminant(kind),
            None => false,
        }
    }

    fn consume(&mut self, kind: TokenKind) -> Result<Token, NdrError> {
        match self.peek() {
            Some(_t) if self.check(&kind) => Ok(self.advance().unwrap().clone()),
            Some(t) => Err(NdrError::UnexpectedToken { token: t.clone() }),
            None => Err(NdrError::UnexpectedEOF),
        }
    }

    fn skip_until(&mut self, kind: TokenKind) -> Result<(), NdrError> {
        while let Some(_token) = self.peek() {
            if self.check(&kind) {
                return Ok(());
            }
            self.advance();
        }

        Err(NdrError::UnexpectedEOF)
    }
}
