#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[repr(C)]
struct SubHeader{
    entry: u64,     // virtual address
    data_len: u64,  // programer's length
}

#[repr(C)]
struct Header{
    magic: [u8;16], // anlj
    class: [u8;8],  // anlj64
    endian: u64,    // little/big
    machine: u64,   // architecture
    app_num: u64,   // programer's length
    subheader: [SubHeader; 2],
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let header_start = PLASH_START as *const u8;
    let header_size = core::mem::size_of::<Header>();
    let slice = unsafe {core::slice::from_raw_parts(header_start, header_size)};
    let header_ptr = unsafe {
        slice.as_ptr() as *const Header
    };
    let header = unsafe {
        &*header_ptr
    };

    let app_num = header.app_num;
    let app_start = PLASH_START + header_size;
    println!("apps'num is {app_num}");
    println!("header_size is {header_size}");

    let app0_size = header.subheader[0].data_len as usize;
    let app1_size = header.subheader[1].data_len as usize;

    let app0_start_ptr = app_start as *const u8;
    let app0_start = unsafe{
        &*app0_start_ptr
    };

    let app1_start_ptr = (app_start + app0_size) as *const u8;
    let app1_start = unsafe{
        &*app1_start_ptr
    };

    println!("Load payload ...");

    let code = unsafe { core::slice::from_raw_parts(app0_start, app0_size) };
    println!("app0 size is {}", app0_size);
    println!("app0's content: {:?}: ", code);

    let code = unsafe { core::slice::from_raw_parts(app1_start, app1_size) };
    println!("app1 size is {}", app1_size);
    println!("app1's content: {:?}: ", code);

    println!("Load payload ok!");
}