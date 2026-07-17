use core::arch::x86_64::{__cpuid, __cpuid_count, CpuidResult};

const VENDOR_LENGTH: usize = 12;
const BRAND_LENGTH: usize = (4 * 4) * 3;

#[repr(u32)]
pub(super) enum CpuidRequest {
    Vendor = 0,
    Features = 1,
    Cache = 4,
    BrandString1 = 0x8000_0002,
    BrandString2 = 0x8000_0003,
    BrandString3 = 0x8000_0004,
    TscInvariantStatus = 0x8000_0007,
}

pub fn get_logical_cores() -> u8 {
    let result = safe_cpuid(CpuidRequest::Features);

    ((result.ebx >> 16) & 0xff) as u8
}

pub fn get_physical_cores() -> u8 {
    let result = safe_cpuid_count(CpuidRequest::Cache, 0);

    (((result.eax >> 26) & 0x3f) + 1) as u8
}

pub fn get_hyperthreading() -> bool {
    get_logical_cores() == (get_physical_cores() * 2)
}

pub fn read_vendor_string() -> heapless::String<VENDOR_LENGTH> {
    let result = safe_cpuid(CpuidRequest::Vendor);
    let mut vendor = heapless::String::new();

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
    .into_iter()
    .for_each(|byte| {
        vendor.push(char::from(byte)).unwrap();
    });

    vendor
}

pub fn read_brand_string() -> heapless::String<BRAND_LENGTH> {
    let mut brand_string_bytes = heapless::Vec::<u8, BRAND_LENGTH>::new();

    'outer: for request in [
        CpuidRequest::BrandString1,
        CpuidRequest::BrandString2,
        CpuidRequest::BrandString3,
    ] {
        let mut last_was_space = false;

        for result_byte in cpuid_result_to_bytes(safe_cpuid(request)) {
            if result_byte == 0 {
                break 'outer;
            }

            if result_byte == b' ' {
                if last_was_space {
                    brand_string_bytes.pop();
                    break 'outer;
                }
                last_was_space = true;
            } else if last_was_space {
                last_was_space = false;
            }

            let _ = brand_string_bytes.push(result_byte);
        }
    }

    unsafe { heapless::String::from_utf8_unchecked(brand_string_bytes) }
}

pub fn has_invariant_tsc() -> bool {
    let res = safe_cpuid(CpuidRequest::TscInvariantStatus);

    // read the 8th bit of the EDX register
    res.edx & (1 << 8) != 0
}

pub fn hypervisor_present() -> bool {
    let res = safe_cpuid(CpuidRequest::Features);

    // read the 31st bit of the ECX register
    (res.ecx & (1 << 31)) != 0
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

fn safe_cpuid(req: CpuidRequest) -> CpuidResult {
    __cpuid(req as u32)
}

fn safe_cpuid_count(req: CpuidRequest, sub_leaf: u32) -> CpuidResult {
    __cpuid_count(req as u32, sub_leaf)
}
