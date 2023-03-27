#![no_main]
#![no_std]

extern crate alloc;

mod helper;
mod verse;

use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    info!("Efira by Midnight233, Version 0.1.0");
    helper::init(system_table);
    Status::SUCCESS
}