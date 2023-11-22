use image::{DynamicImage, Rgba};
use imageutil::gradiation::{linear_gradient_mut, LinearGradiation};

fn main() {
    let mut img = DynamicImage::new_rgba8(100, 100);
    for _ in 0..100 {
        linear_gradient_mut(
            &mut img,
            &LinearGradiation {
                start_color: Rgba([0, 0, 255, 255]),
                end_color: Rgba([255, 255, 255, 255]),
                start_point: (0, 0),
                end_point: (100, 100),
            },
        );
    }
    img.save("examples/gradation.png").unwrap();
}
