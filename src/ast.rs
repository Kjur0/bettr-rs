#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i32),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Exit(Option<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub main: Vec<Statement>,
}

impl Program {
    pub fn normalize(&mut self) {
        Self::complete_main(&mut self.main);
    }

    fn complete_main(statements: &mut Vec<Statement>) {
        if statements.is_empty() {
            statements.push(Statement::Exit(None));
        } else if let Some(Statement::Exit(_)) = statements.last() {
            return;
        } else {
            statements.push(Statement::Exit(None));
        }
    }
}
