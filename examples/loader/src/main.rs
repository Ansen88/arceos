#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[repr(C)]
struct Header{
    magic: [u8;16], // anlj
    class: [u8;8],  // anlj64
    endian: u64,    // little/big
    machine: u64,   // architecture
    data_len: u64,   // programer's length
    entry: u64,   // virtual address
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let header_start = PLASH_START as *const u8;
    let header_size = core::mem::size_of::<Header>();
    let slice = unsafe {core::slice::from_raw_parts(header_start, header_size)};
    let header_ptr = unsafe {
        slice.as_ptr() as *const Header
    };

    let apps_start = (PLASH_START + header_size) as *const u8;
    let header = unsafe {
        &*header_ptr
    };
    let apps_size = header.data_len as usize; // Dangerous!!! We need to get accurate size of apps.

    println!("Load payload ...");

    let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    println!("content: {:?}: ", code);

    println!("Load payload ok!");
}