use image::{DynamicImage, Rgba};
use imageutil::text::{empty_resolver, Fonts};
use once_cell::sync::Lazy;
use rusttype::{Font, Scale};

const NOTO_SANS_JP: Lazy<Font> =
    Lazy::new(|| Font::try_from_bytes(include_bytes!("./NotoSansJP-Regular.ttf")).unwrap());

#[tokio::main]
async fn main() {
    let mut img = DynamicImage::new_rgba8(1000, 1000);
    let fonts = &[NOTO_SANS_JP.clone()];
    let fonts = Fonts::new(fonts);
    for _ in 0..100 {
        fonts
            .write_to(
                &mut img,
                "hello",
                Scale::uniform(200.0),
                Rgba([0, 0, 255, 255]),
                0,
                0,
                empty_resolver,
            )
            .await;
    }
    img.save("examples/font.png").unwrap();
}
