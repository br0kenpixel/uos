#[repr(u32)]
pub(super) enum RequestType {
    Vendor = 0,
    Features = 1,
    Cache = 4,
    BrandString1 = 0x8000_0002,
    BrandString2 = 0x8000_0003,
    BrandString3 = 0x8000_0004,
}
