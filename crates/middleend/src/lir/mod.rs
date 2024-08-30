pub mod opt;
pub mod irgen;

#[derive(Debug)]
pub enum Instruction {
    Mov {
        src: Operand,
        dest: Operand,
    }
}

#[derive(Debug)]
pub enum Operand {
    /// u64 represents index of the constant
    Constant(u64),
    Register,
}

// These are only placeholders for now
#[derive(Debug)]
pub enum Register {
    // General purpose registers
    Gr0,
    Gr1,
    Gr2,
    Gr3,

    // Argument registers
    Ar0,
    Ar1,
    Ar2,
    Ar3,

    // Specific registers
    /// Stack pointer register
    Sr,
}
