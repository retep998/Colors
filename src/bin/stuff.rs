// Copyright © 2014, Peter Atashian

#![allow(dead_code)]

extern crate colors;

use std::iter::{AdditiveIterator};
use colors::{ColorXyz, SRGB, ColorRgbF64, ColorRgbU8};
use colors::tables::CIE_COLOR_MATCH;

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

fn black_body(temp: f64) -> ColorXyz {
    fn sample(w: f64, t: f64) -> f64 {
        let h = 6.62606957E-34;
        let c = 299792458.;
        let k = 1.3806488E-23;
        let v = c / w;
        (2. * h * v.powi(3)) / (c.powi(2) * (((h * v) / (k * t)).exp() - 1.))
    }
    CIE_COLOR_MATCH.iter().enumerate().map(|(wave, col)| {
        let energy = sample((wave as f64 + 390.) * 1E-9, temp);
        ColorXyz::from_array(col) * energy
    }).sum()
}

fn stuff() {
    for i in range(0i, 14) {
        let i = i as f64 * 100. + 1000.;
        let c = black_body(i).to_rgb(&SRGB).constrain().normalize().encode_srgb().to_int();
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }
}

fn wikipedia() {
    let num = 5u;
    let target = 0.2;
    for i in range(0, num) {
        let hue = i as f64 * (6. / num as f64);
        let c = ColorRgbF64::from_hue(hue).target_luminance(target, &SRGB).encode_srgb().to_int();
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }
    let c = ColorRgbF64::white().target_luminance(target, &SRGB).encode_srgb().to_int();
    println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
}

fn grayscale(c: u32) -> f64 {
    let r = (c >> 16) as u8;
    let g = (c >> 8) as u8;
    let b = (c >> 0) as u8;
    ColorRgbU8 { r: r, g: g, b: b }.to_float().decode_srgb().luminance(&SRGB)
}

fn print_colors() {
    for i in range(0u16, 256) {
        print!("\x1b[48;5;{}m ", i);
        if i % 64 == 63 { println!("\x1b[0m") }
    }
    for i in range(0u16, 256) {
        print!("\x1b[38;5;{}mX", i);
        if i % 64 == 63 { println!("\x1b[0m") }
    }
}

fn main() {
    println!("beep beep");
}
