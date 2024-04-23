//! Simple representation of symvols and idents of the IR

use logos::Logos;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Logos)]
#[logos(skip r#"(?:\/\/[^\n]*|\t|\s|\f|\n)*"#)]
pub enum Token<'tok> {
    // --special-characters--
    /// $ - define a constant
    #[token("$")]
    DollarSign,
    /// @ - define a function
    #[token("@")]
    At,
    /// % - reference a variable or function
    #[token("%")]
    PercentSign,
    /// ? - define a variable
    #[token("?")]
    QuestionMark,
    /// = - assign a value to a varable/constant
    #[token("=")]
    Assign,
    /// ' - the marker of a label
    #[token("'")]
    Apostrophe,
    /// : - colon is used to mark a variety of things like labels and primtive type suffixes
    #[token(":")]
    Colon,
    /// , - comma is used to list arguments
    #[token(",")]
    Comma,
    /// . - dot is used for namespaces and methods
    #[token(".")]
    Dot,

    /// Brackets
    /// ( - left parenthesis
    #[token("(")]
    LParent,
    /// ) - right parenthesis
    #[token(")")]
    RParent,
    /// [ - left square brackets
    #[token("[")]
    LSquare,
    /// ] - right square brackets
    #[token("]")]
    RSquare,
    /// { - left curly brackets
    #[token("{")]
    LCurly,
    /// } - right curly brackets
    #[token("}")]
    RCurly,

    // --keywords--
    /// indicates the definition of a struct
    #[token("struct")]
    Struct,
    /// indicates the defintion of a union
    #[token("union")]
    Union,
    /// marks a function as declared only meaning it gets initialized in a different module
    #[token("decl")]
    Decl,
    /// Define a function
    #[token("func")]
    Func,
    /// Call a function
    #[token("call")]
    Call,
    /// Cast a type to a different type
    #[token("cast")]
    Cast,
    /// Return a value
    #[token("ret")]
    Ret,
    /// Break a loop
    #[token("br")]
    Break,
    /// Jump to a label
    #[token("jmp")]
    Jump,
    /// Exit the program
    #[token("exit")]
    Exit,
    // Arithmetic Operations
    /// Addition
    #[token("add")]
    Add,
    /// Subtraction
    #[token("sub")]
    Sub,
    /// Multiplication
    #[token("mul")]
    Mul,
    /// Division
    #[token("div")]
    Div,
    /// Modulo operator
    #[token("mod")]
    Mod,

    #[regex(r#"l\{\s*\"([^\"\n]*)\"\s*\}"#)]
    LitString(&'tok str),

    #[regex(r"l\{(\d+)\}")]
    LitInt(&'tok str),

    #[regex(r"l\{'(.)'\}")]
    LitChar(&'tok str),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'tok str),
}