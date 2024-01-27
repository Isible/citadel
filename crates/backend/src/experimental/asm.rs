#[derive(Debug, Clone, PartialEq)]
pub enum AsmElement {
    Label,
    Instruction,
    Directive(Directive),
    Operand(Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register,
    MemAddr,
    Literal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    Data,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub _type: DirectiveType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {

}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Mov,
    Br,
    Syscall,

    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    XOr,
    Not,
    Cmp,

    Jmp,
    JE,
    JNe,
    JZ,
    JNz,

    Call,
    Ret,

    Push,
    Pop,

    Shl,
    Shr,

    Movsb,
    Movsw,
    Int,

    Fadd,
    Fsub,
    FMul,
    FDiv,

    FCmp,
    FAbs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub _type: InstructionType,
    pub args: Vec<Operand>
}