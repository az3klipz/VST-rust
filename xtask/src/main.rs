use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "installer" => build_installer(),
        "bundle" => run_bundle(),
        _ => println!("Usage: cargo xtask [bundle|installer]"),
    }
}

fn run_bundle() {
    println!("Step 1.1: Bundling Plugin...");
    let status = Command::new("cargo")
        .args(&["xtask", "bundle", "antigravity_designer", "--release"])
        .status()
        .expect("Failed to run bundle");
    assert!(status.success());

    println!("Step 1.2: Building Standalone Executable...");
    let status_bin = Command::new("cargo")
        .args(&["build", "--release", "--bin", "antigravity_standalone"])
        .status()
        .expect("Failed to build standalone");
    assert!(status_bin.success());
}

fn build_installer() {
    // 1. Ensure the plugin is bundled first
    run_bundle();

    println!("Step 2: Compiling .exe Installer via Inno Setup...");
    
    // Path to Inno Setup Compiler (Standard install path)
    let iscc_path = "C:\\Program Files (x86)\\Inno Setup 6\\ISCC.exe";
    
    if !Path::new(iscc_path).exists() {
        panic!("ISCC.exe not found. Please install Inno Setup 6.");
    }

    let status = Command::new(iscc_path)
        .arg("installer.iss")
        .status()
        .expect("Failed to execute Inno Setup");

    if status.success() {
        println!("Success! Installer created in ./target/installer/");
    } else {
        eprintln!("Error: Installer compilation failed.");
    }
}
