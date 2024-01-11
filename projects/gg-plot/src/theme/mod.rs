use ggplot_core::{Palette, Srgb};

pub struct RotatePalette {
    colors: Vec<Srgb>,
    index: usize,
}

impl Palette for RotatePalette {
    fn next_color(&mut self) -> Srgb {
        let color = self.colors[self.index];
        self.index = (self.index + 1) % self.colors.len();
        color
    }

    fn interpolate(&self, t: f32) -> Srgb {
        let index = (t * (self.colors.len() - 1) as f32) as usize;
        let color = self.colors[index];
        color
    }
}
