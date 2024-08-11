//! This is the compiler for translating the IR to assembly
//! Future: This will use the low-level IR at some point but
//!         until the lir is finished, it will use the high-level IR
//!
//! Generally this is only serves as a helper for the actual Backend#compile
//! function.

use std::collections::{HashMap, HashSet};

use citadel_frontend::{
    hir::{
        self, irgen::TypeTable, ArithOpExpr, BlockStmt, CallExpr, ExitStmt, FuncStmt, IRExpr,
        IRStmt, JumpStmt, LabelStmt, ReturnStmt, StructInitExpr, Type, VarStmt, INT16_T,
        INT32_T, INT64_T, INT8_T,
    },
    util::CompositeDataType,
};

use crate::asm::{
    elements::{
        AsmElement, DataSize, Declaration, Directive, DirectiveType, Instruction, Label, Literal,
        Opcode, Operand, Register, Size, SizedLiteral, StdFunction,
    },
    utils::codegen as cutils,
};

pub const FUNCTION_ARG_REGISTERS_8: [Register; 6] = [
    Register::Al,
    Register::Bl,
    Register::Cl,
    Register::Dl,
    Register::R9b,
    Register::R10b,
];

pub const FUNCTION_ARG_REGISTERS_16: [Register; 6] = [
    Register::Ax,
    Register::Bx,
    Register::Cx,
    Register::Dx,
    Register::R9w,
    Register::R10w,
];

pub const FUNCTION_ARG_REGISTERS_32: [Register; 6] = [
    Register::Edi,
    Register::Esi,
    Register::Edx,
    Register::Ecx,
    Register::R9d,
    Register::R10d,
];

pub const FUNCTION_ARG_REGISTERS_64: [Register; 6] = [
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R9,
    Register::R10,
];

#[derive(Default)]
pub struct CodeGenerator<'c> {
    pub out: Vec<AsmElement>,
    pub types: TypeTable<'c>,

    // Literals
    /// Read only data section
    pub rodata: Vec<Declaration>,
    pub data: Vec<Declaration>,
    /// Literal constant index
    pub lc_index: usize,

    pub defined_functions: HashSet<StdFunction>,
    pub symbol_table: HashMap<&'c str, i32>,

    pub stack_pointer: i32,
}

impl<'c> CodeGenerator<'c> {
    pub fn new(types: TypeTable<'c>) -> Self {
        Self {
            types,
            ..Default::default()
        }
    }

    pub fn gen_stmt(&mut self, node: &'c IRStmt) {
        match node {
            IRStmt::DeclaredFunction(_) => todo!(),
            IRStmt::Function(node) => self.gen_function(node),
            IRStmt::Entry(node) => self.gen_entry(node),
            IRStmt::Struct(_) => (),
            IRStmt::Union(_) => (),
            IRStmt::Variable(node) => self.gen_variable(node),
            IRStmt::Label(node) => self.gen_label(node),
            IRStmt::Return(node) => self.gen_return(node),
            IRStmt::Exit(node) => self.gen_exit(node),
            IRStmt::Jump(node) => self.gen_jump(node),
            IRStmt::Call(node) => self.gen_call(node),
        }
    }

    fn gen_expr(&mut self, node: &'c IRExpr) -> Operand {
        match &node {
            IRExpr::Literal(node, type_) => match node {
                hir::Literal::Int32(val) => {
                    Operand::SizedLiteral(SizedLiteral(Literal::Int32(*val), DataSize::DWord))
                }
                hir::Literal::String(val) => self.gen_string(val, type_),
                int => todo!("Handle non-i32 literals here: {:?}", int),
            },
            IRExpr::Call(node) => {
                self.gen_call(node);
                let reg = Register::Rax;
                Operand::Register(reg)
            }
            IRExpr::ArithOp(node) => self.gen_arith_op(node, true),
            IRExpr::Ident(node) => cutils::get_stack_location(
                *self
                    .symbol_table
                    .get(node)
                    .unwrap_or_else(|| panic!("Could not find ident with name {node:?}")),
            ),
            IRExpr::StructInit(node) => self.gen_struct_init(node),
        }
    }

    pub fn gen_entry(&mut self, node: &'c BlockStmt<'c>) {
        // Text directive (entry point)
        self.out.push(AsmElement::Directive(Directive {
            _type: DirectiveType::Text,
        }));
        self.out.push(AsmElement::Declaration(Declaration::Global(
            "_start".to_string(),
        )));

        // _start label
        self.out.push(AsmElement::Label(Label {
            name: "_start".to_string(),
        }));
        for stmt in &node.stmts {
            self.gen_stmt(stmt);
        }
    }

