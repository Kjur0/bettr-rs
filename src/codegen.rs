use crate::ast::Program;

pub fn generate(program: &Program) -> String {
    let exit_code = program
        .main
        .last()
        .expect("main should always have a final statement")
        .exit_code()
        .expect("main always has a final exitcode");

    format!(
        ".intel_syntax noprefix\n\
        .global _start\n\
        \n\
        .section .text\n\
        _start:\n\
            \tmov rax, 60\n\
            \tmov rdi, {exit_code}\n\
            \tsyscall\n\
        "
    )
}
