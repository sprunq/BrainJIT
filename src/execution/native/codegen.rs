use dynasmrt::{Assembler, AssemblyOffset};

use crate::syntax::Instruction;

use super::{executor::NativeExecutor, x64::X64CodeGen};
pub trait NativeCodeGenBackend {
    type Relocation: dynasmrt::relocations::Relocation;

    fn generate_prolouge(&self, ops: &mut Assembler<Self::Relocation>) -> AssemblyOffset;

    fn generate_epilouge(&self, ops: &mut Assembler<Self::Relocation>);

    fn generate_instruction(&self, ops: &mut Assembler<Self::Relocation>, instr: &Instruction) {
        match instr {
            Instruction::Increment { value } => self.generate_increment(ops, value.0),
            Instruction::CellIncrement { value } => self.generate_cell_increment(ops, *value),
            Instruction::Loop { nodes } => self.generate_loop(ops, nodes),
            Instruction::Write => self.generate_write(ops),
            Instruction::Read => self.generate_read(ops),
        }
    }

    fn generate_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i8);

    fn generate_cell_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i32);

    fn generate_loop(&self, ops: &mut Assembler<Self::Relocation>, nodes: &[Instruction]);

    fn generate_write(&self, ops: &mut Assembler<Self::Relocation>);

    fn generate_read(&self, ops: &mut Assembler<Self::Relocation>);
}

pub struct CodeGeneration<B>
where
    B: NativeCodeGenBackend,
{
    codegen: B,
    ops: dynasmrt::Assembler<B::Relocation>,
}

impl CodeGeneration<X64CodeGen> {
    pub fn x64() -> CodeGeneration<X64CodeGen> {
        CodeGeneration {
            codegen: X64CodeGen,
            ops: dynasmrt::x64::Assembler::new().unwrap(),
        }
    }
}

impl<B> CodeGeneration<B>
where
    B: NativeCodeGenBackend,
{
    pub fn generate(mut self, instrs: &[Instruction]) -> NativeExecutor {
        let start = self.codegen.generate_prolouge(&mut self.ops);

        for instr in instrs {
            self.codegen.generate_instruction(&mut self.ops, instr);
        }

        self.codegen.generate_epilouge(&mut self.ops);

        match self.ops.finalize() {
            Ok(code) => NativeExecutor::new(code, start),
            Err(_) => todo!(),
        }
    }
}
