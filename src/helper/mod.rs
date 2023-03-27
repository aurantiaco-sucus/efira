pub mod gop;
pub mod fmt;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, Mode, ModeInfo};
use uefi::Result;
use uefi::table::boot::ScopedProtocol;

static mut EFI_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

#[inline]
pub fn init(mut system_table: SystemTable<Boot>) {
    uefi_services::init(&mut system_table).unwrap();
    unsafe { EFI_SYSTEM_TABLE = Some(system_table); }
    init_gop();
}

#[inline]
pub fn boot_services() -> &'static BootServices {
    unsafe { EFI_SYSTEM_TABLE.as_ref().unwrap_unchecked().boot_services() }
}

#[inline]
pub fn handle_of<T: uefi::proto::Protocol>() -> Result<Handle> {
    boot_services().get_handle_for_protocol::<T>()
}

#[inline]
pub fn proto_excl<T: uefi::proto::Protocol>() -> Result<ScopedProtocol<'static, T>> {
    boot_services().open_protocol_exclusive::<T>(handle_of::<T>()?)
}

static mut GOP_BUFFER: Option<&mut [u32]> = None;

#[inline]
pub fn init_gop() {
    let gop = proto_excl::<GraphicsOutput>().unwrap();
    let modes: Vec<Mode> = gop.modes().collect();
    let modes_info: Vec<(usize, usize)> = modes
        .iter()
        .map(|mode| (
            mode.info().resolution().0,
            mode.info().resolution().1))
        .collect();
    info!("Available GOP modes:\n{}", modes_info.iter()
        .map(|(x, y)| format!("W{x} H{y} BUF{}", x * y * 4))
        .collect::<Vec<String>>()
        .join("\n"));
}