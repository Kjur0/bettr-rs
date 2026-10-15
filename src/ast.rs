#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i32),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Exit(Option<Expression>),
}

impl Statement {
    pub fn exit_code(&self) -> Option<i32> {
        match self {
            Statement::Exit(Some(Expression::Integer(value))) => Some(*value),
            Statement::Exit(None) => Some(0),
            // future statements
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
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
