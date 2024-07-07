use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count};
use request::RequestType;

mod request;

const BRAND_STRING_LENGTH: usize = (4 * 4) * 3;
const VENDOR_STRING_LENGTH: usize = 12;

pub struct CpuInfo {
    brand_string: [u8; 48],
    vendor_string: [u8; 12],
    physical_cores: u8,
    logical_cores: u8,
}

impl CpuInfo {
    pub fn brand_string(&self) -> &str {
        let len = self.brand_string_len();
        let string = core::str::from_utf8(&self.brand_string[..len]).unwrap_or("unknown");

        string
    }

    pub fn vendor_string(&self) -> &str {
        let len = self.vendor_string_len();
        let string = core::str::from_utf8(&self.vendor_string[..len]).unwrap_or("unknown");

        string
    }

    pub fn physical_cores(&self) -> u8 {
        self.physical_cores
    }

    pub fn logical_cores(&self) -> u8 {
        self.logical_cores
    }

    fn vendor_string_len(&self) -> usize {
        self.vendor_string
            .iter()
            .take_while(|entry| *entry != &0)
            .count()
    }

    fn brand_string_len(&self) -> usize {
        self.brand_string
            .iter()
            .take_while(|entry| *entry != &0)
            .count()
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        let brand_string_bytes = read_brand_string();
        let vendor_string_bytes = read_vendor_string();

        Self {
            brand_string: brand_string_bytes,
            vendor_string: vendor_string_bytes,
            physical_cores: get_physical_cores(),
            logical_cores: get_logical_cores(),
        }
    }
}

fn get_logical_cores() -> u8 {
    // Call __cpuid with eax = 1 to get processor info and feature bits
    let result = unsafe { __cpuid(1) };

    // The number of logical processors per physical processor package is given by bits 16-23 of EBX
    ((result.ebx >> 16) & 0xff) as u8
}

fn get_physical_cores() -> u8 {
    // Call __cpuid with eax = 4 and ecx = 0 to get cache and TLB information
    let result = unsafe { __cpuid_count(4, 0) };

    // The number of cores (plus one) is given by bits 26-31 of EAX
    (((result.eax >> 26) & 0x3f) + 1) as u8
}

fn read_vendor_string() -> [u8; VENDOR_STRING_LENGTH] {
    let result = unsafe { __cpuid(0) };

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

fn read_brand_string() -> [u8; BRAND_STRING_LENGTH] {
    let mut brand_string_bytes = [0; BRAND_STRING_LENGTH];

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
