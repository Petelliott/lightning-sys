#![allow(clippy::mutex_atomic)] // Avoid clippy warning about JITS_MADE
#![allow(clippy::new_without_default)] // Avoid clippy warning about Jit::new
#![deny(unused_must_use)]

use std::sync::Mutex;

use crate::bindings;
use crate::JitState;

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Jit<'a>(PhantomData<&'a ()>);

lazy_static! {
    static ref JITS_MADE: Mutex<usize> = Mutex::new(0);
}

impl<'a> Jit<'a> {
    #[must_use]
    pub fn new() -> Jit<'a> {
        let mut m = JITS_MADE.lock().unwrap();

        if *m == 0 {
            unsafe {
                //TODO: figure out how to get ptr to argv[0]
                bindings::init_jit(std::ptr::null());
            }
        }

        *m += 1;
        Jit(PhantomData)
    }

    // This takes &mut self instead of &self because the unsafe operations wrapped herein are
    // inherently mutating.
    #[must_use]
    pub fn new_state(&mut self) -> JitState {
        JitState {
            state: unsafe {
                bindings::jit_new_state()
            },
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn r_num() -> bindings::jit_gpr_t {
        unsafe {
            bindings::lgsys_JIT_R_NUM()
        }
    }

    #[must_use]
    pub fn v_num() -> bindings::jit_gpr_t {
        unsafe {
            bindings::lgsys_JIT_V_NUM()
        }
    }

    #[must_use]
    pub fn f_num() -> bindings::jit_gpr_t {
        unsafe {
            bindings::lgsys_JIT_F_NUM()
        }
    }

}

impl<'a> Drop for Jit<'a> {
    fn drop(&mut self) {
        let mut m = JITS_MADE.lock().unwrap();
        *m -= 1;

        if *m == 0 {
            unsafe {
                bindings::finish_jit();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Jit;
    use crate::Reg;
    use crate::types::ToFFI;

    #[test]
    fn test_jit() {
        {
            let _jit = Jit::new();
            let _ = Jit::new();
        }

        {
            let _jit = Jit::new();
            let _ = Jit::new();
        }

    }

    #[test]
    fn test_reg_num() {
        assert!(Jit::r_num() >= 3);
        assert!(Jit::v_num() >= 3);
        assert!(Jit::f_num() >= 6);
    }

    #[test]
    #[should_panic]
    fn test_r_invalid() { let _ = Reg::R(Jit::r_num()).to_ffi(); }

    #[test]
    #[should_panic]
    fn test_v_invalid() { let _ = Reg::R(Jit::v_num()).to_ffi(); }

    #[test]
    #[should_panic]
    fn test_f_invalid() { let _ = Reg::R(Jit::f_num()).to_ffi(); }

    #[test]
    fn test_to_ffi() {
        for n in 0..Jit::r_num() { let _ = Reg::R(n).to_ffi(); }
        for n in 0..Jit::v_num() { let _ = Reg::V(n).to_ffi(); }
        for n in 0..Jit::f_num() { let _ = Reg::F(n).to_ffi(); }
    }
}
