use std::mem;

use super::{state::State, RuntimeResultCode, TAPE_SIZE};

pub struct NativeExecutor {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

impl NativeExecutor {
    pub fn new(code: dynasmrt::ExecutableBuffer, start: dynasmrt::AssemblyOffset) -> Self {
        Self { code, start }
    }

    pub fn run(self, state: &mut State) -> RuntimeResultCode {
        let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
            unsafe { mem::transmute(self.code.ptr(self.start)) };
        let start = state.tape.as_mut_ptr();
        let end = unsafe { start.add(TAPE_SIZE) };
        let result = f(state, start, start, end);
        RuntimeResultCode::try_from(result).unwrap()
    }
}
