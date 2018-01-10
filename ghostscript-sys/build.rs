fn main() {
    // The sane default on linux systems.
    println!("cargo:rustc-link-lib=gs");

    // You can override ghostscript library name/location using
    // [target.<your triple>.ghostscript] section in ".cargo/config".
    // See cargo docs for "links" attribute for how.

    // Maybe this build script should download precompiled ghostscript
    // or build it from sources. Suggestions of better ways to do that are welcome.
}
