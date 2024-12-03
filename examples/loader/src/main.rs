#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#![cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main(){
    let load_start = PLASH_START as *const u8;
    let load_size = 32;

    println!("Load payload ..");

    let load_code = unsafe {core::slice::from_raw_parts(load_start, load_size)};
    println!("load code {:?}; address [{:?}]", load_code, load_code.as_ptr());
    
    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    let run_code = unsafe {
        core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size)
    };
    run_code.copy_from_slice(load_code);
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    println!("Load payload ok!");
}

