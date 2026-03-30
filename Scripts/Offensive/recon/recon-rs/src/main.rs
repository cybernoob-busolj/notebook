use clap::Parser;
use regex::Regex;
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Target (IP or domain)
    target: String,
}

fn run_command(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("failed to execute command");

    if !status.success() {
        eprintln!("[-] Command failed: {}", cmd);
    }
}

fn run_nmap_full(target: &str) {
    println!("[+] Running full port scan...");

    run_command(
        "nmap",
        &[
            "-p-",
            "--min-rate=1000",
            "-oN",
            "scans/tcp_all_ports.nmap",
            target,
        ],
    );
}

fn extract_ports() -> String {
    println!("[+] Extracting open ports...");

    let content =
        fs::read_to_string("scans/tcp_all_ports.nmap").expect("failed to read nmap output");

    let re = Regex::new(r"(\d+)/tcp\s+open").unwrap();

    let mut ports = Vec::new();

    for cap in re.captures_iter(&content) {
        ports.push(cap[1].to_string());
    }

    let ports_str = ports.join(",");

    println!("[+] Open ports: {}", ports_str);

    ports_str
}

fn run_nmap_services(target: &str, ports: &str) {
    println!("[+] Running service scan...");

    run_command(
        "nmap",
        &[
            "-sV",
            "-sC",
            "-p",
            ports,
            "-oN",
            "scans/tcp_services.nmap",
            target,
        ],
    );
}

fn http_enum() {
    println!("[+] Running httpx...");

    let cmd = r#"cat scans/tcp_services.nmap \
        | grep -Eo '([0-9]{1,5})/tcp.*open' \
        | cut -d'/' -f1 \
        | while read port; do echo "TARGET:$port"; done \
        | httpx -tech-detect -status-code -title -json \
        > scans/httpx.json"#;

    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .status()
        .expect("failed httpx");
}

fn main() {
    let args = Args::parse();

    // setup
    fs::create_dir_all("scans").unwrap();

    // pipeline
    run_nmap_full(&args.target);
    let ports = extract_ports();

    if ports.is_empty() {
        println!("[-] No open ports found");
        return;
    }

    run_nmap_services(&args.target, &ports);

    println!("[+] Recon completed.");
}
