//! Performs the final compilation from citadel's x86 format to machine code

use std::fs::File;

use object::{
    write::{Object, Relocation, StandardSection, Symbol, SymbolSection},
    Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationFlags, RelocationKind,
    SymbolFlags, SymbolKind, SymbolScope,
};

use crate::x86::{
    elements::{Instruction, Register},
    CompileResult,
};

pub struct MachineGenerator<'m> {
    obj: Object<'m>,
    main_data: Vec<u8>,
}

impl<'m> MachineGenerator<'m> {
    pub fn new() -> Self {
        Self {
            obj: Object::new(
                BinaryFormat::native_object(),
                Architecture::X86_64,
                Endianness::Little,
            ),
            main_data: Vec::new(),
        }
    }

    pub fn generate(&mut self, input: CompileResult) {
        self.obj.add_file_symbol(b"hello.c".into());

        for ins in input.instructions {
            match ins {
                Instruction::MovR2R { val, dest } => todo!(),
                Instruction::MovI2R { val, dest } => self.gen_move_i2r(val, dest),
                Instruction::MovM2R { val, dest } => todo!(),
                Instruction::MovR2M { val, dest } => todo!(),
                Instruction::Syscall => self.gen_syscall(),
            }
        }

        self.create_start();
        
                // Write the object file.
        let file = File::create("hello.o").unwrap();
        self.obj.write_stream(file).unwrap();
    }

    fn create_start(&mut self) {
        // Add a globally visible symbol for the main function.
        let start_symbol = self.obj.add_symbol(Symbol {
            name: b"_start".into(),
            value: 0,
            size: self.main_data.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Undefined,
            flags: SymbolFlags::None,
        });
    
        // Add the _start function in the .text section.
        let text_section = self.obj.section_id(StandardSection::Text);
        self.obj
            .add_symbol_data(start_symbol, text_section, &self.main_data, 1);
    }
    
    fn gen_move_i2r(&mut self, val: i64, dest: Register) {
        let opcode: u8 = match dest {
            Register::Rax => 0xb8,
            Register::Rdi => 0xbf,
            _ => todo!(),
        };
        self.main_data.push(opcode);
        let val: u8 = val
            .try_into()
            .ok()
            .unwrap_or_else(|| panic!("Silly mode activated, val cant be cast to u8 :3"));
        self.main_data.push(val);
        self.main_data.extend_from_slice(&[0x0, 0x0, 0x0]);
    }

    fn gen_syscall(&mut self) {
        self.main_data.extend_from_slice(&[0x0f, 0x05]);
    }
}
