#![allow(clippy::unnecessary_map_or)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let icons_dir = Path::new("assets/icons");
    let _ = fs::create_dir_all(icons_dir);
    let ico_src = Path::new("assets");

    embed_resource::compile("resources.rc", embed_resource::NONE);
    let mappings = [
        ("cpu.ico", "cpu"),
        ("default.ico", "default"),
        ("disk.ico", "disk"),
        ("input.ico", "input"),
        ("network.ico", "network"),
        ("pause.ico", "paused"),
        ("process.ico", "process"),
        ("sound.ico", "sound"),
    ];

    const PREFERRED_SIZES: [u32; 3] = [48, 32, 16];

    for (ico_name, out_name) in &mappings {
        let ico_path = ico_src.join(ico_name);
        if !ico_path.exists() { continue; }

        let all_sizes = extract_all_rgba_from_ico(&ico_path, &PREFERRED_SIZES);
        if all_sizes.is_empty() { continue; }

        let (main_w, _, main_rgba) = &all_sizes[0];
        generate_icon(icons_dir, out_name, main_rgba);
        generate_icon(icons_dir, &format!("{}_dark", out_name), &invert_rgba(main_rgba));

        for (w, _, rgba) in all_sizes.iter().skip(1) {
            if *w != *main_w {
                let suffix = format!("{}_{}", out_name, w);
                generate_icon(icons_dir, &suffix, rgba);
                generate_icon(icons_dir, &format!("{}_dark", &suffix), &invert_rgba(rgba));
            }
        }
    }

    // .rgba → コンパクトバイナリに変換して OUT_DIR に出力
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut entries = fs::read_dir(icons_dir)
        .expect("Failed to read icons dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rgba"))
        .collect::<Vec<_>>();
    entries.sort_by_key(|e| e.file_name());

    for entry in &entries {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let data = fs::read(&path).expect("Failed to read icon file");
        let compact = quantize_rgba(&data);
        let out_file = out_dir.join(format!("{}.bin", name));
        fs::write(&out_file, &compact).expect("Failed to write icon binary");
    }
}

fn extract_all_rgba_from_ico(path: &Path, preferred_sizes: &[u32]) -> Vec<(u32, u32, Vec<u8>)> {
    let data = match fs::read(path) { Ok(d) => d, Err(_) => return vec![] };
    if data.len() < 6 { return vec![]; }
    let count = u16::from_le_bytes([data[4], data[5]]) as usize;
    if count == 0 { return vec![]; }
    let mut results: Vec<(u32, u32, Vec<u8>)> = vec![];

    for i in 0..count {
        let offset = 6 + i * 16;
        if offset + 16 > data.len() { break; }
        let w = if data[offset] == 0 { 256 } else { data[offset] as u32 };
        let h = if data[offset + 1] == 0 { 256 } else { data[offset + 1] as u32 };
        if !preferred_sizes.contains(&w) || w != h { continue; }
        let img_size = u32::from_le_bytes([data[offset+8],data[offset+9],data[offset+10],data[offset+11]]) as usize;
        let img_offset = u32::from_le_bytes([data[offset+12],data[offset+13],data[offset+14],data[offset+15]]) as usize;
        if img_offset + img_size > data.len() { continue; }
        let img_data = &data[img_offset..img_offset + img_size];
        let rgba = if img_data.len() >= 4 && &img_data[0..4] == b"\x89PNG" {
            decode_png_rgba_any(img_data)
        } else {
            decode_bmp_rgba(img_data, w, h)
        };
        if let Some(rgba) = rgba { results.push((w, h, rgba)); }
    }
    results.sort_by(|a, b| b.0.cmp(&a.0));
    results
}

fn decode_png_rgba_any(data: &[u8]) -> Option<Vec<u8>> {
    let decoder = png::Decoder::new(std::io::Cursor::new(data));
    let mut reader = decoder.read_info().ok()?;
    let color_type = reader.info().color_type;
    let w = reader.info().width as usize;
    let h = reader.info().height as usize;
    let mut rgba = vec![0u8; w * h * 4];
    reader.next_frame(&mut rgba).ok()?;
    if matches!(color_type, png::ColorType::Rgba) {
        Some(rgba)
    } else if color_type == png::ColorType::Rgb {
        let mut rgba_out = vec![0u8; w * h * 4];
        for i in 0..(w * h) {
            rgba_out[i * 4] = rgba[i * 3];
            rgba_out[i * 4 + 1] = rgba[i * 3 + 1];
            rgba_out[i * 4 + 2] = rgba[i * 3 + 2];
            rgba_out[i * 4 + 3] = 255;
        }
        Some(rgba_out)
    } else { None }
}

fn decode_bmp_rgba(data: &[u8], w: u32, h: u32) -> Option<Vec<u8>> {
    if data.len() < 40 { return None; }
    let bi_height = i32::from_le_bytes([data[8],data[9],data[10],data[11]]);
    let top_down = bi_height < 0;
    let xor_size = (w as usize) * (h as usize) * 4;
    if 40 + xor_size > data.len() { return None; }
    let mut rgba = vec![0u8; xor_size];
    let src = &data[40..40 + xor_size];
    for y in 0..h {
        let src_row = if top_down { (y * w * 4) as usize } else { ((h - 1 - y) * w * 4) as usize };
        let dst_row = (y * w * 4) as usize;
        for x in 0..w as usize {
            let s = src_row + x * 4;
            let d = dst_row + x * 4;
            rgba[d] = src[s + 2]; rgba[d + 1] = src[s + 1]; rgba[d + 2] = src[s]; rgba[d + 3] = src[s + 3];
        }
    }
    Some(rgba)
}

fn invert_rgba(rgba: &[u8]) -> Vec<u8> {
    let mut out = rgba.to_vec();
    for i in (0..out.len()).step_by(4) {
        out[i] = 255 - out[i]; out[i + 1] = 255 - out[i + 1]; out[i + 2] = 255 - out[i + 2];
    }
    out
}

fn generate_icon(dir: &Path, name: &str, rgba: &[u8]) {
    fs::write(dir.join(format!("{}.rgba", name)), rgba).expect("Failed to write icon file");
}

fn quantize_rgba(rgba: &[u8]) -> Vec<u8> {
    let pixel_count = rgba.len() / 4;
    let mut color_map: HashMap<[u8; 4], u8> = HashMap::new();
    let mut palette: Vec<[u8; 4]> = Vec::new();
    let mut indices: Vec<u8> = Vec::with_capacity(pixel_count);
    for i in 0..pixel_count {
        let color = [rgba[i * 4], rgba[i * 4 + 1], rgba[i * 4 + 2], rgba[i * 4 + 3]];
        let idx = *color_map.entry(color).or_insert_with(|| { let idx = palette.len() as u8; palette.push(color); idx });
        indices.push(idx);
    }
    if palette.len() <= 256 {
        let mut compact = Vec::with_capacity(1 + 1 + palette.len() * 4 + pixel_count);
        compact.push(1u8); compact.push((palette.len() - 1) as u8);
        for color in &palette { compact.extend_from_slice(color); }
        compact.extend_from_slice(&indices);
        compact
    } else {
        let mut compact = Vec::with_capacity(1 + rgba.len());
        compact.push(0u8); compact.extend_from_slice(rgba);
        compact
    }
}
