#[repr(u32)]
pub(super) enum RequestType {
    Vendor = 0,
    Features = 1,
    Cache = 4,
    BrandString1 = 0x80000002,
    BrandString2 = 0x80000003,
    BrandString3 = 0x80000004,
}
