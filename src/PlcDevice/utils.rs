extern crate byte;

use byte::*;

/// 从一个字节中获得相应的位
pub fn read_bit(val:u8, index:u8) -> bool {
    let mask = 1<<index;
    return if (val & mask) != 0 {
        true
    } else {
        false
    }
}

/// 向一个字节中的某一位写入
pub fn write_bit(val:u8, index:u8, bitstate:bool) -> u8 {
    if bitstate {
        let mask = 1<<index;
        return val|mask;
    }
    else{
        let mask = !(1<<index);
        return val&mask;
    }
}

/// 从一个双字中解析浮点数
pub fn vd_to_f32(input:u32) -> f32 {
    let mut bytes_g = [0xffu8; 4];
    let offset = &mut 0;
    bytes_g.write_with(offset, input, LE).unwrap();
    let offset = &mut 0;
    let output:f32 = bytes_g.read_with::<f32>(offset, BE).unwrap();
    return output;
}

/// 将一个浮点数编码为一个双字
pub fn f32_to_vd(val:f32) -> u32 {
    let mut bytes_g = [0xffu8; 4];
    let offset = &mut 0;
    bytes_g.write_with(offset, val, BE).unwrap();
    let offset = &mut 0;
    let output:u32 = bytes_g.read_with::<u32>(offset, LE).unwrap();
    return output;
}

/// 从一个字中解析16位整数
pub fn vw_to_u16(input:u16) -> u16 {
    let mut bytes_g = [0xffu8; 2];
    let offset = &mut 0;
    bytes_g.write_with(offset, input, LE).unwrap();
    let offset = &mut 0;
    let output:u16 = bytes_g.read_with::<u16>(offset, BE).unwrap();
    return output;
}

/// 将一个16位整数编码为一个字
pub fn u16_to_vw(val:u16) -> u16 {
    let mut bytes_g = [0xffu8; 2];
    let offset = &mut 0;
    bytes_g.write_with(offset, val, BE).unwrap();
    let offset = &mut 0;
    let output:u16 = bytes_g.read_with::<u16>(offset, LE).unwrap();
    return output;
}
