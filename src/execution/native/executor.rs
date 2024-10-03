use std::mem;

use super::{state::State, TAPE_SIZE};

pub struct NativeExecutor {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

impl NativeExecutor {
    pub fn new(code: dynasmrt::ExecutableBuffer, start: dynasmrt::AssemblyOffset) -> Self {
        Self { code, start }
    }

    pub fn run(self, state: &mut State) -> Result<(), &'static str> {
        let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
            unsafe { mem::transmute(self.code.ptr(self.start)) };
        let start = state.tape.as_mut_ptr();
        let end = unsafe { start.offset(TAPE_SIZE as isize) };
        let res = f(state, start, start, end);
        if res == 0 {
            Ok(())
        } else if res == 1 {
            Err("An overflow occurred")
        } else if res == 2 {
            Err("IO error")
        } else {
            panic!("Unknown error code");
        }
    }
}
