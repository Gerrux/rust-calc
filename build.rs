fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();

        // Application icon
        res.set_icon("assets/icon.ico");

        // Version info
        res.set("ProductName", "Rust Calculator");
        res.set("FileDescription", "Lightweight scientific calculator");
        res.set("LegalCopyright", "Copyright © 2025 gerrux. MIT License.");
        res.set("CompanyName", "gerrux");
        res.set("OriginalFilename", "rust-calc.exe");
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));

        res.compile().expect("Failed to compile Windows resources");
    }
}
