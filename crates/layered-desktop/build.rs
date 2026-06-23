//! Build script for layered-desktop.
//!
//! Checks for the required WebKit2GTK system library on Linux and prints
//! an actionable install command when it is missing, so users see a helpful
//! message instead of the raw pkg-config failure from webkit2gtk-sys.

fn main() {
    // Only relevant on Linux; macOS and Windows provide their own WebView.
    #[cfg(target_os = "linux")]
    check_linux_deps();
}

#[cfg(target_os = "linux")]
fn check_linux_deps() {
    let libs = [
        ("webkit2gtk-4.1", "webkit2gtk-4.1 >= 2.40"),
        ("javascriptcoregtk-4.1", "javascriptcoregtk-4.1 >= 2.38"),
    ];

    let mut missing = Vec::new();

    for (name, version_req) in &libs {
        let ok = std::process::Command::new("pkg-config")
            .args(["--exists", version_req])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !ok {
            missing.push(*name);
        }
    }

    if !missing.is_empty() {
        // cargo:warning lines appear in the build output before the full
        // compiler error, giving the user an immediate fix.
        println!("cargo:warning=");
        println!("cargo:warning=╔══════════════════════════════════════════════════════════════╗");
        println!("cargo:warning=║  MISSING SYSTEM LIBRARIES — layered-desktop cannot build     ║");
        println!("cargo:warning=╠══════════════════════════════════════════════════════════════╣");
        println!("cargo:warning=║  The following pkg-config packages were not found:            ║");
        for lib in &missing {
            println!("cargo:warning=║    • {lib:<57}║");
        }
        println!("cargo:warning=╠══════════════════════════════════════════════════════════════╣");
        println!("cargo:warning=║  Debian / Ubuntu:                                            ║");
        println!("cargo:warning=║    sudo apt install libwebkit2gtk-4.1-dev \\                  ║");
        println!("cargo:warning=║                     libjavascriptcoregtk-4.1-dev \\           ║");
        println!("cargo:warning=║                     libgtk-3-dev libxdo-dev libssl-dev       ║");
        println!("cargo:warning=║                                                              ║");
        println!("cargo:warning=║  Fedora / RHEL:                                              ║");
        println!("cargo:warning=║    sudo dnf install webkit2gtk4.1-devel \\                    ║");
        println!("cargo:warning=║                     javascriptcoregtk4.1-devel \\             ║");
        println!("cargo:warning=║                     gtk3-devel openssl-devel                 ║");
        println!("cargo:warning=║                                                              ║");
        println!("cargo:warning=║  Arch Linux:                                                 ║");
        println!("cargo:warning=║    sudo pacman -S webkit2gtk-4.1 gtk3 openssl xdotool        ║");
        println!("cargo:warning=║                                                              ║");
        println!("cargo:warning=║  See PLATFORMS.md for more detail and troubleshooting.       ║");
        println!("cargo:warning=╚══════════════════════════════════════════════════════════════╝");
        println!("cargo:warning=");
    }

    // Always tell Cargo to re-run this script if pkg-config availability changes.
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR");
}
