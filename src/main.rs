use std::env;
use std::fs;
use std::path::Path;
use std::process;
use img_hash::{HasherConfig, HashAlg, ImageHash};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (dir, threshold, algo) = parse_args(&args);

    let paths = gather_images(&dir);
    if paths.is_empty() {
        eprintln!("在 {} 中没有找到 jpg/png 图片", dir);
        process::exit(1);
    }
    println!("找到 {} 张图片\n", paths.len());

    match algo {
        Algo::All => compare_all(&paths, threshold),
        single => run_single(&paths, threshold, single),
    }
}

#[derive(Clone, Copy)]
enum Algo {
    /// ahash = Mean，最朴素的均值哈希
    Ahash,
    /// dhash = Gradient，逐行梯度比较
    Dhash,
    /// phash = Mean + DCT 预处理，抗压缩/缩放最强
    Phash,
    /// 三合一对比
    All,
}

impl Algo {
    fn name(&self) -> &str {
        match self {
            Algo::Ahash => "ahash",
            Algo::Dhash => "dhash",
            Algo::Phash => "phash",
            Algo::All => "all",
        }
    }
}

/// 解析命令行参数
fn parse_args(args: &[String]) -> (String, u32, Algo) {
    let mut dir = String::new();
    let mut threshold = 15u32;
    let mut algo = Algo::Phash;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--threshold" | "-t" => {
                i += 1;
                if i < args.len() {
                    threshold = args[i].parse().unwrap_or(15);
                }
            }
            "--algo" | "-a" => {
                i += 1;
                if i < args.len() {
                    algo = match args[i].as_str() {
                        "ahash" => Algo::Ahash,
                        "dhash" => Algo::Dhash,
                        "phash" => Algo::Phash,
                        "all" => Algo::All,
                        _ => Algo::Phash,
                    };
                }
            }
            _ => {
                if !args[i].starts_with('-') && dir.is_empty() {
                    dir = args[i].clone();
                }
            }
        }
        i += 1;
    }

    if dir.is_empty() {
        eprintln!("用法: cargo run -- <图片文件夹> [--threshold N] [--algo phash|ahash|dhash|all]");
        eprintln!("");
        eprintln!("  --threshold, -t  相似度阈值，汉明距离 ≤ N 视为相似（默认 15）");
        eprintln!("  --algo, -a       哈希算法：phash(默认) / ahash / dhash / all");
        process::exit(1);
    }

    (dir, threshold, algo)
}

/// 构建 Hasher
/// - ahash: Mean 算法，无预处理
/// - dhash: Gradient 算法，无预处理
/// - phash: Mean 算法 + DCT 预处理（经典感知哈希做法）
fn build_hasher(algo: Algo) -> HasherConfig {
    match algo {
        Algo::Ahash => HasherConfig::new().hash_alg(HashAlg::Mean),
        Algo::Dhash => HasherConfig::new().hash_alg(HashAlg::Gradient),
        Algo::Phash => HasherConfig::new()
            .hash_alg(HashAlg::Mean)
            .preproc_dct(),
        Algo::All => unreachable!(),
    }
}

/// 单算法模式
fn run_single(paths: &[String], threshold: u32, algo: Algo) {
    let hasher = build_hasher(algo).to_hasher();

    let hashed: Vec<(&String, ImageHash)> = paths
        .iter()
        .filter_map(|p| match image::open(p) {
            Ok(img) => Some((p, hasher.hash_image(&img))),
            Err(e) => {
                eprintln!("跳过 {}: {}", p, e);
                None
            }
        })
        .collect();

    println!("--- {} (阈值 ≤ {}) ---", algo.name(), threshold);
    print_matches(&hashed, threshold);
}

/// 三算法对比模式
fn compare_all(paths: &[String], threshold: u32) {
    // 预载所有图片，避免每换一个算法重新解码
    let images: Vec<(&String, image::DynamicImage)> = paths
        .iter()
        .filter_map(|p| match image::open(p) {
            Ok(img) => Some((p, img)),
            Err(e) => {
                eprintln!("跳过 {}: {}", p, e);
                None
            }
        })
        .collect();

    // 三个算法：名称 → HasherConfig
    let hashers: Vec<(&str, _)> = vec![
        ("ahash", build_hasher(Algo::Ahash).to_hasher()),
        ("dhash", build_hasher(Algo::Dhash).to_hasher()),
        ("phash", build_hasher(Algo::Phash).to_hasher()),
    ];

    // 表头
    print!("{:<32}", "图片对");
    for (name, _) in &hashers {
        print!("{:>8}", name);
    }
    println!();

    // 两两比较
    for i in 0..images.len() {
        for j in i + 1..images.len() {
            let a = basename(images[i].0);
            let b = basename(images[j].0);
            let label = format!("{} <-> {}", a, b);
            print!("{:<32}", truncate(&label, 32));

            for (_, hasher) in &hashers {
                let h1 = hasher.hash_image(&images[i].1);
                let h2 = hasher.hash_image(&images[j].1);
                let dist = h1.dist(&h2);
                if dist <= threshold {
                    print!("{:>6}✓ ", dist);
                } else {
                    print!("{:>7}", "-");
                }
            }
            println!();
        }
    }

    println!("\n阈值 = {}（汉明距离 ≤ {} 视为相似）", threshold, threshold);
    println!("ahash: Mean 均值哈希，最简单，对亮度变化敏感");
    println!("dhash: Gradient 梯度哈希，逐行比较，抗亮度变化");
    println!("phash: Mean + DCT 预处理，基于频域，抗缩放/压缩最强");
}

/// 打印匹配结果
fn print_matches(hashed: &[(&String, ImageHash)], threshold: u32) {
    let mut found = 0;
    for i in 0..hashed.len() {
        for j in i + 1..hashed.len() {
            let dist = hashed[i].1.dist(&hashed[j].1);
            if dist <= threshold {
                println!(
                    "相似: {}  <->  {}  (距离: {})",
                    hashed[i].0, hashed[j].0, dist
                );
                found += 1;
            }
        }
    }
    if found == 0 {
        println!("没有找到相似图片");
    } else {
        println!("\n共找到 {} 对相似图片", found);
    }
}

/// 遍历文件夹，收集 jpg / jpeg / png
fn gather_images(dir: &str) -> Vec<String> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png") {
                    if let Some(p) = path.to_str() {
                        paths.push(p.to_string());
                    }
                }
            }
        }
    }
    paths
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max - 3).collect();
        format!("{}...", truncated)
    }
}

/// 提取文件名（不含路径）
fn basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}
