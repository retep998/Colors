
mod tables;

static SRGB: ColorSpace = ColorSpace {
    r: ColorXyy {
        x: 0.6400,
        y: 0.3300,
        Y: 0.2126,
    },
    g: ColorXyy {
        x: 0.3000,
        y: 0.6000,
        Y: 0.7152,
    },
    b: ColorXyy {
        x: 0.1500,
        y: 0.0600,
        Y: 0.0722,
    },
    w: ColorXyy {
        x: 0.3127,
        y: 0.3290,
        Y: 1.0000,
    },
};

#[deriving(Show)]
struct ColorSpace {
    r: ColorXyy,
    g: ColorXyy,
    b: ColorXyy,
    w: ColorXyy,
}

#[deriving(Show)]
struct Color3<T, U>(T, T, T);
#[deriving(Show)]
struct Color4<T, U>(T, T, T, T);

#[deriving(Show)]
struct ColorXyz {
    x: f64,
    y: f64,
    z: f64,
}

impl ColorXyz {
    fn from_wavelength(wavelength: uint) -> ColorXyz {
        use self::tables::CIE_COLOR_MATCH;
        match CIE_COLOR_MATCH.get(wavelength - 390) {
            Some(c) => ColorXyz {
                x: c[0],
                y: c[1],
                z: c[2],
            },
            None => ColorXyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    fn to_rgb(&self, cs: &ColorSpace) -> ColorRgbF64 {
        let (xc, yc, zc) = (self.x, self.y, self.z);
        let (xr, yr, zr) = (cs.r.x, cs.r.y, 1. - (cs.r.x + cs.r.y));
        let (xg, yg, zg) = (cs.g.x, cs.g.y, 1. - (cs.g.x + cs.g.y));
        let (xb, yb, zb) = (cs.b.x, cs.b.y, 1. - (cs.b.x + cs.b.y));
        let (xw, yw, zw) = (cs.w.x, cs.w.y, 1. - (cs.w.x + cs.w.y));
        let (rx, ry, rz) = (yg * zb - yb * zg, xb * zg - xg * zb, xg * yb - xb * yg);
        let (gx, gy, gz) = (yb * zr - yr * zb, xr * zb - xb * zr, xb * yr - xr * yb);
        let (bx, by, bz) = (yr * zg - yg * zr, xg * zr - xr * zg, xr * yg - xg * yr);
        let rw = (rx * xw + ry * yw + rz * zw) / yw;
        let gw = (gx * xw + gy * yw + gz * zw) / yw;
        let bw = (bx * xw + by * yw + bz * zw) / yw;
        let (rx, ry, rz) = (rx / rw, ry / rw, rz / rw);
        let (gx, gy, gz) = (gx / gw, gy / gw, gz / gw);
        let (bx, by, bz) = (bx / bw, by / bw, bz / bw);
        ColorRgbF64 {
            r: rx * xc + ry * yc + rz * zc,
            g: gx * xc + gy * yc + gz * zc,
            b: bx * xc + by * yc + bz * zc,
        }
    }
    fn normalize(&self) -> ColorXyz {
        let m = self.x.max(self.y).max(self.z);
        ColorXyz {
            x: self.x.div(&m),
            y: self.y.div(&m),
            z: self.z.div(&m),
        }
    }
}

#[allow(uppercase_variables)]
#[deriving(Show)]
struct ColorXyy {
    x: f64,
    y: f64,
    Y: f64,
}

#[deriving(Show)]
struct ColorRgbU8 {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRgbU8 {
    fn to_float(&self) -> ColorRgbF64 {
        ColorRgbF64 {
            r: self.r as f64 / 255.,
            g: self.g as f64 / 255.,
            b: self.b as f64 / 255.,
        }
    }
}

#[deriving(Show)]
struct ColorRgbF64 {
    r: f64,
    g: f64,
    b: f64,
}

impl ColorRgbF64 {
    fn to_int(&self) -> ColorRgbU8 {
        ColorRgbU8 {
            r: (self.r.min(1.).max(0.) * 255.).round() as u8,
            g: (self.g.min(1.).max(0.) * 255.).round() as u8,
            b: (self.b.min(1.).max(0.) * 255.).round() as u8,
        }
    }
    fn luminance(&self, cs: &ColorSpace) -> f64 {
        self.r * cs.r.Y + self.g * cs.g.Y + self.b * cs.b.Y
    }
    fn normalize(&self) -> ColorRgbF64 {
        let m = self.r.max(self.g).max(self.b);
        ColorRgbF64 {
            r: self.r.div(&m),
            g: self.g.div(&m),
            b: self.b.div(&m),
        }
    }
    fn constrain(&self) -> ColorRgbF64 {
        let w = 0f64.min(self.r).min(self.g).min(self.b);
        ColorRgbF64 {
            r: self.r - w,
            g: self.g - w,
            b: self.b - w,
        }
    }
    fn encode_srgb(&self) -> ColorRgbF64 {
        fn encode(x: f64) -> f64 {
            if x <= 0.0031308 {
                x * 12.92
            } else {
                x.powf(2.4f64.recip()) * (1. + 0.055) - 0.055
            }
        }
        ColorRgbF64 {
            r: encode(self.r),
            g: encode(self.g),
            b: encode(self.b),
        }
    }
    fn decode_srgb(&self) -> ColorRgbF64 {
        fn decode(x: f64) -> f64 {
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / (1. + 0.055)).powf(2.4)
            }
        }
        ColorRgbF64 {
            r: decode(self.r),
            g: decode(self.g),
            b: decode(self.b),
        }
    }
    fn from_hue(hue: f64) -> ColorRgbF64 {
        let x = 1. - (hue % 2. - 1.).abs();
        match hue {
            h if h >= 0. && h < 1. => ColorRgbF64 { r: 1., g: x, b: 0. },
            h if h >= 1. && h < 2. => ColorRgbF64 { r: x, g: 1., b: 0. },
            h if h >= 2. && h < 3. => ColorRgbF64 { r: 0., g: 1., b: x },
            h if h >= 3. && h < 4. => ColorRgbF64 { r: 0., g: x, b: 1. },
            h if h >= 4. && h < 5. => ColorRgbF64 { r: x, g: 0., b: 1. },
            h if h >= 5. && h < 6. => ColorRgbF64 { r: 1., g: 0., b: x },
            _ => unreachable!(),
        }
    }
    fn target_luminance(&self, lum: f64, cs: &ColorSpace) -> ColorRgbF64 {
        let l = self.luminance(cs);
        if l < lum {
            let d = (lum - 1.) / (l - 1.);
            self * d + ColorRgbF64::white() * (1. - d)
        } else {
            self * (lum / l)
        }
    }
    fn white() -> ColorRgbF64 {
        ColorRgbF64 {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
}

impl Add<ColorRgbF64, ColorRgbF64> for ColorRgbF64 {
    fn add(&self, o: &ColorRgbF64) -> ColorRgbF64 {
        ColorRgbF64 {
            r: self.r.add(&o.r),
            g: self.g.add(&o.g),
            b: self.b.add(&o.b),
        }
    }
}

impl Mul<f64, ColorRgbF64> for ColorRgbF64 {
    fn mul(&self, o: &f64) -> ColorRgbF64 {
        ColorRgbF64 {
            r: self.r.mul(o),
            g: self.g.mul(o),
            b: self.b.mul(o),
        }
    }
}

impl Div<f64, ColorRgbF64> for ColorRgbF64 {
    fn div(&self, o: &f64) -> ColorRgbF64 {
        ColorRgbF64 {
            r: self.r.div(o),
            g: self.g.div(o),
            b: self.b.div(o),
        }
    }
}

fn rainbow_username() {
    let s = "ABCDEFGHI";
    let len = s.len();
    let num = len - 1;
    let lo = 400;
    let hi = 650;
    let colors = range(0, len).map(|i| {
        let xyz = ColorXyz::from_wavelength(lo + (num - i) * (hi - lo) / num);
        (xyz.to_rgb(&SRGB).constrain().normalize() + ColorRgbF64::white() * 0.1).normalize()
    }).collect::<Vec<ColorRgbF64>>();
    let minlum = colors.iter().fold(1f64, |s, c| {
        s.min(c.luminance(&SRGB))
    });
    for c in colors.iter().map(|c| {
        (c * minlum / c.luminance(&SRGB)).encode_srgb().to_int()
    }) {
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }
}
fn irc_nick_colors() {
    let num = 9u;
    for c in range(0, num).map(|i| {
        let hue = i as f64 * (6. / num as f64);
        ColorRgbF64::from_hue(hue).target_luminance(0.5, &SRGB).encode_srgb().to_int()
    }) {
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }
}
fn main() {
    irc_nick_colors()
}
