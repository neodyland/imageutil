use image::Pixel;

pub struct Mixer<'a, P: Pixel>
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
