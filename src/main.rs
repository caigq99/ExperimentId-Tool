use std::process::Command;
use std::str;
use uuid::Uuid;

const DEFAULT_DOMAIN: &str = "dev.warp.Warp-Stable";

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    // 解析 --domain
    let mut domain = DEFAULT_DOMAIN.to_string();
    if let Some(i) = args.iter().position(|a| a == "--domain") {
        if i + 1 >= args.len() {
            eprintln!("❌ --domain 需要一个值");
            std::process::exit(2);
        }
        domain = args[i + 1].clone();
        args.drain(i..=i + 1);
    }

    // 读取旧值
    let old_val = Command::new("defaults")
        .args(["read", &domain, "ExperimentId"])
        .output()
        .ok()
        .and_then(|out| {
            if out.status.success() {
                Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
            } else {
                None
            }
        });

    if let Some(v) = &old_val {
        println!("🔎 旧的 ExperimentId: {}", v);
    } else {
        println!("⚠️ 未找到旧的 ExperimentId，可能是首次写入。");
    }

    // 解析 --id
    let mut chosen: Option<Uuid> = None;
    if let Some(i) = args.iter().position(|a| a == "--id") {
        if i + 1 >= args.len() {
            eprintln!("❌ --id 需要一个 UUID 值");
            std::process::exit(2);
        }
        let raw = args[i + 1].trim();
        match Uuid::parse_str(raw) {
            Ok(u) => chosen = Some(u),
            Err(_) => {
                eprintln!("❌ 提供的 --id 不是合法的 UUID：{raw}");
                std::process::exit(2);
            }
        }
        args.drain(i..=i + 1);
    }

    // 若未提供，则生成一个新的 v4
    let new_id = chosen.unwrap_or_else(Uuid::new_v4);

    // 写入新的 ExperimentId
    let write_status = Command::new("defaults")
        .args([
            "write",
            &domain,
            "ExperimentId",
            "-string",
            &new_id.to_string(),
        ])
        .status()
        .expect("无法启动 defaults 命令，请确认在 macOS 上运行");

    if !write_status.success() {
        eprintln!("❌ ExperimentId 写入失败。");
        std::process::exit(write_status.code().unwrap_or(1));
    }

    println!("✅ 新的 ExperimentId 已写入: {}", new_id);

    // 同时重置 DidNonAnonymousUserLogIn 为 false
    let reset_status = Command::new("defaults")
        .args([
            "write",
            &domain,
            "DidNonAnonymousUserLogIn",
            "-bool",
            "false",
        ])
        .status()
        .expect("无法启动 defaults 命令，请确认在 macOS 上运行");

    if !reset_status.success() {
        eprintln!("❌ DidNonAnonymousUserLogIn 重置失败。");
        std::process::exit(reset_status.code().unwrap_or(1));
    }

    println!("✅ DidNonAnonymousUserLogIn 已重置为 false");

    // 再次回读确认 ExperimentId
    let output = Command::new("defaults")
        .args(["read", &domain, "ExperimentId"])
        .output()
        .expect("读取失败：无法启动 defaults");

    if output.status.success() {
        let val = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("📌 回读确认 ExperimentId: {}", val);
    }

    // 回读确认 DidNonAnonymousUserLogIn
    let login_output = Command::new("defaults")
        .args(["read", &domain, "DidNonAnonymousUserLogIn"])
        .output()
        .expect("读取失败：无法启动 defaults");

    if login_output.status.success() {
        let login_val = String::from_utf8_lossy(&login_output.stdout).trim().to_string();
        println!("📌 回读确认 DidNonAnonymousUserLogIn: {}", login_val);
    }
}
