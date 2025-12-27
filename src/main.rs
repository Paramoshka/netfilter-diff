use std::process::Command;

fn detect_netfilter_frontend() {
    let status = Command::new("iptables")
        .arg("-V")
        .status()
        .expect("Failed get infomation from iptables");

    println!("{:?}", status)
}

fn main() {
    detect_netfilter_frontend();
}
