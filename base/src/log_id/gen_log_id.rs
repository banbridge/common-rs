use std::{mem::MaybeUninit, net::IpAddr, sync::LazyLock};

use get_if_addrs::get_if_addrs;
use rand::{RngExt, rngs::ThreadRng};

use crate::context::LogId;

const VERSION: &str = "01";

const CHARSET: &[u8] = b"0123456789ABCDEF";

#[derive(Default)]

pub struct LogIdGenerator {}

pub static LOG_ID: LazyLock<LogIdGenerator> = LazyLock::new(LogIdGenerator::default);

static IP: LazyLock<String> = LazyLock::new(format_ip);

pub fn gen_log_id() -> LogId {
    LOG_ID.generate()
}

impl LogIdGenerator {
    // 生成一个新的 LogID
    pub fn generate(&self) -> LogId {
        const LEN: usize = 63;

        let mut buf: MaybeUninit<[u8; LEN]> = MaybeUninit::uninit();

        let mut index = 0;

        unsafe {
            use std::ptr;

            // [2] 02
            ptr::copy_nonoverlapping(VERSION.as_ptr(), buf.as_mut_ptr().cast(), 2);

            index += 2;

            // [13] time
            let mut itoa_buf = itoa::Buffer::new();

            let t = itoa_buf.format(coarsetime::Clock::now_since_epoch().as_millis());

            ptr::copy_nonoverlapping(t.as_ptr(), buf.as_mut_ptr().cast::<u8>().add(index), 13);

            index += 13;

            // [32] ip
            ptr::copy_nonoverlapping(IP.as_ptr(), buf.as_mut_ptr().cast::<u8>().add(index), 32);

            index += 32;

            // [6] random hex
            let random_bytes = self.random_part();

            ptr::copy_nonoverlapping(
                random_bytes.as_ptr(),
                buf.as_mut_ptr().cast::<u8>().add(index),
                16,
            );
        }

        unsafe { faststr::FastStr::new(std::str::from_utf8_unchecked(&buf.assume_init())) }.into()
    }

    // 生成随机部分 (16 位)
    fn random_part(&self) -> String {
        let mut rng = ThreadRng::default();

        (0..16)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());

                CHARSET[idx] as char
            })
            .collect()
    }
}

/// Format ip to 32 byte string.
/// 16 byte ipv6 to hex -> 32 byte
/// 0,0,0,0,0,0,0,0,0,0,ff,ff,i,p,v,4 to hex -> 32 byte

pub(crate) fn format_ip() -> String {
    let ip_addr = get_local_ip();

    match ip_addr {
        None => "00000000000000000000000000000000".to_string(),
        Some(ip_addr) => match ip_addr {
            IpAddr::V4(v4) => {
                let ostets = v4.octets();

                format!("00000000000000000000ffff{}", hex::encode(ostets.as_ref()))
            }
            IpAddr::V6(v6) => {
                let seg = v6.segments();

                let seg = unsafe { std::mem::transmute::<[u16; 8], [u8; 16]>(seg) };

                hex::encode(seg.as_ref())
            }
        },
    }
}

// 获取本地 IPv6 地址
fn get_local_ip() -> Option<IpAddr> {
    // 遍历所有网络接口地址
    if let Ok(if_addrs) = get_if_addrs() {
        for if_addr in if_addrs {
            // 检查是否为 IPv6 地址且非回环
            if !if_addr.is_loopback() {
                return Some(if_addr.addr.ip());
            }
        }
    }

    None
}

#[cfg(test)]

mod tests {

    use std::ops::Deref;

    use super::*;

    #[test]
    fn test_log_id() {
        let log_id = LOG_ID.generate();

        println!("log_id: {}", log_id.deref());
        //assert_eq!(log_id.len(), 53);
    }

    // 基准测试

    #[test]

    fn byted_log_id() {}
}
