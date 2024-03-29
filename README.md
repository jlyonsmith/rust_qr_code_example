# QR Code Rust Example

This is an example of how to decode and encode QR codes using the currently available Rust crates. It's a good idea to read [How QR codes work](https://www.thonky.com/qr-code-tutorial/) for some background.  Also, this YouTube video on [How QR Codes Are Built](https://www.youtube.com/watch?v=142TGhaTMtI) is the best I have found.

The app uses the [image](https://docs.rs/image/latest/image/) to read and write image files.

For decoding I tried a bunch of crates that showed up on Google and the [`quircs`](https://docs.rs/quircs/latest/quircs/) crate seems to work best.  All the decoders I tried are really slow if the image is too big, and often failed to detect the QR code, so down sampling the image to around ~500 wide is a good idea.  In Rust you can down sample with [`ispc-downsampler`](https://lib.rs/crates/ispc-downsampler). For this example, I just used [Acorn on macOS](https://flyingmeat.com/acorn/) for a one time down sample, which is in the examples directory.

Rust has [many QR code encoding crates](https://crates.io/search?q=qrcode).  After some research, I found a lot of them encode using the UTF-8 character encoding, and not ISO 8859-1 (Windows 1252).  Although most libraries correctly decect UTF-8, the QR Code specification indicates that ISO 8859-1 is the intended character encoding. Take a look at the codes generated by [qrcodegen](https://crates.io/crates/qrcodegen) which does it correctly.  I discovered this by capturing QR codes from around town and trying to round trip decode then re-encode them. The [`encoding_rs`](https://docs.rs/encoding_rs/0.8.33/encoding_rs/index.html#iso-8859-1) crate explains more about different character encoding schemes.  Most likely, the difference is because to encode UTF-8 in a QR code you need to use a [Extended Channel Interpretation (ECI)](https://www.bardecode.com/en1/qr-codes-and-utf-8-encoding/) to wrap the data.

The `qrcodegen` project has a good [web interface](https://www.nayuki.io/page/qr-code-generator-library) for testing.
