use std::env;
use std::fs;
use img_hash::{HasherConfig, ImageHash};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("用法: cargo run -- <图片文件夹路径>");
        return;
    }

    let dir = &args[1];
    // 阶段 2+3+4：收集图片 → 算哈希 → 两两比较
    match run(dir) {
        Ok(_) => {}
        Err(e) => eprintln!("出错: {}", e),
    }
}

fn run(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = gather_images(dir);
    if paths.is_empty() {
        eprintln!("在 {} 中没有找到 jpg/png 图片", dir);
        return Ok(());
    }

    println!("找到 {} 张图片\n", paths.len());

    // 阶段 2：计算每张图片的感知哈希
    let mut hashed: Vec<(String, ImageHash)> = Vec::new();
    for path in &paths {
        match hash_image(path) {
            Ok(h) => {
                println!("[哈希] {} → {}", path, hash_to_hex(&h));
                hashed.push((path.clone(), h));
            }
            Err(e) => eprintln!("跳过 {}: {}", path, e),
        }
    }

    // 阶段 3+4：两两比较汉明距离
    println!("\n--- 相似结果 ---");
    let threshold = 15u32; // 汉明距离 ≤ 15 视为相似
    let mut found = 0;
    for i in 0..hashed.len() {
        for j in i + 1..hashed.len() {
            let dist = hashed[i].1.dist(&hashed[j].1);
            if dist <= threshold {
                println!("相似: {}  <->  {}  (距离: {})", hashed[i].0, hashed[j].0, dist);
                found += 1;
            }
        }
    }

    if found == 0 {
        println!("没有找到相似图片");
    } else {
        println!("\n共找到 {} 对相似图片", found);
    }

    Ok(())
}

/// 遍历文件夹，收集 jpg/jpeg/png 文件
fn gather_images(dir: &str) -> Vec<String> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();
                if ext == "jpg" || ext == "jpeg" || ext == "png" {
                    if let Some(p) = path.to_str() {
                        paths.push(p.to_string());
                    }
                }
            }
        }
    }
    paths
}

/// 把 ImageHash 转成十六进制字符串显示
fn hash_to_hex(hash: &ImageHash) -> String {
    hash.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
}

/// 阶段 2 核心：打开图片 → 计算感知哈希 (phash)
fn hash_image(path: &str) -> Result<ImageHash, Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let hasher = HasherConfig::new().to_hasher();
    Ok(hasher.hash_image(&img))
}
