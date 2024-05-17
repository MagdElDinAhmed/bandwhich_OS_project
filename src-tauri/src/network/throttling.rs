use std::process::Command;
use std::io::Error;

pub fn set_egress_bandwidth_limit(interface: &str, limit_mbps: usize) -> Result<(), Error> {
    let command = format!(
        "tc qdisc replace dev {} root handle 1: tbf rate {}mbit burst 32kbit latency 400ms",
        interface, limit_mbps
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to set egress bandwidth limit"
        ))
    }
}

pub fn set_ingress_bandwidth_limit(interface: &str, limit_mbps: usize) -> Result<(), Error> {
    let setup_ifb = Command::new("sh")
        .arg("-c")
        .arg("modprobe ifb; ip link add ifb0 type ifb; ip link set dev ifb0 up")
        .output()?;
    
    if !setup_ifb.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to setup ifb device"
        ));
    }

    let redirect_ingress = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "tc qdisc replace dev {} ingress; tc filter replace dev {} parent ffff: protocol ip u32 match u32 0 0 action mirred egress redirect dev ifb0",
            interface, interface
        ))
        .output()?;
    
    if !redirect_ingress.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to redirect ingress traffic"
        ));
    }

    let limit_ifb = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc replace dev ifb0 root tbf rate {}mbit burst 30kbit latency 400ms",
            limit_mbps
        ))
        .output()?;
    
    if limit_ifb.status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to set ingress bandwidth limit"
        ))
    }
}