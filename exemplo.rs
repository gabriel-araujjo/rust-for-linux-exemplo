//! Module example
//! Feito na aula Rust for Linux no IMD em 5/mar/2022
//! 
//! Exemplo simpllificado do scull do LDD

use kernel::file::File;
use kernel::file_operations::FileOperations;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::miscdev;

module! {
    type: Exemplo,
    name: b"exemplo",
    license: b"GPL v2",
}

struct Exemplo {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

impl FileOperations for Exemplo {
    type OpenData = ();
    kernel::declare_file_operations!(read, write);

    fn open(_data: &(), _file: &File) -> Result {
        pr_info!("Dispositivo sendo aberto\n");
        Ok(())
    }

    fn read(_this: (), _file: &File, _data: &mut impl IoBufferWriter, _offset: u64) -> Result<usize> {
        
        Ok(0)
    }

    fn write(_this: (),  _file: &File, data: &mut impl IoBufferReader,  _offset: u64) -> Result<usize> {
        Ok(data.len())
    }
}

impl KernelModule for Exemplo {
    fn init( _name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Olá mundo\n");
        let reg = miscdev::Registration::new_pinned(fmt!("exemplo"), ())?;
        Ok(Exemplo { _dev: reg })
    }
}

impl Drop for Exemplo {
    fn drop(&mut self) {
        pr_info!("Adeus mundo\n");
    }
}