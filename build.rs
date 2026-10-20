#![allow(clippy::unnecessary_map_or)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let icons_dir = Path::new("assets/icons");
    let _ = fs::create_dir_all(icons_dir);
    let ico_src = Path::new("assets");

    // アプリケーションアイコン用に Default.png から default.ico を自動生成
    let default_png = ico_src.join("Default.png");
    let default_ico = ico_src.join("default.ico");
    let default_res = ico_src.join("default.res");
    if default_png.exists() {
        let _ = write_ico_from_png(&default_png, &default_ico);
    }
    if default_ico.exists() {
        let _ = generate_res_from_ico(&default_ico, &default_res);
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let res_abs = Path::new(&manifest_dir).join("assets/default.res");
    if res_abs.exists() {
        println!("cargo:rustc-link-arg={}", res_abs.display());
    } else {
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
    let mappings = [
        ("cpu", "cpu", "CPU.png"),
        ("default", "default", "Default.png"),
        ("disk", "disk", "Disk.png"),
        ("input", "input", "Input.png"),
        ("network", "network", "Network.png"),
        ("pause", "paused", "Pause.png"),
        ("process", "process", "Process.png"),
        ("sound", "sound", "Sound.png"),
    ];

    const PREFERRED_SIZES: [u32; 3] = [48, 32, 16];

    for (ico_base, out_name, png_name) in &mappings {
        let png_path = ico_src.join(png_name);
        let ico_path = ico_src.join(format!("{}.ico", ico_base));

        if png_path.exists() {
            if let Some((w, h, rgba)) = decode_png_file(&png_path) {
                let main_w = PREFERRED_SIZES[0];
                let main_rgba = resize_rgba(&rgba, w, h, main_w, main_w);
                generate_icon(icons_dir, out_name, &main_rgba);

                for &size in PREFERRED_SIZES.iter().skip(1) {
                    let resized = resize_rgba(&rgba, w, h, size, size);
                    let suffix = format!("{}_{}", out_name, size);
                    generate_icon(icons_dir, &suffix, &resized);
                }
                continue;
            }
        }

        if !ico_path.exists() { continue; }

        let all_sizes = extract_all_rgba_from_ico(&ico_path, &PREFERRED_SIZES);
        if all_sizes.is_empty() { continue; }

        let (main_w, _, main_rgba) = &all_sizes[0];
        generate_icon(icons_dir, out_name, main_rgba);

        for (w, _, rgba) in all_sizes.iter().skip(1) {
            if *w != *main_w {
                let suffix = format!("{}_{}", out_name, w);
                generate_icon(icons_dir, &suffix, rgba);
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

fn decode_png_file(path: &Path) -> Option<(u32, u32, Vec<u8>)> {
    let data = fs::read(path).ok()?;
    let decoder = png::Decoder::new(std::io::Cursor::new(data));
    let mut reader = decoder.read_info().ok()?;
    let w = reader.info().width;
    let h = reader.info().height;
    let color_type = reader.info().color_type;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    reader.next_frame(&mut buf).ok()?;

    let mut rgba = vec![0u8; (w * h * 4) as usize];
    match color_type {
        png::ColorType::Rgba => {
            rgba.copy_from_slice(&buf[0..(w * h * 4) as usize]);
        }
        png::ColorType::Rgb => {
            for i in 0..(w * h) as usize {
                rgba[i * 4] = buf[i * 3];
                rgba[i * 4 + 1] = buf[i * 3 + 1];
                rgba[i * 4 + 2] = buf[i * 3 + 2];
                rgba[i * 4 + 3] = 255;
            }
        }
        _ => return None,
    }
    Some((w, h, rgba))
}

fn resize_rgba(rgba: &[u8], src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) -> Vec<u8> {
    if src_w == dst_w && src_h == dst_h {
        return rgba.to_vec();
    }
    let mut dst = vec![0u8; (dst_w * dst_h * 4) as usize];
    for y in 0..dst_h {
        let src_y = (y * src_h / dst_h) as usize;
        for x in 0..dst_w {
            let src_x = (x * src_w / dst_w) as usize;
            let src_idx = (src_y * src_w as usize + src_x) * 4;
            let dst_idx = (y as usize * dst_w as usize + x as usize) * 4;
            dst[dst_idx..dst_idx+4].copy_from_slice(&rgba[src_idx..src_idx+4]);
        }
    }
    dst
}

fn write_ico_from_png(png_path: &Path, ico_path: &Path) -> Option<()> {
    let png_data = fs::read(png_path).ok()?;
    let decoder = png::Decoder::new(std::io::Cursor::new(&png_data));
    let reader = decoder.read_info().ok()?;
    let w = reader.info().width;
    let h = reader.info().height;

    let mut ico = Vec::new();
    // ヘッダー: Reserved(0) u16, Type(1) u16, Count(1) u16
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());

    // ディレクトリエントリ
    let w_byte = if w >= 256 { 0u8 } else { w as u8 };
    let h_byte = if h >= 256 { 0u8 } else { h as u8 };
    ico.push(w_byte);
    ico.push(h_byte);
    ico.push(0); // ColorCount
    ico.push(0); // Reserved

    // Planes(1) u16, BitCount(32) u16
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());

    // BytesInRes (PNGデータサイズ) u32
    ico.extend_from_slice(&(png_data.len() as u32).to_le_bytes());

    // ImageOffset (ヘッダー6バイト + エントリ16バイト = 22) u32
    ico.extend_from_slice(&22u32.to_le_bytes());

    // PNGデータ本体
    ico.extend_from_slice(&png_data);

    fs::write(ico_path, ico).ok()?;
    Some(())
}

fn generate_res_from_ico(ico_path: &Path, res_path: &Path) -> Option<()> {
    let ico_bytes = fs::read(ico_path).ok()?;
    if ico_bytes.len() < 6 { return None; }
    let count = u16::from_le_bytes([ico_bytes[4], ico_bytes[5]]) as usize;
    if count == 0 { return None; }

    struct Entry {
        w: u8, h: u8, color_count: u8, res_val: u8,
        planes: u16, bit_count: u16, bytes_in_res: u32, img_offset: u32, id: u16,
    }
    let mut entries = Vec::new();
    for i in 0..count {
        let offset = 6 + i * 16;
        if offset + 16 > ico_bytes.len() { break; }
        entries.push(Entry {
            w: ico_bytes[offset],
            h: ico_bytes[offset + 1],
            color_count: ico_bytes[offset + 2],
            res_val: ico_bytes[offset + 3],
            planes: u16::from_le_bytes([ico_bytes[offset + 4], ico_bytes[offset + 5]]),
            bit_count: u16::from_le_bytes([ico_bytes[offset + 6], ico_bytes[offset + 7]]),
            bytes_in_res: u32::from_le_bytes([ico_bytes[offset + 8], ico_bytes[offset + 9], ico_bytes[offset + 10], ico_bytes[offset + 11]]),
            img_offset: u32::from_le_bytes([ico_bytes[offset + 12], ico_bytes[offset + 13], ico_bytes[offset + 14], ico_bytes[offset + 15]]),
            id: (i + 1) as u16,
        });
    }

    let mut res = Vec::new();
    // Null Resource Header (32 bytes)
    res.extend_from_slice(&0u32.to_le_bytes());
    res.extend_from_slice(&32u32.to_le_bytes());
    res.extend_from_slice(&0x0000FFFFu32.to_le_bytes());
    res.extend_from_slice(&0x0000FFFFu32.to_le_bytes());
    res.extend_from_slice(&[0u8; 16]);

    fn pad(b: &mut Vec<u8>) {
        let p = (4 - (b.len() % 4)) % 4;
        b.extend(std::iter::repeat(0).take(p));
    }

    // RT_ICON (type 3)
    for entry in &entries {
        let start = entry.img_offset as usize;
        let end = start + entry.bytes_in_res as usize;
        if end > ico_bytes.len() { continue; }
        let img_data = &ico_bytes[start..end];
        let data_size = img_data.len() as u32;

        res.extend_from_slice(&data_size.to_le_bytes());
        res.extend_from_slice(&32u32.to_le_bytes());
        res.extend_from_slice(&0xFFFFu16.to_le_bytes());
        res.extend_from_slice(&3u16.to_le_bytes()); // RT_ICON
        res.extend_from_slice(&0xFFFFu16.to_le_bytes());
        res.extend_from_slice(&entry.id.to_le_bytes());
        res.extend_from_slice(&0u32.to_le_bytes());
        res.extend_from_slice(&0x1030u16.to_le_bytes());
        res.extend_from_slice(&0u16.to_le_bytes());
        res.extend_from_slice(&0u32.to_le_bytes());
        res.extend_from_slice(&0u32.to_le_bytes());

        res.extend_from_slice(img_data);
        pad(&mut res);
    }

    // RT_GROUP_ICON (type 14)
    let mut grp_data = Vec::new();
    grp_data.extend_from_slice(&0u16.to_le_bytes());
    grp_data.extend_from_slice(&1u16.to_le_bytes());
    grp_data.extend_from_slice(&(count as u16).to_le_bytes());
    for entry in &entries {
        grp_data.push(entry.w);
        grp_data.push(entry.h);
        grp_data.push(entry.color_count);
        grp_data.push(entry.res_val);
        grp_data.extend_from_slice(&entry.planes.to_le_bytes());
        grp_data.extend_from_slice(&entry.bit_count.to_le_bytes());
        grp_data.extend_from_slice(&entry.bytes_in_res.to_le_bytes());
        grp_data.extend_from_slice(&entry.id.to_le_bytes());
    }

    let data_size = grp_data.len() as u32;
    res.extend_from_slice(&data_size.to_le_bytes());
    res.extend_from_slice(&32u32.to_le_bytes());
    res.extend_from_slice(&0xFFFFu16.to_le_bytes());
    res.extend_from_slice(&14u16.to_le_bytes()); // RT_GROUP_ICON
    res.extend_from_slice(&0xFFFFu16.to_le_bytes());
    res.extend_from_slice(&1u16.to_le_bytes()); // icon_id = 1
    res.extend_from_slice(&0u32.to_le_bytes());
    res.extend_from_slice(&0x1030u16.to_le_bytes());
    res.extend_from_slice(&0u16.to_le_bytes());
    res.extend_from_slice(&0u32.to_le_bytes());
    res.extend_from_slice(&0u32.to_le_bytes());

    res.extend_from_slice(&grp_data);
    pad(&mut res);

    fs::write(res_path, res).ok()?;
    Some(())
}
