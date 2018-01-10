Memory-render Example
=====================

This is a small command-line utility for PDF/PS-to-PNG conversion.

Though it could use built-in Ghostscript image output
e.g. with "pnggray" device and "-sOutputFile" setting,
instead it saves images manually using the image crate,
in order to demonstrate the technique of PDF/PS rendering
into in-memory buffer with the help of Ghostscript's
"display" device functionality.

Usage
=====

See `cargo run --example memory-render -- --help`
