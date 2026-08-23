pub struct TerminalUi;

impl TerminalUi {
    pub fn print_banner() {
        println!(
            r"
  ███████╗███████╗███╗   ███╗  v1.0.0
  ██╔════╝██╔════╝████╗ ████║  High-Performance Windows System Optimizer
  ███████╗███████╗██╔████╔██║  Open-Source Native Rust CLI
  ╚════██║╚════██║██║╚██╔╝██║  https://github.com/thisath111/ssm
  ███████║███████║██║ ╚═╝ ██║
  ╚══════╝╚══════╝╚═╝     ╚═╝
"
        );
    }

    pub fn print_header(title: &str) {
        println!("\n=== {title} ===");
    }

    pub fn print_success(msg: &str) {
        println!("[+] SUCCESS: {msg}");
    }

    pub fn print_info(msg: &str) {
        println!("[*] INFO: {msg}");
    }

    pub fn print_warning(msg: &str) {
        println!("[!] WARNING: {msg}");
    }

    pub fn print_error(msg: &str) {
        println!("[x] ERROR: {msg}");
    }

    pub fn print_key_value(key: &str, val: &str) {
        println!("  {key:<28} : {val}");
    }
}
