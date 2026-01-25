use system_error::SystemError;

use crate::arch::interrupt::TrapFrame;
use crate::syscall::table::FormattedSyscallParam;
use crate::syscall::table::Syscall;
use alloc::string::ToString;
use alloc::vec::Vec;
use log::info;


pub struct Sys2333Handle;

pub const Sys2333: usize = 2333;

impl Syscall for Sys2333Handle {
    fn num_args(&self) -> usize {
        0
    }

    fn handle(&self, _args: &[usize], _frame: &mut TrapFrame) -> Result<usize, SystemError> {
        info!("syscall 2333 called");

        let ret: usize = 6666;

        Ok(ret)
    }

    fn entry_format(&self, _args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![FormattedSyscallParam::new(
            "No arguments",
            "2333()".to_string(),
        )]
    }
}

syscall_table_macros::declare_syscall!(Sys2333, Sys2333Handle);