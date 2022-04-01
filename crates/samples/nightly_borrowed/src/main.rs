use windows::{core::*, Win32::Foundation::*};

use std::ops::*;

#[interface("6101df9e-ac33-4f97-9006-b59c131d0eed")]
unsafe trait ICompiler: IUnknown {
    fn compile_the_compiler(&self, compiler: Borrowed<ICompiler>) -> HRESULT;
    fn stamp_exe(&self) -> HRESULT;
}

#[implement(ICompiler)]
struct Compiler();

impl ICompiler_Impl for Compiler {
    unsafe fn compile_the_compiler(&self, compiler: Borrowed<ICompiler>) -> HRESULT {
        if let Some(compiler) = compiler.deref() {
            compiler.stamp_exe()
        } else {
            S_FALSE
        }
    }
    unsafe fn stamp_exe(&self) -> HRESULT {
        S_OK
    }
}

fn main() {
    unsafe {
        let compiler: ICompiler = Compiler().into();
        assert_eq!(S_FALSE, compiler.compile_the_compiler(Borrowed::default()));
        assert_eq!(S_OK, compiler.compile_the_compiler(Borrowed::new(&compiler)));
    }
}
