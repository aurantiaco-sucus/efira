pub mod fmt;

use alloc::{format, vec};
use alloc::string::String;
use alloc::vec::Vec;
use core::{mem, ptr};
use core::cmp::{max, min};
use core::ptr::slice_from_raw_parts_mut;
use bytemuck::{cast_mut, cast_slice_mut};
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, Mode, ModeInfo};
use uefi::proto::rng::{Rng, RngAlgorithmType};
use uefi::Result;
use uefi::table::boot::ScopedProtocol;
use uefi::table::runtime::TimeParams;
use crate::helper::fmt::format_size;

static mut EFI_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

#[inline]
pub fn init(mut system_table: SystemTable<Boot>) {
    uefi_services::init(&mut system_table).unwrap();
    unsafe { EFI_SYSTEM_TABLE = Some(system_table); }
    gop_init();
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

static mut GOP_REMOTE_BUFFER: Option<&mut [u32]> = None;
static mut GOP_LOCAL_BUFFER: Vec<u32> = Vec::new();
static mut GOP_DIMENSION: (i32, i32) = (800, 600);

#[inline]
pub fn gop_init() {
    let mut gop = proto_excl::<GraphicsOutput>().unwrap();
    let modes: Vec<Mode> = gop.modes().collect();
    let modes_info: Vec<(usize, usize)> = modes
        .iter()
        .map(|mode| (
            mode.info().resolution().0,
            mode.info().resolution().1))
        .collect();
    info!("Available GOP modes:\n{}", modes_info.iter()
        .map(|(x, y)| format!("W{x} H{y} BUF{}", format_size(x * y * 4)))
        .collect::<Vec<String>>()
        .join("\n"));
    info!("Current GOP mode: {:#?}", gop.current_mode_info());
    if let Some(pos) = modes_info.iter().position(|val| val == &(800, 600)) {
        gop.set_mode(&modes[pos])
            .expect("Error switching resolution to 800x600!");
    } else {
        panic!("Required resolution (800x600) is not available!");
    }
    unsafe {
        let remote_buf = &mut *slice_from_raw_parts_mut(
            gop.frame_buffer().as_mut_ptr(),
            800 * 600 * 4);
        GOP_REMOTE_BUFFER = Some(cast_slice_mut(remote_buf));
        GOP_LOCAL_BUFFER = vec![0_u32; 800 * 600];
    }

    unsafe {
        GOP_LOCAL_BUFFER.fill(0xFFFFFFFF);
    }
    unsafe {
        for i in 0..301 {
            EFI_SYSTEM_TABLE.as_ref().unwrap_unchecked().runtime_services().get_time().unwrap().nanosecond()
            let border = 300 - i;
            gop_blit_area(border, border, 799 - border, 599 - border);
            boot_services().stall(1_000_000 / 60);
        }
    }
    boot_services().stall(1_000_000_000);
}

#[inline]
pub fn gop_blit_area(mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) {
    let (gw, gh) = unsafe { GOP_DIMENSION };
    if x1 > x2 { (x1, x2) = (x2, x1); }
    if y1 > y2 { (y1, y2) = (y2, y1); }
    if x1 < 0 { x1 = 0; }
    if y1 < 0 { y1 = 0; }
    if x2 >= gw { x2 = gw - 1; }
    if y2 >= gh { y2 = gh - 1; }
    let (w, h) = (x2 - x1 + 1, y2 - y1 + 1);
    if w == 0 || h == 0 { return; }
    for li in 0..h {
        unsafe {
            let lb = (GOP_DIMENSION.0 * (y1 + li) + x1) as usize;
            let ub = lb+ w as usize;
            GOP_REMOTE_BUFFER.as_mut().unwrap_unchecked()[lb..ub]
                .copy_from_slice(&GOP_LOCAL_BUFFER[lb..ub]);
        }
    }
}