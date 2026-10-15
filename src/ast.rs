#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i32),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Exit(Option<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub main: Vec<Statement>,
}
