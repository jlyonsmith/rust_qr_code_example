use qrcodegen::{Mask, QrCode, QrCodeEcc, QrSegment, Version};
use quircs;

fn main() {
    // open the image from disk
    let img = image::open("./example_downsample.jpg").unwrap();

    // convert to gray scale
    let img_gray = img.into_luma8();

    // create a decoder
    let mut decoder = quircs::Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(
        img_gray.width() as usize,
        img_gray.height() as usize,
        &img_gray,
    );

    for code in codes {
        let code = code.expect("failed to extract qr code");
        let decoded = code.decode().expect("failed to decode qr code");
        let content = std::str::from_utf8(&decoded.payload).unwrap();

        println!("version: {}", decoded.version);
        println!("ecc_level: {:?}", decoded.ecc_level);
        println!("mask: {:?}", decoded.mask);
        println!(
            "data_type: {:?}",
            decoded.data_type.unwrap_or(quircs::DataType::Byte)
        );
        println!("content: {}", content);

        let code = QrCode::encode_segments_advanced(
            // &[QrSegment::make_bytes(&WINDOWS_1252.encode(content).0)],
            &[QrSegment::make_bytes(content.as_bytes())],
            QrCodeEcc::High,
            Version::new(6),
            Version::new(6),
            Some(Mask::new(4)),
            false,
        )
        .unwrap();
        let new_img = qrcodegen_image::draw_canvas(code);

        new_img.save("./qrcode.png").unwrap();
    }
}
