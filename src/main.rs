
fn main() {
    let code = &[
        0x40, 0x00, 0x80, 0xd2, // mov     x0, #2
        0x21, 0x01, 0x80, 0xd2, // mov     x1, #9
        0x00, 0x00, 0x01, 0x8b, // add     x0, x0, x1
        0xc0, 0x03, 0x5f, 0xd6, // ret
    ];

    let code_len = code.len();
    println!("{:?}", code_len);
    let code = code.as_ptr();

    let mmap_ptr: *mut u8 = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            0x10000,
            libc::PROT_READ | libc::PROT_EXEC | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0
        ) as *mut u8
    };

    unsafe { core::ptr::copy_nonoverlapping(code, mmap_ptr, code_len) }

    println!("Hello, world!");
}
