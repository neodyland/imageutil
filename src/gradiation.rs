use image::{GenericImage, ImageBuffer, Pixel};

use crate::{canvas::Canvas, util::Mixer};

#[derive(Debug, Clone)]
pub struct LinearGradiation<P: Pixel> {
    pub start_color: P,
    pub end_color: P,
    pub start_point: (u32, u32),
    pub end_point: (u32, u32),
}

pub fn linear_gradient_mut<C: Canvas>(canvas: &mut C, gradiation: &LinearGradiation<C::Pixel>)
where
    C::Pixel: Clone,
    C::Pixel: Pixel<Subpixel = u8>,
{
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let end_x = if gradiation.end_point.0 > canvas_width {
        canvas_width
    } else {
        gradiation.end_point.0
    };
    let end_y = if gradiation.end_point.1 > canvas_height {
        canvas_height
    } else {
        gradiation.end_point.1
    };
    let start_x = if gradiation.start_point.0 > canvas_width {
        canvas_width
    } else {
        gradiation.start_point.0
    };
    let start_y = if gradiation.start_point.1 > canvas_height {
        canvas_height
    } else {
        gradiation.start_point.1
    };
    let width = end_x - start_x;
    let height = end_y - start_y;
    let f32_width = width as f32;
    let f32_height = height as f32;
    let mixer = Mixer::new(&gradiation.start_color, &gradiation.end_color);
    for x in 0..width {
        for y in 0..height {
            let p = (x as f32 / f32_width + y as f32 / f32_height) / 2.0;
            let point_color = mixer.at(p);
            let x = x + start_x;
            let y = y + start_y;
            canvas.draw_pixel(x, y, point_color);
        }
    }
}

pub fn linear_gradient<C: GenericImage>(
    canvas: &C,
    gradiation: &LinearGradiation<C::Pixel>,
) -> ImageBuffer<C::Pixel, Vec<<C::Pixel as Pixel>::Subpixel>>
where
    C::Pixel: Clone,
    C::Pixel: Pixel<Subpixel = u8>,
{
    let mut out = ImageBuffer::new(canvas.width(), canvas.height());
    out.copy_from(canvas, 0, 0).unwrap();
    linear_gradient_mut(&mut out, gradiation);
    out
}
