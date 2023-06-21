mod PlcDll;
mod utils;

use std::thread;
use std::time::Duration;
use std::ffi::CString;
use utils::*;

pub struct PlcDevice {
    pub is_connected: bool,
    rack:i32,
    slot:i32,
    Address:String
}

impl PlcDevice {
    /// 构造函数
    pub fn new(address:String) ->PlcDevice {
        PlcDevice{
            is_connected: false,
            rack:0,
            slot:1,
            Address:address
        }
    }

    /// 检测连接情况,如果连接被断开，需要重新连接
    pub fn detect_connect(&mut self){
        let mut cnt_try = 0;
        while cnt_try <= 3 {
            if self.is_connected {
                return;
            } else{
                self.destory();
                self.create();
                thread::sleep(Duration::from_millis(1000));
                self.connect();
                cnt_try += 1;
            }
        }
        error!(target:"app", "PLC连接断开，且重试多次无效！");
    }

    /// 创建PLC客户端对象
    pub fn create(&self){
        let address = CString::new(self.Address.clone()).unwrap().into_raw();
        unsafe{PlcDll::plc_set_param(address, self.rack, self.slot)};
        unsafe{
            if PlcDll::get_exist_client() < 1 {
                PlcDll::plc_create();
            }
        }
    }

    /// 销毁PLC客户端对象
    pub fn destory(&self){
        unsafe{
            if PlcDll::get_exist_client() > 0 {
                PlcDll::plc_destory();
            }
        }
    }

    /// #### 连接PLC
    ///
    /// 输入：
    /// * rack：0
    /// * slot：1
    /// * Address：IP地址
    /// 输出：
    /// 是否成功连接PLC
    pub fn connect(&mut self){
        let mut connect_flag = false;
        unsafe{
            connect_flag = PlcDll::plc_connect()>0;
            if connect_flag{
                PlcDll::switch_run_mode();
            }
        }
        self.is_connected = connect_flag;
    }

    /// 断开与PLC的连接
    pub fn disconnect(&self){
        unsafe{PlcDll::plc_disconnect()};
    }

    /// 读PLC字节
    pub fn read_vb(&mut self, index:i32) -> u8{
        let mut val:u8 = 0xff;
        if self.is_connected{
            unsafe{val = PlcDll::plc_read_vb(index)};
        }
        return val;
    }

    /// 写PLC字节
    pub fn write_vb(&mut self, index:i32, val:u8){
        self.detect_connect();
        if self.is_connected{
            unsafe{PlcDll::plc_write_vb(index, val)};
        }
    }

    /// 带校验地向PLC写入字节
    pub fn write_vb_safe(&mut self, index:i32, val:u8){
        let mut cnt_retry = 0;
        self.write_vb(index, val);
        while cnt_retry <= 3{
            if self.read_vb(index) == val{
                break;
            } else{
                self.is_connected = false;
                self.write_vb(index, val);
                cnt_retry += 1;
            }
        }
        if cnt_retry > 3{
            self.is_connected = false;
            error!(target:"app", "向PLC写入字节失败！write_vb_safe, index={}, val={}", index, val);
        }
    }

    /// 读PLC字
    fn read_vw(&mut self, index:i32) -> u16 {
        let mut val:u16 = 0xffff;
        if self.is_connected{
            unsafe{val = PlcDll::plc_read_vw(index)};
        }
        return val;
    }

    /// 写PLC字
    fn write_vw(&mut self, index:i32, val:u16){
        self.detect_connect();
        if self.is_connected{
            unsafe{PlcDll::plc_write_vw(index, val)};
        }
    }

    /// 带校验地向PLC写入字
    fn write_vw_safe(&mut self, index:i32, val:u16){
        let mut cnt_retry = 0;
        self.write_vw(index, val);
        while cnt_retry <= 3{
            if self.read_vw(index) == val{
                break;
            } else{
                self.is_connected = false;
                self.write_vw(index, val);
                cnt_retry += 1;
            }
        }
        if cnt_retry > 3{
            self.is_connected = false;
            error!(target:"app", "向PLC写入字失败！write_vw_safe, index={}, val={}", index, val);
        }
    }