    fn gen_call(&mut self, node: &'c CallExpr) {
        match node.name {
            "print" => self.gen_print(node),
            _ => {
                self.gen_call_args(node);
                self.out.push(cutils::gen_call(&node.name))
            }
        }
    }

    fn gen_jump(&mut self, node: &'c JumpStmt) {
        self.out.push(AsmElement::Instruction(Instruction {
            opcode: Opcode::Jmp,
            args: vec![Operand::Ident(node.label.to_string())],
        }))
    }

    fn gen_string(&mut self, val: &str, type_: &Type<'c>) -> Operand {
        let size = *match type_ {
            Type::Ident(_) => todo!(),
            Type::Array(_, len) => len,
        };
        // TODO: use different splitting techniques based on string length
        let mut strings = cutils::split_string(val, 8);
        let last_string = strings.pop().unwrap();
        self.stack_pointer -= size as i32;
        Operand::SizedLiteral(SizedLiteral(
            Literal::Int64(cutils::conv_str_to_bytes(last_string) as i64),
            cutils::word_from_size(size as u8),
        ))
    }

    fn gen_arith_op(&mut self, node: &'c ArithOpExpr, move_to_rax: bool) -> Operand {
        if move_to_rax {
            let left_expr = self.gen_expr(&node.values.0);
            self.gen_mov_ins(Operand::Register(Register::Rax), left_expr)
        }
        let arith_op = match node.op {
            hir::Operator::Add => self.gen_arith_op_ins(Opcode::Add, node),
            hir::Operator::Sub => self.gen_arith_op_ins(Opcode::Sub, node),
            hir::Operator::Mul => self.gen_arith_op_ins(Opcode::Mul, node),
            hir::Operator::Div => self.gen_arith_op_ins(Opcode::Div, node),
        };
        self.out.push(arith_op);
        Operand::Register(Register::Rax)
    }

    fn gen_arith_op_ins(&mut self, opcode: Opcode, node: &'c ArithOpExpr) -> AsmElement {
        AsmElement::Instruction(Instruction {
            opcode,
            args: vec![
                Operand::Register(Register::Rax),
                match &*node.values.1 {
                    IRExpr::ArithOp(expr) => self.gen_arith_op(expr, false),
                    expr => self.gen_expr(expr),
                },
            ],
        })
    }

    fn gen_return(&mut self, node: &'c ReturnStmt) {
        let val = self.gen_expr(&node.ret_val);
        self.out
            .push(cutils::gen_mov_ins(Operand::Register(Register::Rax), val));
        self.out.push(cutils::destroy_stackframe());
        self.out.push(cutils::gen_ret());
    }

    fn gen_exit(&mut self, node: &'c ExitStmt) {
        let expr = self.gen_expr(&node.exit_code);
        self.out
            .push(cutils::gen_mov_ins(Operand::Register(Register::Rdi), expr));
        self.gen_mov_ins(
            Operand::Register(Register::Rax),
            Operand::Literal(Literal::Int32(60)),
        );
        self.out.push(cutils::gen_syscall());
    }

    fn gen_variable(&mut self, node: &'c VarStmt) {
        let size = self.size_of(&node.name._type);
        let mut val = self.gen_expr(&node.val);
        // FIXME: This is a hack to ensure that the size does not get decremented for arrays
        if let Type::Ident(_) = node.name._type {
            self.stack_pointer -= size as i32
        }

        if let Operand::Literal(lit) = val {
            val = Operand::SizedLiteral(cutils::literal_to_sized_literal(lit)
                .expect("Failed to convert literal to sized literal, most likely caused due to usage of float which are not supported yet"))
        };

        if let Operand::SizedLiteral(SizedLiteral(lit, DataSize::QWord)) = val {
            self.gen_mov_ins(Operand::Register(Register::Rax), Operand::Literal(lit));
            val = Operand::Register(Register::Rax);
        }

        self.gen_mov_ins(cutils::get_stack_location(self.stack_pointer), val);

        self.symbol_table
            .insert(&node.name.ident, self.stack_pointer);
    }

