use super::codegen::NativeCodeGenBackend;
use crate::{execution::native::state::State, syntax::Instruction};
use dynasmrt::dynasm;
use dynasmrt::DynasmApi;
use dynasmrt::DynasmLabelApi;
use dynasmrt::{x64::X64Relocation, Assembler, AssemblyOffset};

/// Alias registers for easier access in the generated code.
/// Store all our relevant data in registers.
/// Registers are nonvolatile and must be presereved
/// across function calls by the callee.
/// https://learn.microsoft.com/en-us/cpp/build/x64-software-conventions?view=msvc-170
#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "windows")]
macro_rules! alias_asm {
    ($ops:expr, $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias state, r12
            ; .alias tape_start, r13
            ; .alias tape_end, r14
            ; .alias cell_ptr, r15

            ; .alias retval, rax
            ; .alias retval_lower_8, al

            ; .alias fn_call_reg, rax

            ; .alias first_arg, rcx
            ; .alias second_arg, rdx
            ; .alias third_arg, r8
            ; .alias fourth_arg, r9
            $($t)*
        )
    }
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
macro_rules! alias_asm {
    ($ops:expr, $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias state, r12
            ; .alias tape_start, r13
            ; .alias tape_end, r14
            ; .alias cell_ptr, r15

            ; .alias retval, rax
            ; .alias retval_lower_8, al

            ; .alias fn_call_reg, rax

            ; .alias first_arg, rdi
            ; .alias second_arg, rsi
            ; .alias third_arg, rdx
            ; .alias fourth_arg, rcx
            $($t)*
        )
    }
}

macro_rules! x64_save_registers {
    ($ops:expr) => {
        alias_asm!($ops,
            ; sub rsp, 32
            ; push state
            ; push tape_start
            ; push tape_end
            ; push cell_ptr
        );
    };
}

macro_rules! x64_restore_registers {
    ($ops:expr) => {
        alias_asm!($ops,
            ; pop cell_ptr
            ; pop tape_end
            ; pop tape_start
            ; pop state
            ; add rsp, 32
        );
    };
}

/// The x64 code generation backend. Only for windows x64.
pub struct X64CodeGen;

impl NativeCodeGenBackend for X64CodeGen {
    type Relocation = X64Relocation;

    fn generate_prolouge(&self, ops: &mut Assembler<Self::Relocation>) -> AssemblyOffset {
        let start = ops.offset();
        alias_asm!(ops,
            // Allocate shadow space for win64 calling convention
            ; sub rsp, 32

            ;; x64_save_registers!(ops)

            // Save the passed arguments to their registers
            // Signature: fn(State* state, u8* tape_start, u8* tape_end) -> u8
            ; mov state, first_arg
            ; mov tape_start, second_arg
            ; mov tape_end, third_arg

            ; mov cell_ptr, tape_start
        );
        start
    }

    fn generate_epilouge(&self, ops: &mut Assembler<Self::Relocation>) {
        macro_rules! epilogue {
            ($ops:expr, $e:expr) => {
                alias_asm!($ops,
                    ;; x64_restore_registers!($ops)
                    ; mov retval, $e
                    ; add rsp, 32
                    ; ret
                );
            };
        }

        // All possible exit points from the program can be jumped to by their respective label.
        alias_asm!(ops,
            ;; epilogue!(ops, 0)
            ;->error_io:
            ;; epilogue!(ops, 1)
        );
    }

    /// Handles overflows and underflows by wrapping around the value.
    fn generate_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i8) {
        alias_asm!(ops,
            ; add BYTE [cell_ptr], value
        );
    }

    fn generate_set(&self, ops: &mut Assembler<Self::Relocation>, value: u8) {
        alias_asm!(ops,
            ; mov BYTE [cell_ptr], value as i8
        );
    }

    /// Does not handle overflows and underflows. Will panic if the value is or out of bounds.
    fn generate_cell_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i32) {
        alias_asm!(ops,
            ; add cell_ptr, value
        );
    }

    fn generate_loop(&self, ops: &mut Assembler<Self::Relocation>, nodes: &[Instruction]) {
        let backward_label = ops.new_dynamic_label();
        let forward_label = ops.new_dynamic_label();

        // Start of the loop: Check if the current cell is 0, jump to the forward label (end of loop) if true.
        alias_asm!(ops,
            ; cmp BYTE [cell_ptr], 0
            ; jz =>forward_label
            ;=>backward_label
        );

        // Generate the instructions inside the loop
        for node in nodes {
            self.generate_instruction(ops, node);
        }

        // End of the loop: Jump back to the start of the loop if the condition is still true.
        alias_asm!(ops,
            ; cmp BYTE [cell_ptr], 0
            ; jnz =>backward_label
            ;=>forward_label
        );
    }

    fn generate_write(&self, ops: &mut Assembler<Self::Relocation>) {
        alias_asm!(ops,
            ;; x64_save_registers!(ops)

            ; mov first_arg, state
            ; mov second_arg, cell_ptr
            ; mov fn_call_reg, QWORD State::putchar as *const () as i64
            ; call fn_call_reg

            ;; x64_restore_registers!(ops)

            ; cmp retval_lower_8, 0
            ; jnz ->error_io
        );
    }

    fn generate_read(&self, ops: &mut Assembler<Self::Relocation>) {
        alias_asm!(ops,
            ;; x64_save_registers!(ops)

            ; mov first_arg, state
            ; mov second_arg, cell_ptr
            ; mov fn_call_reg, QWORD State::getchar as *const () as i64
            ; call fn_call_reg

            ;; x64_restore_registers!(ops)

            ; cmp retval_lower_8, 0
            ; jnz ->error_io
        );
    }
}
