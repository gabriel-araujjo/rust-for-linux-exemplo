//! Module example
//! Feito na aula Rust for Linux no IMD em 5/mar/2022

use kernel::prelude::*;

module! {
    type: Exemplo,
    name: b"exemplo",
    license: b"GPL v2",
}

struct Exemplo;

impl KernelModule for Exemplo {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        Ok(Exemplo)
    }
}