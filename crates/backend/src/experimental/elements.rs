#[derive(Debug, Clone, PartialEq)]
pub enum AsmElement {
    Label(Label),
    Instruction(Instruction),
    Directive(Directive),
    Operand(Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub _type: DirectiveType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub _type: InstructionType,
    pub args: Vec<Operand>
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    Data,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    // TODO: Might be able to change this to vec<instruction>
    pub elements: Vec<AsmElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    MemAddr(Address),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,

    Rdi,
    Rsi,
    Rbp,
    Rsp,

    
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Mov,
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

    Dec,
    Inc,
}