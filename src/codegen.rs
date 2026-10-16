use crate::ast::{Expression, Program, Statement};

struct Emmiter {
    output: String,
}

impl Emmiter {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn line(&mut self, text: &str) {
        self.output.push_str(text);
        self.output.push('\n');
    }

    fn finish(self) -> String {
        self.output
    }

    fn emit_program(&mut self, program: &Program) {
        self.line(".intel_syntax noprefix");
        self.line(".global _start");
        self.line("");
        self.line(".section .text");
        self.line("_start:");

        for statement in &program.main {
            self.emit_statement(statement);
        }
    }

    fn emit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Exit(value) => self.emit_exit(value.as_ref()),
        }
    }

    fn emit_exit(&mut self, code: Option<&Expression>) {
        match code {
            Some(expression) => self.emit_exit_value(expression),
            None => self.line("\tmov rdi, 0"),
        };

        self.line(&format!("\tmov rax, 60"));
        self.line(&format!("\tsyscall"));
    }

    fn emit_exit_value(&mut self, expression: &Expression) {
        match expression {
            Expression::Integer(value) => {
                self.line(&format!("\tmov rdi, {value}"));
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                self.emit_exit_value(left);
                self.line("\tpush rdi");
                self.emit_exit_value(right);
                self.line("\tpop rax");

                match operator {
                    crate::ast::BinaryOperator::Add => {
                        self.line("\tadd rdi, rax");
                    }
                }
            }
        }
    }
}

pub fn generate(program: &Program) -> String {
    let mut emitter = Emmiter::new();
    emitter.emit_program(program);
    emitter.finish()
}
