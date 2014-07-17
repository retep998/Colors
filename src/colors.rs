
mod tables;

static SRGB: ColorSystem = ColorSystem {
    r: ColxyY {
        x: 0.6400,
        y: 0.3300,
        Y: 0.2126,
    },
    g: ColxyY {
        x: 0.3000,
        y: 0.6000,
        Y: 0.7152,
    },
    b: ColxyY {
        x: 0.1500,
        y: 0.0600,
        Y: 0.0722,
    },
    w: ColxyY {
        x: 0.3127,
        y: 0.3290,
        Y: 1.0000,
    },
};

#[deriving(Show)]
struct ColorSystem {
    r: ColxyY,
    g: ColxyY,
    b: ColxyY,
    w: ColxyY,
}

#[deriving(Show)]
struct ColXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl ColXYZ {
    fn from_wavelength(wavelength: uint) -> ColXYZ {
        use self::tables::CIE_COLOR_MATCH;
        match CIE_COLOR_MATCH.get(wavelength - 390) {
            Some(c) => ColXYZ {
                x: c[0],
                y: c[1],
                z: c[2],
            },
            None => ColXYZ {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    fn to_rgb(&self, cs: &ColorSystem) -> ColRGB {
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
        ColRGB {
            r: rx * xc + ry * yc + rz * zc,
            g: gx * xc + gy * yc + gz * zc,
            b: bx * xc + by * yc + bz * zc,
        }
    }
    fn normalize(&self) -> ColXYZ {
        let m = self.x.max(self.y).max(self.z);
        ColXYZ {
            x: self.x.div(&m),
            y: self.y.div(&m),
            z: self.z.div(&m),
        }
    }
}

#[allow(uppercase_variables)]
#[deriving(Show)]
struct ColxyY {
    x: f64,
    y: f64,
    Y: f64,
}

#[deriving(Show)]
struct ColRGB24 {
    r: u8,
    g: u8,
    b: u8,
}

impl ColRGB24 {
    fn to_float(&self) -> ColRGB {
        ColRGB {
            r: self.r as f64 / 255.,
            g: self.g as f64 / 255.,
            b: self.b as f64 / 255.,
        }
    }
}

#[deriving(Show)]
struct ColRGB {
    r: f64,
    g: f64,
    b: f64,
}

impl ColRGB {
    fn to_int(&self) -> ColRGB24 {
        ColRGB24 {
            r: (self.r.min(1.).max(0.) * 255.).round() as u8,
            g: (self.g.min(1.).max(0.) * 255.).round() as u8,
            b: (self.b.min(1.).max(0.) * 255.).round() as u8,
        }
    }
    fn luminance(&self, cs: &ColorSystem) -> f64 {
        self.r * cs.r.Y + self.g * cs.g.Y + self.b * cs.b.Y
    }
    fn normalize(&self) -> ColRGB {
        let m = self.r.max(self.g).max(self.b);
        ColRGB {
            r: self.r.div(&m),
            g: self.g.div(&m),
            b: self.b.div(&m),
        }
    }
    fn constrain(&self) -> ColRGB {
        let w = 0f64.min(self.r).min(self.g).min(self.b);
        ColRGB {
            r: self.r - w,
            g: self.g - w,
            b: self.b - w,
        }
    }
    fn encode_srgb(&self) -> ColRGB {
        fn encode(x: f64) -> f64 {
            if x <= 0.0031308 {
                x * 12.92
            } else {
                x.powf(2.4f64.recip()) * (1. + 0.055) - 0.055
            }
        }
        ColRGB {
            r: encode(self.r),
            g: encode(self.g),
            b: encode(self.b),
        }
    }
    fn decode_srgb(&self) -> ColRGB {
        fn decode(x: f64) -> f64 {
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / (1. + 0.055)).powf(2.4)
            }
        }
        ColRGB {
            r: decode(self.r),
            g: decode(self.g),
            b: decode(self.b),
        }
    }
}

impl Mul<f64, ColRGB> for ColRGB {
    fn mul(&self, o: &f64) -> ColRGB {
        ColRGB {
            r: self.r.mul(o),
            g: self.g.mul(o),
            b: self.b.mul(o),
        }
    }
}

impl Div<f64, ColRGB> for ColRGB {
    fn div(&self, o: &f64) -> ColRGB {
        ColRGB {
            r: self.r.div(o),
            g: self.g.div(o),
            b: self.b.div(o),
        }
    }
}

fn main() {
    let s = "Retep998";
    let len = s.len();
    let num = len - 1;
    let lo = 400;
    let hi = 650;
    let colors = range(0, len).map(|i| {
        let xyz = ColXYZ::from_wavelength(lo + (num - i) * (hi - lo) / num);
        xyz.to_rgb(&SRGB).constrain().normalize()
    }).collect::<Vec<ColRGB>>();
    let minlum = colors.iter().fold(1f64, |s, c| {
        s.min(c.luminance(&SRGB))
    });
    for c in colors.iter().map(|c| {
        (c * minlum / c.luminance(&SRGB)).encode_srgb().to_int()
    }) {
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }
}
