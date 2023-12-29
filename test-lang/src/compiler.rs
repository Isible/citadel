use frontend::{ast::*, ir_gen::IRGenerator};

use crate::{ast::Statement, parser::Parser};

pub struct Compiler<'a> {
    pub generator: IRGenerator,
    pub parser: &'a mut Parser<'a>,
    pub cur_stmt: Option<Statement>,
}

impl<'a> Compiler<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Self {
        let cur_stmt = match parser.parse_stmt() {
            Ok(stmt) => Some(stmt),
            Err(_) => None,
        };

        let mut compiler = Self {
            generator: IRGenerator::new(),
            cur_stmt,
            parser,
        };

        compiler.init_program();

        compiler
    }

    pub fn compile_program(&mut self) {
        loop {
            self.next_stmt();
            if self.cur_stmt != None {
                self.compile_stmt();
            } else {
                break;
            }
        }
    }

    fn init_program(&mut self) {
        self.generator
            .gen_ir(frontend::ast::Statement::Label(LabelStmt {
                name: "entry".into(),
                block: BlockStmt {
                    stmts: vec![frontend::ast::Statement::Expression(
                        frontend::ast::Expression::Call(CallExpr {
                            name: "main".into(),
                            args: vec![frontend::ast::Expression::Call(CallExpr {
                                name: "citadel.std.env.args".into(),
                                args: Vec::new(),
                            })],
                        }),
                    )],
                },
            }))
    }

    fn compile_stmt(&mut self) {
        match &self.cur_stmt {
            Some(stmt) => match stmt {
                Statement::Let(_let) => todo!(),
                Statement::Fn(_fn) => todo!(),
                Statement::If(_if) => todo!(),
                Statement::Loop(_loop) => todo!(),
                Statement::Call(_call) => todo!(),
                Statement::Block(_block) => todo!(),
            },
            None => return,
        }
    }

    fn next_stmt(&mut self) {
        self.cur_stmt = match self.parser.parse_stmt() {
            Ok(stmt) => Some(stmt),
            Err(_) => None,
        };
    }
}
