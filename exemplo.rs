//! Module example
//! Feito na aula Rust for Linux no IMD em 5/mar/2022
//! 
//! Exemplo simpllificado do scull do LDD

use kernel::file::File;
use kernel::file_operations::FileOperations;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::miscdev;
use kernel::sync::{Ref, RefBorrow, Mutex, UniqueRef};

module! {
    type: Exemplo,
    name: b"exemplo",
    license: b"GPL v2",
}

struct  Conteudo {
    dados: Vec<u8>,
}

struct Dispositivo {
    numero: usize,
    conteudo: Mutex<Conteudo>,
}

impl Dispositivo {
    fn try_new(numero: usize) -> Result<Ref<Self>> {
        let mut dev = Pin::from(UniqueRef::try_new(
            Dispositivo { 
                numero,
                // SAFETY: `mutex_init` é chamado a baixo
                conteudo: unsafe { 
                    Mutex::new(Conteudo {
                        dados: Vec::new(),
                    }) 
                },
            }
        )?);
        
        // SAFETY: Conteudo está fixado quando dispositivo também está
        let m = unsafe {
            dev.as_mut().map_unchecked_mut(|d| &mut d.conteudo)
        };

        kernel::mutex_init!(m, "dev::content");

        return Ok(dev.into());
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

    fn read(this: RefBorrow<'_, Dispositivo>, _file: &File, data: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        let offset = offset.try_into()?;
        let guard = this.conteudo.lock();
        let len = core::cmp::min(data.len(), guard.dados.len().saturating_sub(offset));

        data.write_slice(&guard.dados[offset..][..len])?;
        Ok(len)
    }

    fn write(this: RefBorrow<'_, Dispositivo>,  _file: &File, data: &mut impl IoBufferReader,  _offset: u64) -> Result<usize> {
        let copia = data.read_all()?;
        let len = copia.len();
        this.conteudo.lock().dados = copia;
        Ok(len)
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