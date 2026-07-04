fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }

    // enable KVM acceleration and CPU passtrough on Linux
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/dev/kvm").exists() {
            cmd.arg("-enable-kvm");
            cmd.arg("-cpu").arg("host");
        }

        if std::env::var_os("WSL_DISTRO_NAME").is_some() {
            // no GUI window under WSL; serial is routed to this terminal
            cmd.arg("-nographic");
        } else {
            // keep the framebuffer window, but also mirror serial here
            cmd.args(["-serial", "stdio"]);
        }
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
