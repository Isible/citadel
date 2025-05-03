//! Performs the final compilation from citadel's x86 format to machine code

use std::{f64::consts::E, fs::File};

use byteorder::{LittleEndian, WriteBytesExt};
use citadel_middleend::lir::{self, irgen::LIRStream};
use object::{
    write::{Object, Relocation, SectionId, StandardSection, Symbol, SymbolSection}, Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationFlags, RelocationKind, SymbolFlags, SymbolKind, SymbolScope
};

use crate::{
    api::Target,
    x86::{
        machine::{Immediate, Instruction, Register},
        CompileResult, TargetX86_64,
    },
};

use super::{machine, MachineStream};

type ByteInstruction = u8;

#[derive(Debug, Default)]
struct Frame {
    instructions: Vec<ByteInstruction>,
    offset: u64,
}

pub struct MachineGenerator<'m, T: Target> {
    pub obj: Object<'m>,
    frames: Vec<Frame>,
    frames_index: usize,
    target: T,
}

impl<'m, T: Target> MachineGenerator<'m, T> {
    pub fn new(target: T) -> Self {
        Self {
            obj: Object::new(
                BinaryFormat::native_object(),
                Architecture::X86_64,
                Endianness::Little,
            ),
            frames: Vec::new(),
            frames_index: 0,
            target,
        }
    }

    pub fn generate(&mut self, mut input: MachineStream<'m>) {
        self.obj.add_file_symbol(b"hello.c".into());

        let start_label = input
            .labels
            .remove("main")
            .expect("Cannot find _start label");

        // We push a first frame for the entry block
        self.frames.push(Frame::default());
        dbg!(&input.instructions);

        self.compile_entry(&input.instructions[start_label..(start_label + input.entry_size)]);

        for ins in input.instructions {
            //    self.gen_ins(&ins);
        }

        // Write the object file.
        let file = File::create("hello.o").unwrap();
        self.obj.write_stream(file).unwrap();
    }

    fn gen_ins(&mut self, ins: &Instruction<'m>) {
        match ins {
            Instruction::MovR2R { val, dest } => todo!(),
            Instruction::MovI2R { val, dest } => self.gen_move_i2r(ins, *val, *dest),
            Instruction::MovM2R { val, dest } => todo!(),
            Instruction::MovR2M { val, dest } => todo!(),
            Instruction::Call { func } => self.gen_call_ins(ins, func), // self.gen_opcode_ins(ins),
            Instruction::Ret => self.gen_opcode_only_ins(ins),
            Instruction::Syscall => self.gen_opcode_only_ins(ins),
        }
    }

    fn compile_entry(&mut self, input: &[Instruction<'m>]) {
        let entry_frame_index = self.frames_index;
        for ins in input {
            self.gen_ins(ins);
        }
        self.frames.push(Frame::default());
        self.frames_index += 1;

        // Add a globally visible symbol for the main function.
        let entry_frame = &self.frames[entry_frame_index];

        let text_section = self.obj.section_id(StandardSection::Text);
        let start_symbol = self.obj.add_symbol(Symbol {
            name: b"_start".into(),
            value: 0,
            size: entry_frame.instructions.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text_section),
            flags: SymbolFlags::None,
        });

        // Add the _start function in the .text section.
        let _start_offset = self.obj.add_symbol_data(
            start_symbol,
            text_section,
            entry_frame.instructions.as_slice(),
            1,
        );
    }

    fn gen_move_i2r(&mut self, ins: &Instruction<'m>, val: Immediate, dest: Register) {
        let frame = &mut self.frames[self.frames_index];
        let opcode = ins.opcode(self.target);
        frame.instructions.extend_from_slice(opcode);
        let mut bytes = vec![];
        dbg!(val).write::<LittleEndian>(&mut bytes).unwrap();
        self.frames[self.frames_index]
            .instructions
            .extend_from_slice(&bytes);
    }

    fn gen_opcode_only_ins(&mut self, ins: &Instruction<'m>) {
        self.frames[self.frames_index]
            .instructions
            .extend_from_slice(ins.opcode(self.target));
    }

    fn gen_call_ins(&mut self, ins: Instruction<'m>, func: &str) {
        self.frames[self.frames_index]
            .instructions
            .extend_from_slice(ins.opcode(self.target));
        let func_symbol = self.obj.add_symbol(Symbol {
            name: func.as_bytes().to_vec(),
            value: 0,
            size: 0,
            kind: SymbolKind::Text,
            scope: SymbolScope::Dynamic,
            weak: false,
            section: SymbolSection::Undefined,
            flags: SymbolFlags::None,
        });
        self.obj.add_relocation(
            self.obj.section_id(StandardSection::Text),
            Relocation {
                offset: offset + 5, // the offset of the call's 4-byte displacement
                symbol: func_symbol,
                addend: -4,
                flags: RelocationFlags::Generic {
                    kind: RelocationKind::PltRelative,
                    encoding: RelocationEncoding::X86Branch,
                    size: 32,
                },
            },
        );
    }

    fn gen_functions(&mut self, ) {

    }

}
