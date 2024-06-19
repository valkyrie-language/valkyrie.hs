use ggplot_types::{Palette, Srgb};

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

pub struct WolframTheme {
    domain: Vec<Srgb>,
    blender: Vec<Srgb>,
    shift: f32,
    index: usize,
}

impl Palette for WolframTheme {
    fn next_color(&mut self) -> Srgb {
        let color = self.domain[self.index];
        self.index = (self.index + 1) % self.domain.len();
        color
    }

    fn interpolate(&self, t: f32) -> Srgb {
        let index = (t * (self.domain.len() - 1) as f32) as usize;
        let color = self.domain[index];
        color
    }
}

impl RotatePalette {
    /// Wolfram [Business Theme](https://www.wolfram.com/mathematica/new-in-10/plot-themes/business.html).
    ///
    /// Same as `ColorData[106]`
    pub fn business() -> Self {
        Self {
            colors: vec![
                Srgb { r: 0.0955813, g: 0.209225, b: 0.79187, a: 1.0 },
                Srgb { r: 0.935186, g: 0.332231, b: 0.00396887, a: 1.0 },
                Srgb { r: 0.812627, g: 0.0804155, b: 0.0444973, a: 1.0 },
                Srgb { r: 0.170927, g: 0.445323, b: 0.0162669, a: 1.0 },
                Srgb { r: 0.351437, g: 0.0606827, b: 0.444022, a: 1.0 },
                Srgb { r: 0.00512237, g: 0.361362, b: 0.70906, a: 1.0 },
                Srgb { r: 0.181648, g: 0.133678, b: 0.63574, a: 1.0 },
                Srgb { r: 0.872733, g: 0.182899, b: 0.0182361, a: 1.0 },
                Srgb { r: 0.0, g: 0.456263, b: 0.456263, a: 1.0 },
                Srgb { r: 0.658523, g: 0.14678, b: 0.000281927, a: 1.0 },
            ],
            index: 0,
        }
    }
}
