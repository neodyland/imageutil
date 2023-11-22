use image::{
    imageops::{overlay, resize, FilterType},
    GenericImage, GenericImageView, Pixel,
};
use rusttype::{point, Font, GlyphId, PositionedGlyph, Rect, Scale};
use std::{cmp::max, future::Future};

use crate::{canvas::Canvas, util::Mixer};

#[derive(Clone)]
pub struct Fonts<'a> {
    fonts: &'a [Font<'a>],
}

impl<'a> Fonts<'a> {
    pub fn new(fonts: &'a [Font<'a>]) -> Self {
        Self { fonts }
    }

    pub fn get_avalible_font(&self, char: char) -> Option<&'a Font<'a>> {
        self.fonts
            .iter()
            .find(|f| f.glyph(char).id() != GlyphId(0))
            .or(self.fonts.last())
    }
    #[inline(always)]
    pub fn layout_glyphs(
        scale: Scale,
        font: &Font,
        char: char,
        mut f: impl FnMut(PositionedGlyph, Rect<i32>),
    ) -> (i32, i32) {
        let v_metrics = font.v_metrics(scale);
        let pos = font
            .glyph(char)
            .scaled(scale)
            .positioned(point(0.0, v_metrics.ascent));
        let (mut w, mut h) = (0, 0);
        if let Some(bb) = pos.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
            f(pos, bb);
        }
        (w, h)
    }

    #[inline(always)]
    pub fn write_char<C: Canvas>(
        &self,
        img: &mut C,
        char: char,
        scale: Scale,
        color: C::Pixel,
        image_width: i32,
        image_height: i32,
        x: i32,
        y: i32,
        font: &Font,
    ) -> i32
    where
        C::Pixel: Pixel<Subpixel = u8>,
    {
        Self::layout_glyphs(scale, font, char, |pos, bb| {
            pos.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;
                let image_x = gx + x;
                let image_y = gy + y;

                if (0..image_width).contains(&image_x) && (0..image_height).contains(&image_y) {
                    let pixel = img.get_pixel(image_x as u32, image_y as u32);
                    let mixer = Mixer::new(&color, &pixel);
                    let weighted_color = mixer.at(1.0 - gv);
                    img.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            });
        })
        .0 + scale.x as i32 / 15
    }

    pub async fn text_size<'b, Fut>(
        &self,
        text: &'b str,
        scale: Scale,
        resolver: impl Fn(&'b str) -> Fut,
    ) -> (i32, i32)
    where
        Fut: Future<Output = &'b str>,
    {
        let mut width = 0;
        let mut height = 0;
        let text = resolver(text).await;
        for char in text.chars() {
            let font = self.get_avalible_font(char).unwrap();
            let wh = Self::layout_glyphs(scale, font, char, |_, _| {});
            width += wh.0 + scale.x as i32 / 15;
            height = max(height, wh.1);
        }
        (width, height)
    }

    pub async fn write_to_middle<'b, C: Canvas + GenericImage, Fut, Fut2>(
        &self,
        img: &mut C,
        text: &'b str,
        mut scale: Scale,
        color: <C as Canvas>::Pixel,
        width: i32,
        x: i32,
        y: i32,
        resolver: impl Fn(&'b str) -> Fut,
        text_size_resolver: impl Fn(&'b str) -> Fut2,
        shrink: bool,
    ) where
        Fut: Future<Output = Vec<StrOrImg<'a, C>>>,
        Fut2: Future<Output = &'b str>,
        <C as Canvas>::Pixel: Pixel<Subpixel = u8>,
        <C as GenericImageView>::Pixel: 'static,
    {
        let text_size = self.text_size(text, scale, text_size_resolver).await.0;
        let mut newx = x + (width - text_size) / 2;
        if newx < x && shrink {
            let per = width as f32 / text_size as f32;
            scale = Scale {
                x: scale.x * per,
                y: scale.y * per,
            };
            newx = x;
        }
        self.write_to(img, text, scale, color, newx, y, resolver)
            .await;
    }

    pub async fn write_to<'b, C: Canvas + GenericImage + GenericImageView, Fut>(
        &self,
        img: &mut C,
        text: &'b str,
        scale: Scale,
        color: <C as Canvas>::Pixel,
        mut x: i32,
        y: i32,
        resolver: impl Fn(&'b str) -> Fut,
    ) where
        Fut: Future<Output = Vec<StrOrImg<'a, C>>>,
        <C as Canvas>::Pixel: Pixel<Subpixel = u8>,
        <C as GenericImageView>::Pixel: 'static,
    {
        let image_width = GenericImageView::width(img) as i32;
        let image_height = GenericImageView::height(img) as i32;
        let text = resolver(text).await;
        for text in text.iter() {
            match text {
                StrOrImg::Str(text) => {
                    for char in text.chars() {
                        let font = self.get_avalible_font(char).unwrap();
                        x = x + self.write_char(
                            img,
                            char,
                            scale,
                            color,
                            image_width,
                            image_height,
                            x,
                            y,
                            font,
                        );
                    }
                }
                StrOrImg::Img(over) => {
                    let over = resize(
                        over,
                        (scale.x * 0.9) as u32,
                        (scale.y * 0.9) as u32,
                        FilterType::Nearest,
                    );
                    overlay(img, &over, (x + scale.x as i32 / 10) as i64, y as i64);
                    x = x + over.width() as i32 + scale.x as i32 / 5;
                }
            }
        }
    }
}

pub enum StrOrImg<'a, C: Canvas> {
    Str(&'a str),
    Img(C),
}

pub async fn empty_resolver<'a, C: Canvas>(s: &'a str) -> Vec<StrOrImg<C>> {
    vec![StrOrImg::Str(s)]
}

pub async fn empty_size_resolver<'a>(s: &'a str) -> &'a str {
    s
}
