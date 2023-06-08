use std::{process::Command, thread, time::Duration};

fn main() -> std::io::Result<()> {
    let ip_list = [
        "192.169.1.83",
        "192.169.1.82",
        "192.169.1.81",
        "192.169.1.80",
        "192.169.1.77",
        "192.169.1.84",
        "192.169.1.85",
        "192.169.1.86",
        "192.169.1.87",
        "192.169.1.88",
    ];
    for ip in ip_list {
        Command::new("adb").args(["connect", ip]).status()?;
    }
    thread::sleep(Duration::from_secs(2));
    for ip in ip_list {
        Command::new("adb")
            .args(["-s", ip, "push", "./tmp/hugep_start.sh", "/data/local/tmp/"])
            .status()?;
        Command::new("adb")
            .args([
                "-s",
                ip,
                "shell",
                "chmod",
                "+x",
                "/data/local/tmp/hugep_start.sh",
            ])
            .status()?;
        Command::new("adb")
            .args(["-s", ip, "shell", "/data/local/tmp/hugep_start.sh"])
            .status()?;
    }
    // for ip in ip_list {
    //     // Command::new("adb").args(["-s", ip, "root"]).status()?;
    //     // Command::new("adb")
    //     //     .args([
    //     //         "-s",
    //     //         ip,
    //     //         "shell",
    //     //         "nohup starthugep.sh > /data/local/tmp/hugep.log 2>&1 &",
    //     //     ])
    //     //     .status()?;
    //     let out = Command::new("adb")
    //         .args([
    //             "-s",
    //             ip,
    //             "shell",
    //             "cat",
    //             "/data/smallp/dianxinfs_arm32_v0.71-arm32/writable/.deviceID",
    //         ])
    //         .output()?;
    //     println!("ip = {ip} sn = {}", String::from_utf8_lossy(&out.stdout));
    //     // Command::new("adb").args(["disconnect", ip]).status()?;
    // }
    // for ip in ip_list {
    //     Command::new("adb").args(["connect", ip]).status()?;
    //     thread::sleep(Duration::from_secs(2));
    //     Command::new("adb").arg("root").status()?;
    //     Command::new("adb")
    //         .args(["push", "./tmp/GoUdev", "/data/local/tmp/"])
    //         .status()?;
    //     Command::new("adb").args(["push", "./tmp/hugep-all_CBJAVR1A03A4720230414144506707_v0.19_v0.33_v0.71_arm32_2023-04-14-14-45.zip", "/data/local/tmp/"]).status()?;
    //     Command::new("adb")
    //         .args(["shell", "chmod", "+x", "/data/local/tmp/GoUdev"])
    //         .status()?;
    //     Command::new("adb").args(["shell", "/data/local/tmp/GoUdev", "-hugepZip", "/data/local/tmp/hugep-all_CBJAVR1A03A4720230414144506707_v0.19_v0.33_v0.71_arm32_2023-04-14-14-45.zip"]).status()?;
    //     Command::new("adb").args(["disconnect", ip]).status()?;
    // }
    Ok(())
}
