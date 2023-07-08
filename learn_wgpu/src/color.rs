pub struct RGBA {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl RGBA {
    pub fn to_HSLA(self) -> HSLA {
        let RGBA { r, g, b, a } = self;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let l = (max + min) / 2.0;

        let s = if max == min {
            0.0
        } else if l <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        let h = if max == min {
            0.0
        } else if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };

        let h = if h < 0.0 { h + 6.0 } else { h };

        let h = h / 6.0;

        HSLA { h, s, l, a }
    }
}
impl From<wgpu::Color> for RGBA {
    fn from(value: wgpu::Color) -> Self {
        let wgpu::Color { r, g, b, a } = value;
        Self { r, g, b, a }
    }
}
impl Into<wgpu::Color> for RGBA {
    fn into(self) -> wgpu::Color {
        let RGBA { r, g, b, a } = self;
        wgpu::Color { r, g, b, a }
    }
}
pub struct HSLA {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}
impl HSLA {
    pub fn to_RGBA(self) -> RGBA {
        let HSLA { h, s, l, a } = self;
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - (h * 6.0 % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 1.0 / 6.0 {
            (c, x, 0.0)
        } else if h < 2.0 / 6.0 {
            (x, c, 0.0)
        } else if h < 3.0 / 6.0 {
            (0.0, c, x)
        } else if h < 4.0 / 6.0 {
            (0.0, x, c)
        } else if h < 5.0 / 6.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = r + m;
        let g = g + m;
        let b = b + m;

        RGBA { r, g, b, a }
    }
}
