use libloading::{Library, Symbol};
use std::os::raw::c_char;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref lib: Library = unsafe{Library::new("plc.dll").unwrap()};
    pub static ref plc_set_param: Symbol<'static, unsafe fn(*mut c_char, i32, i32) -> ()> = unsafe{lib.get(b"plc_set_param").unwrap()};
    pub static ref plc_create: Symbol<'static, unsafe fn() -> ()> = unsafe{lib.get(b"plc_create").unwrap()};
    pub static ref plc_connect: Symbol<'static, unsafe fn() -> (i32)> = unsafe{lib.get(b"plc_connect").unwrap()};
    pub static ref plc_disconnect: Symbol<'static, unsafe fn() -> ()> = unsafe{lib.get(b"plc_disconnect").unwrap()};
    pub static ref plc_destory: Symbol<'static, unsafe fn() -> ()> = unsafe{lib.get(b"plc_destory").unwrap()};
    pub static ref get_ping_timeout: Symbol<'static, unsafe fn() -> (i32)> = unsafe{lib.get(b"get_ping_timeout").unwrap()};
    pub static ref switch_run_mode: Symbol<'static, unsafe fn() -> ()> = unsafe{lib.get(b"switch_run_mode").unwrap()};
    pub static ref get_exist_client: Symbol<'static, unsafe fn() -> (i32)> = unsafe{lib.get(b"get_exist_client").unwrap()};
    pub static ref plc_read_merker: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_merker").unwrap()};
    pub static ref plc_read_input: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_input").unwrap()};
    pub static ref plc_read_output: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_output").unwrap()};
    pub static ref plc_read_timer: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_timer").unwrap()};
    pub static ref plc_read_counter: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_counter").unwrap()};
    pub static ref plc_read_vb: Symbol<'static, unsafe fn(i32) -> (u8)> = unsafe{lib.get(b"plc_read_vb").unwrap()};
    pub static ref plc_read_vw: Symbol<'static, unsafe fn(i32) -> (u16)> = unsafe{lib.get(b"plc_read_vw").unwrap()};
    pub static ref plc_read_vd: Symbol<'static, unsafe fn(i32) -> (u32)> = unsafe{lib.get(b"plc_read_vd").unwrap()};
    pub static ref plc_write_vb: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_vb").unwrap()};
    pub static ref plc_write_vw: Symbol<'static, unsafe fn(i32, u16) -> ()> = unsafe{lib.get(b"plc_write_vw").unwrap()};
    pub static ref plc_write_vd: Symbol<'static, unsafe fn(i32, u32) -> ()> = unsafe{lib.get(b"plc_write_vd").unwrap()};
    pub static ref plc_write_merker: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_merker").unwrap()};
    pub static ref plc_write_output: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_output").unwrap()};
    pub static ref plc_write_input: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_input").unwrap()};
    pub static ref plc_write_timer: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_timer").unwrap()};
    pub static ref plc_write_counter: Symbol<'static, unsafe fn(i32, u8) -> ()> = unsafe{lib.get(b"plc_write_counter").unwrap()};
}

/*
// PlcDll测试代码
    let address = CString::new("172.31.1.145").unwrap().into_raw();

    let mut cnt_try = 0;
    while cnt_try < 1000{
        unsafe{
            plc_set_param(address, 0, 1);
            // 创建Client
            if get_exist_client() < 1{
                plc_create();
            }
            // 连接PLC
            let mut is_connected = plc_connect() > 0;
            if is_connected {
                println!("Connected!");
                switch_run_mode();
                println!("Switch to running mode!");
                plc_write_merker(0, 0xfe);
                let mut x = plc_read_merker(0);
                println!("read merker: {}", x);
                assert_eq!(x, 0xfe);
                let mut y = plc_read_input(0);
                println!("read input: {}", y);
                let mut z = plc_read_output(0);
                println!("read output: {}", z);
                let mut u = plc_read_timer(0);
                println!("read timer: {}", u);
                let mut v = plc_read_counter(0);
                println!("read counter: {}", v);
                plc_write_vb(0, 0xfe);
                let mut w = plc_read_vb(0);
                println!("read vb: {}", w);
                assert_eq!(w, 0xfe);
                plc_write_vw(0, 0xfffe);
                let mut a = plc_read_vw(0);
                println!("read vw: {}", a);
                assert_eq!(a, 0xfffe);
                plc_write_vd(0, 0xfffefffe);
                let mut b = plc_read_vd(0);
                println!("read vd: {}", b);
                assert_eq!(b, 0xfffefffe);
                plc_disconnect();
            } else{
                println!("Cannot connected!");
            }
            if get_exist_client() > 0{
                plc_destory();
            }
        }
        cnt_try += 1;
    }*/