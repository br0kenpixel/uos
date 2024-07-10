use crate::heapless::strings::StackString;
use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count};
use log::debug;
use request::RequestType;

mod request;

const VENDOR_LENGTH: usize = 12;
const BRAND_LENGTH: usize = (4 * 4) * 3;

pub type Brand = StackString<BRAND_LENGTH>;
pub type Vendor = StackString<VENDOR_LENGTH>;

pub struct CpuInfo {
    brand_string: Brand,
    vendor_string: Vendor,
    physical_cores: u8,
    logical_cores: u8,
}

impl CpuInfo {
    pub fn brand(&self) -> &str {
        &self.brand_string
    }

    pub fn vendor(&self) -> &str {
        &self.vendor_string
    }

    pub fn physical_cores(&self) -> u8 {
        self.physical_cores
    }

    pub fn logical_cores(&self) -> u8 {
        self.logical_cores
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        let mut brand_string: Brand = read_brand_string().into();
        let vendor_string = read_vendor_string().into();

        if brand_string.starts_with(' ') {
            debug!("CPU brand string needs cleaning");

            let trimmed = brand_string.trim_start();
            brand_string = trimmed.try_into().unwrap();
        }

        Self {
            brand_string,
            vendor_string,
            physical_cores: get_physical_cores(),
            logical_cores: get_logical_cores(),
        }
    }
}

fn get_logical_cores() -> u8 {
    let result = safe_cpuid(RequestType::Features);

    ((result.ebx >> 16) & 0xff) as u8
}

fn get_physical_cores() -> u8 {
    let result = safe_cpuid_count(RequestType::Cache, 0);

    (((result.eax >> 26) & 0x3f) + 1) as u8
}

fn read_vendor_string() -> [u8; VENDOR_LENGTH] {
    let result = safe_cpuid(RequestType::Vendor);

    [
        (result.ebx as u8) as u8,
        ((result.ebx >> 8) as u8) as u8,
        ((result.ebx >> 16) as u8) as u8,
        ((result.ebx >> 24) as u8) as u8,
        (result.edx as u8) as u8,
        ((result.edx >> 8) as u8) as u8,
        ((result.edx >> 16) as u8) as u8,
        ((result.edx >> 24) as u8) as u8,
        (result.ecx as u8) as u8,
        ((result.ecx >> 8) as u8) as u8,
        ((result.ecx >> 16) as u8) as u8,
        ((result.ecx >> 24) as u8) as u8,
    ]
}

fn read_brand_string() -> [u8; BRAND_LENGTH] {
    let mut brand_string_bytes = [0; BRAND_LENGTH];

    let first = cpuid_result_to_bytes(safe_cpuid(RequestType::BrandString1));
    let second = cpuid_result_to_bytes(safe_cpuid(RequestType::BrandString2));
    let third = cpuid_result_to_bytes(safe_cpuid(RequestType::BrandString3));

    brand_string_bytes[..16].copy_from_slice(&first);
    brand_string_bytes[16..32].copy_from_slice(&second);
    brand_string_bytes[32..48].copy_from_slice(&third);

    brand_string_bytes
}

fn cpuid_result_to_bytes(res: CpuidResult) -> [u8; 16] {
    let mut result = [0; 4 * 4];

    let bytes_iter = [res.eax, res.ebx, res.ecx, res.edx]
        .into_iter()
        .flat_map(|n: u32| n.to_ne_bytes());

    for (i, byte) in bytes_iter.enumerate() {
        result[i] = byte;
    }

    result
}

fn safe_cpuid(req: RequestType) -> CpuidResult {
    unsafe { __cpuid(req as u32) }
}

fn safe_cpuid_count(req: RequestType, sub_leaf: u32) -> CpuidResult {
    unsafe { __cpuid_count(req as u32, sub_leaf) }
}