    /// 读PLC双字
    fn read_vd(&mut self, index:i32) -> u32 {
        let mut val:u32 = 0xffffffff;
        if self.is_connected{
            unsafe{val = PlcDll::plc_read_vd(index)};
        }
        return val;
    }

    /// 写PLC双字
    fn write_vd(&mut self, index:i32, val:u32){
        self.detect_connect();
        if self.is_connected{
            unsafe{PlcDll::plc_write_vd(index, val)};
        }
    }

    /// 带校验地向PLC写入双字
    fn write_vd_safe(&mut self, index:i32, val:u32){
        let mut cnt_retry = 0;
        self.write_vd(index, val);
        while cnt_retry <= 3{
            if self.read_vd(index) == val{
                break;
            } else{
                self.is_connected = false;
                self.write_vd(index, val);
                cnt_retry += 1;
            }
        }
        if cnt_retry > 3{
            self.is_connected = false;
            error!(target:"app", "向PLC写入双字失败！write_vd_safe, index={}, val={}", index, val);
        }
    }

    /// 读merker
    pub fn read_merker(&mut self, index:i32) -> u8 {
        let mut val:u8 = 0xff;
        if self.is_connected{
            unsafe{val = PlcDll::plc_read_merker(index)};
        }
        return val;
    }

    /// 写merker
    fn write_merker(&mut self, index:i32, val: u8) {
        self.detect_connect();
        if self.is_connected{
            unsafe{PlcDll::plc_write_merker(index, val)};
        }
    }

    /// 带校验地向PLC写入merker
    pub fn write_merker_safe(&mut self, index:i32, val:u8){
        let mut cnt_retry = 0;
        self.write_merker(index, val);
        while cnt_retry <= 3{
            if self.read_merker(index) == val{
                break;
            } else{
                self.is_connected = false;
                self.write_merker(index, val);
                cnt_retry += 1;
            }
        }
        if cnt_retry > 3{
            self.is_connected = false;
            error!(target:"app", "向PLC写入merker失败！write_merker_safe, index={}, val={}", index, val);
        }
    }

    /// 单独地读bit
    pub fn read_bit(&mut self, address:i32, index:u8) -> bool {
        let mut vb = self.read_vb(address);
        return read_bit(vb, index);
    }

    /// 带校验地写bit
    pub fn write_bit(&mut self, address:i32, index:u8, val:bool){
        // 读取经过校验的准确值
        let mut vb = 0;
        let mut cnt_retry = 0;
        while cnt_retry <= 3{
            vb = self.read_vb(address);
            let mut verify = self.read_vb(address);
            if vb == verify && vb != 0xff{
                break;
            }
        }
        if cnt_retry <= 3{
            vb = write_bit(vb, index, val);
            self.write_vb_safe(address, vb);
        } else{
            self.is_connected = false;
            error!(target:"app", "始终读取不到正确的值！write_bit, address={}, index={}, val={}", address, index, val);
        }
    }

    /// 读取U16整数
    pub fn read_u16(&mut self, index:i32) -> u16 {
        let buf = self.read_vw(index);
        return vw_to_u16(buf);
    }

    /// 写入U16整数
    pub fn write_u16(&mut self, index:i32, val:u16){
        let buf = u16_to_vw(val);
        self.write_vw_safe(index, buf);
    }

    /// 读取F32浮点数
    pub fn read_f32(&mut self, index:i32) -> f32 {
        let buf = self.read_vd(index);
        return vd_to_f32(buf);
    }

    /// 写入F32浮点数
    pub fn write_f32(&mut self, index:i32, val:f32){
        let buf = f32_to_vd(val);
        self.write_vd_safe(index, buf);
    }
}