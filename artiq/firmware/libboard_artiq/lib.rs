#![feature(lang_items, never_type)]
#![no_std]

extern crate failure;
#[cfg(has_drtio)]
#[macro_use]
extern crate failure_derive;
extern crate byteorder;
extern crate crc;
#[macro_use]
extern crate log;
extern crate io;
extern crate board_misoc;
extern crate proto_artiq;

pub mod spi;

#[cfg(has_kernel_cpu)]
pub mod mailbox;
#[cfg(has_kernel_cpu)]
pub mod rpc_queue;

#[cfg(has_si5324)]
pub mod si5324;

#[cfg(has_grabber)]
pub mod grabber;

#[cfg(has_drtio)]
pub mod drtioaux;
pub mod drtio_routing;
