pub fn display_header() {
    println!("=========================================");
    println!("        CYBERHEIST: TERMINAL INFILTRATION");
    println!("=========================================");
}

pub fn show_help() {
    println!();
    println!("Available actions:");
    println!("  scan (s)      - System scan for hints");
    println!("  hack (h)      - Hack firewall node / guess code");
    println!("  brute (b)     - Brute force attempt");
    println!("  status        - Show mission status");
    println!("  retreat (r)   - Abort mission");
    println!("  help          - This help text");
    println!();
}