    fn gen_function(&mut self, node: &'c FuncStmt) {
        self.out.push(AsmElement::Label(Label {
            name: node.name.ident.to_string(),
        }));

        let stack_frame = cutils::create_stackframe();

        self.out.push(stack_frame.0);
        self.out.push(stack_frame.1);

        self.gen_args(node);

        for stmt in &node.block.stmts {
            self.gen_stmt(stmt);
        }

        if let Some(elem) = self.out.last() {
            match elem {
                AsmElement::Instruction(Instruction {
                    opcode: Opcode::Ret,
                    ..
                }) => (),
                _ => {
                    self.out.push(cutils::destroy_stackframe());
                    self.out.push(cutils::gen_ret());
                }
            }
        }
    }

    fn gen_struct_init(&mut self, node: &'c StructInitExpr) -> Operand {
        let size = self.size_of(&hir::Type::Ident(node.name));
        self.gen_mov_ins(
            cutils::get_stack_location(self.stack_pointer - size as i32),
            Operand::Literal(Literal::Int32(0)),
        );
        self.stack_pointer -= size as i32;
        // TODO: Use type suffixes for this
        for (i, val) in node.values.iter().enumerate() {
            let fields = &self.types.get(&node.name).unwrap().1;
            let _field = &fields[i];
            let expr = self.gen_expr(val);
            self.gen_mov_ins(cutils::get_stack_location(0), expr);
        }
        todo!()
    }

    fn gen_args(&mut self, node: &'c FuncStmt) {
        for (i, expr) in node.args.iter().enumerate() {
            let size = self.size_of(&expr._type);
            self.gen_mov_ins(
                cutils::get_stack_location(self.stack_pointer - size as i32),
                Operand::Register(
                    cutils::arg_regs_by_size(size.try_into().expect("Failed to convert u32 to u8"))
                        [i],
                ),
            );
            self.stack_pointer -= size as i32;
            self.symbol_table.insert(&expr.ident, self.stack_pointer);
        }
    }

    fn gen_call_args(&mut self, node: &'c CallExpr) {
        for (i, expr) in node.args.iter().enumerate() {
            let val = self.gen_expr(expr);
            self.gen_mov_ins(
                Operand::Register(cutils::arg_regs_by_size(val.size())[i]),
                val,
            );
        }
    }

    fn gen_print(&mut self, node: &'c CallExpr) {
        let arg = self.gen_expr(
            node.args
                .first()
                .expect("Print function neeeds at least one argument"),
        );
        self.gen_mov_ins(Operand::Register(Register::Rsi), arg);
        self.gen_mov_ins(
            Operand::Register(Register::Rdx),
            Operand::Literal(Literal::Int8(8)),
        );
        self.out.push(cutils::gen_call("print"));
        self.defined_functions.insert(StdFunction::Print);
    }

    fn gen_label(&mut self, node: &'c LabelStmt) {
        self.out.push(AsmElement::Label(Label {
            name: node.name.to_string(),
        }));
    }

    /// Returns the size of the type in bytes
    fn size_of(&self, _type: &hir::Type<'c>) -> u32 {
        // The type or array is an integer type/array
        match _type {
            Type::Ident(ident @ (INT8_T | INT16_T | INT32_T | INT64_T)) => {
                return cutils::int_size(*ident) as u32;
            }
            Type::Array(Type::Ident(ident @ (INT8_T | INT16_T | INT32_T | INT64_T)), size) => {
                return cutils::int_size(*ident) as u32 * *size;
            }
            _ => (),
        }

        let type_name = match _type {
            Type::Ident(ident) => ident,
            Type::Array(ident, _) => match ident {
                Type::Ident(id) => id,
                Type::Array(id, _) => return self.size_of(id),
            },
        };

        let cdt = self
            .types
            .get(type_name)
            .unwrap_or_else(|| panic!("Could not find type with the name {}", _type));
        let mut size: u32 = 0;
        match cdt.0 {
            // Add sizes if cdt is a struct
            CompositeDataType::Struct => {
                for field in &cdt.1 {
                    size += self.size_of(&field._type);
                }
            }
            // Use largest size if cdt is a union
            CompositeDataType::Union => {
                for variant in &cdt.1 {
                    let size1 = self.size_of(&variant._type);
                    if size1 > size {
                        size = size1;
                    }
                }
            }
        }
        match _type {
            Type::Ident(_) => size,
            Type::Array(_, arr_size) => size * *arr_size,
        }
    }

    fn gen_mov_ins(&mut self, target: Operand, val: Operand) {
        if target != val {
            self.out.push(cutils::gen_mov_ins(target, val))
        }
    }
}
