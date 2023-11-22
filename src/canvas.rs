use image::{GenericImage, GenericImageView, Pixel};

// from https://github.com/image-rs/imageproc/blob/master/src/drawing/canvas.rs

pub trait Canvas {
    type Pixel: Pixel;
    fn dimensions(&self) -> (u32, u32);
    fn width(&self) -> u32 {
        self.dimensions().0
    }
    fn height(&self) -> u32 {
        self.dimensions().1
    }
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel;
    fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel);
}

impl<I> Canvas for I
where
    I: GenericImage,
{
    type Pixel = I::Pixel;

    fn dimensions(&self) -> (u32, u32) {
        <I as GenericImageView>::dimensions(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        self.get_pixel(x, y)
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel) {
        self.put_pixel(x, y, color)
    }
}

pub struct Blend<I>(pub I);

impl<I: GenericImage> Canvas for Blend<I> {
    type Pixel = I::Pixel;

    fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        self.0.get_pixel(x, y)
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel) {
        let mut pix = self.0.get_pixel(x, y);
        pix.blend(&color);
        self.0.put_pixel(x, y, pix);
    }
}
