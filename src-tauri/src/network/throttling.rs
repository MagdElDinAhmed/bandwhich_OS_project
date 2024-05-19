use std::process::Command;
use std::io::Error;
use std::str;


pub fn set_egress_bandwidth_limit(interface: &str, limit_mbps: usize) -> Result<(), Error> {
    let command = format!(
        "sudo tc qdisc replace dev {} root handle 1: tbf rate {}mbit burst 32kbit latency 400ms",
        interface, limit_mbps
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to parse error message");
        Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to set egress bandwidth limit: {}", stderr),
        ))
    }
}

pub fn reset_egress_bandwidth_limit(interface: &str) -> Result<(), Error> {
    let command = format!("tc qdisc del dev {} root", interface);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to reset egress bandwidth limit"
        ))
    }
}
pub fn set_ingress_bandwidth_limit(interface: &str, limit_mbps: usize) -> Result<(), Error> {
    // Load ifb module
    let modprobe_ifb = Command::new("sudo")
        .arg("modprobe")
        .arg("ifb")
        .output()?;

    if !modprobe_ifb.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to load ifb module",
        ));
    }
    let interfaces = pnet::datalink::interfaces();
    for int_face in interfaces {
        // Remove any existing qdisc on the specified interface
        let remove_existing_qdisc = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc del dev {} root",
            int_face.name
        ))
        .output()?;

        let remove_existing_qdisc2 = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc del dev {} ingress",
            int_face.name
        ))
        .output()?;   
    }
    // Ignore errors from removing non-existing qdiscs

    // Add ifb0 interface and set it up
    let setup_ifb = Command::new("sh")
        .arg("-c")
        .arg("sudo ip link add ifb0 type ifb; sudo ip link set dev ifb0 up")
        .output()?;

    if !setup_ifb.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to setup ifb0 device",
        ));
    }

    // Redirect ingress traffic from the specified interface to ifb0
    let redirect_ingress = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc add dev {} handle ffff: ingress; \
            sudo tc filter add dev {} parent ffff: protocol ip u32 match u32 0 0 action mirred egress redirect dev ifb0",
            interface, interface
        ))
        .output()?;

    if !redirect_ingress.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to redirect ingress traffic",
        ));
    }

    // Set up token bucket filter on ifb0 to limit bandwidth
    let limit_ifb = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc add dev ifb0 root handle 1: tbf rate {}mbit burst 32kbit latency 400ms",
            limit_mbps
        ))
        .output()?;

    if limit_ifb.status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to set ingress bandwidth limit: {}",
                String::from_utf8_lossy(&limit_ifb.stderr)
            ),
        ))
    }
}
pub fn reset_ingress_bandwidth_limit(interface: &str) -> Result<(), Error> {
    // Remove qdisc and filter from ifb0
    let remove_ifb_qdisc = Command::new("sh")
        .arg("-c")
        .arg("sudo tc qdisc del dev ifb0 root")
        .output()?;
    
    if !remove_ifb_qdisc.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to remove qdisc from ifb0"
        ));
    }

    // Remove ingress qdisc and filter from the specified interface
    let remove_ingress = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo tc qdisc del dev {} ingress",
            interface
        ))
        .output()?;
    
    if !remove_ingress.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to remove ingress qdisc"
        ));
    }

    // Delete the ifb0 interface
    let delete_ifb = Command::new("sh")
        .arg("-c")
        .arg("sudo ip link del ifb0")
        .output()?;
    
    if !delete_ifb.status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to delete ifb0 interface"
        ));
    }

    // Remove the ifb kernel module
    let remove_ifb_module = Command::new("sudo")
        .arg("rmmod")
        .arg("ifb")
        .output()?;
    
    if remove_ifb_module.status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to remove ifb module"
        ))
    }
}