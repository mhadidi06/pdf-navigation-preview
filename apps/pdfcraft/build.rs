//! Windows only: embed the app icon and version info (VERSIONINFO) into `pdfcraft.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/community-icon/navigation-preview.ico");
    println!("cargo:rerun-if-env-changed=PDFCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/community-icon/navigation-preview.ico")
        .set("ProductName", "PDF Navigation Preview")
        .set("FileVersion", "0.1.0.0")
        .set("ProductVersion", "0.1.0.0")
        .set("FileDescription", "Unofficial Community Beta — Windows x64")
        .set("LegalCopyright", "Copyright (c) the PdfCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pdf-navigation-preview.exe")
        .set("InternalName", "pdf-navigation-preview");
    res.set_version_info(winresource::VersionInfo::FILEVERSION, 1u64 << 32);
    res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, 1u64 << 32);
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=pdfcraft.exe built without icon/version resources: {e}");
    }
}
