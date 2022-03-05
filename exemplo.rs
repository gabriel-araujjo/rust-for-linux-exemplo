//! Module example
//! Feito na aula Rust for Linux no IMD em 5/mar/2022
//! 
//! Exemplo simpllificado do scull do LDD

use kernel::file::File;
use kernel::file_operations::FileOperations;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::miscdev;
use kernel::sync::{Ref, RefBorrow};

module! {
    type: Exemplo,
    name: b"exemplo",
    license: b"GPL v2",
}

struct Dispositivo {
    numero: usize,
}

impl Dispositivo {
    fn try_new(numero: usize) -> Result<Ref<Self>> {
        Ref::try_new(Dispositivo { numero })
    }
}

struct Exemplo {
    _dev: Pin<Box<miscdev::Registration<Self>>>,
}

impl FileOperations for Exemplo {
    type OpenData = Ref<Dispositivo>; // device state
    type Wrapper = Ref<Dispositivo>; // file state
    kernel::declare_file_operations!(read, write);

    fn open(data: &Ref<Dispositivo>, _file: &File) -> Result<Self::OpenData> {
        pr_info!("Dispositivo sendo aberto: {}\n", data.numero);
        Ok(data.clone())
    }

    fn read(_this: RefBorrow<'_, Dispositivo>, _file: &File, _data: &mut impl IoBufferWriter, _offset: u64) -> Result<usize> {
        
        Ok(0)
    }

    fn write(_this: RefBorrow<'_, Dispositivo>,  _file: &File, data: &mut impl IoBufferReader,  _offset: u64) -> Result<usize> {
        Ok(data.len())
    }
}

impl KernelModule for Exemplo {
    fn init( _name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Olá mundo\n");
        let reg = miscdev::Registration::new_pinned(fmt!("exemplo"), Dispositivo::try_new(0)?)?;
        Ok(Exemplo { _dev: reg })
    }
}

impl Drop for Exemplo {
    fn drop(&mut self) {
        pr_info!("Adeus mundo\n");
    }
}