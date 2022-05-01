use std::env;
use image::RgbImage;

const RR: f32 = 0.30;
const GR: f32 = 0.59;
const BR: f32 = 0.11;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("image path is none");
    let mut img = image::open(path).expect("file not found").to_rgb8();
    convert(&mut img);
    img.save(path.to_string() + ".out.jpg").unwrap();
}

// 丸める
fn clamp(f: f32) -> f32 {
    if f < 0.0 { return 0.0; }
    else if f > 255.0 { return 255.0; }
    else { return f; }
}

// グレイスケール変換
fn to_gray(r: f32, g: f32, b: f32, wr: f32, wg: f32, wb: f32) -> f32 {
    let sum = wr + wg + wb;
    return r * (wr / sum) + g * (wg / sum) + b * (wb / sum);
}

// 2ストリップ変換
fn convert(img: &mut RgbImage) {
    let (width, height) = img.dimensions();
    for py in 0..height {
        for px in 0..width {
            let pixel = img.get_pixel(px, py);
            let r = f32::from(pixel[0]);
            let g = f32::from(pixel[1]);
            let b = f32::from(pixel[2]);
// println!("r: {}, g: {}, b: {}", r, g, b);

            let c = to_gray(r, g, b, 0.0,  GR,  BR);
            let m = to_gray(r, g, b,  RR, 0.0,  BR);
            let y = to_gray(r, g, b,  RR,  GR, 0.0);
// println!("c: {}, m: {}, ye: {}", c, m, ye);

            let r2 = 255.0 - clamp(r - c);
            let g2 = 255.0 - clamp(g - m);
            let b2 = 255.0 - clamp(b - y);
// println!("r2: {}, g2: {}, b2: {}", r2, g2, b2);

            let r = clamp(r - (255.0 - (g2 * b2 / 256.0)));
            let g = clamp(g - (255.0 - (r2 * b2 / 256.0)));
            let b = clamp(b - (255.0 - (r2 * g2 / 256.0)));
// println!("r: {}, g: {}, b: {}", r, g, b);
// println!("");

            let new_color = [r as u8, g as u8, b as u8];
            img.put_pixel(px, py, image::Rgb(new_color));
        }
     }
}
