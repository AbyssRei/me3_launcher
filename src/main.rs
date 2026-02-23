use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
// use std::path::PathBuf;
use std::process::Command;

/// 配置文件结构
#[derive(Deserialize)]
struct Config {
    /// me3 程序路径
    me3_path: String,
    /// 模组文件路径 (-p 参数)
    mod_path: String,
    /// 游戏主程序路径 (--exe 参数)
    game_exe: String,
    /// 游戏类型 (--game 参数)，例如 "eldenring"
    game: String,
    /// 附加参数，例如 ["--skip-steam-init"]
    extra_args: Option<Vec<String>>,
}

fn main() -> Result<()> {
    // 1. 设置 Windows 控制台代码页为 UTF-8 (相当于 chcp 65001)
    // 使用 unsafe 块调用 Windows API
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};
        let _ = SetConsoleCP(65001);
        let _ = SetConsoleOutputCP(65001);
    }

    // 2. 确定配置文件路径
    // 获取当前 exe 所在目录，查找同目录下的 config.toml
    let exe_path = env::current_exe().context("无法获取当前程序路径")?;
    let exe_dir = exe_path.parent().context("无法获取程序所在目录")?;
    let config_path = exe_dir.join("config.toml");

    println!("正在读取配置: {:?}", config_path);

    // 3. 读取并解析 TOML
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("读取配置文件失败: {:?}", config_path))?;

    let config: Config =
        toml::from_str(&config_content).context("解析配置文件失败，请检查 TOML 格式是否正确")?;

    // 4. 构建命令参数列表
    let mut args = vec![
        "launch".to_string(),
        "-p".to_string(),
        config.mod_path,
        "--game".to_string(),
        config.game, // 从配置文件读取 game 参数
    ];

    // 添加可选的附加参数
    if let Some(extra) = &config.extra_args {
        args.extend(extra.clone());
    }

    // 最后添加 --exe 参数
    args.push("--exe".to_string());
    args.push(config.game_exe);

    println!("--------------------------------");
    println!("启动程序: {}", config.me3_path);
    println!("工作目录: {:?}", exe_dir);
    println!("参数列表: {:?}", args);
    println!("--------------------------------");

    // 5. 启动进程
    let status = Command::new(&config.me3_path)
        .args(&args)
        .current_dir(exe_dir) // 设置工作目录，确保相对路径有效
        .status()
        .with_context(|| format!("无法启动程序: {}", config.me3_path))?;

    if !status.success() {
        eprintln!("\n程序异常退出，退出码: {:?}", status.code());
    } else {
        println!("\n程序已正常退出。");
    }

    Ok(())
}
