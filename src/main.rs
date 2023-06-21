#[macro_use]
extern crate log;
extern crate log4rs;

mod PlcDevice;

use std::ffi::CString;
use PlcDevice::*;

fn main() {
    let mut device = PlcDevice::PlcDevice::new("172.31.1.145".to_string());
    let mut cnt_try = 0;
    while cnt_try < 1000{
        device.create();
        device.connect();
        if device.is_connected{
            println!("Connected!");
            device.write_bit(0, 1, true);
            let mut a = device.read_bit(0, 1);
            println!("read bit: {}", a);
            device.write_vb_safe(0, 0xfa);
            let mut x = device.read_vb(0);
            println!("read vb: {}", x);
            device.write_u16(0, 25000);
            let mut y = device.read_u16(0);
            println!("read u16: {}", y);
            device.write_f32(0, 356.7);
            let mut z = device.read_f32(0);
            println!("read f32: {}", z);
            device.disconnect();
        } else{
            println!("Cannot connected!");
        }
        device.destory();
        cnt_try += 1;
    }

}
