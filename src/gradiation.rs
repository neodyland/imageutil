use image::Pixel;

use crate::canvas::Canvas;

#[derive(Debug, Clone)]
pub struct Gradiation<P: Pixel> {
    pub start_color: P,
    pub end_color: P,
    pub start_point: (u32, u32),
    pub end_point: (u32, u32),
}

struct Mixer<'a, P: Pixel>
where
    P: Pixel<Subpixel = u8>,
{
    pub color1: &'a [P::Subpixel],
    pub color2: &'a [P::Subpixel],
}

impl<'a, P: Pixel> Mixer<'a, P>
where
    P: Pixel<Subpixel = u8>,
{
    pub fn new(color1: &'a P, color2: &'a P) -> Self {
        let color1 = color1.channels();
        let color2 = color2.channels();
        Self {
            color1: &color1,
            color2: &color2,
        }
    }
    pub fn at(&self, p: f32) -> P {
        *P::from_slice(
            self.color1
                .iter()
                .zip(self.color2.iter())
                .map(|(c1, c2)| {
                    let c1 = *c1 as f32;
                    let c2 = *c2 as f32;
                    let c = c1 + (c2 - c1) * p;
                    c as u8
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

pub fn linear_gradient<C: Canvas>(canvas: &mut C, gradiation: &Gradiation<C::Pixel>)
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
