use::std::fs;
use std::env;

fn get_linux_distro() -> String {
    let file_path = "/etc/os-release";
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let mut distro = String::new();
    for line in content.lines() {
        if line.starts_with("ID=") {
            distro = line.chars().skip(3).collect();
            distro = distro.trim().to_string();
        }
    }
    distro
}

pub fn get_os_type() -> OperatingSystem {
    let operating_sys_type = env::consts::OS;
    if operating_sys_type == "windows" {
        OperatingSystem::Windows
    } else if operating_sys_type == "macos" {
        OperatingSystem::Macos
    } else if operating_sys_type == "linux" {
        let distro_str = get_linux_distro();
        match distro_str.as_str() {
            "alpine" => OperatingSystem::Alpine,
            "fedora" => OperatingSystem::Fedora,
            "ubuntu" => OperatingSystem::Ubuntu,
            "arch" => OperatingSystem::Arch,
            "debian" => OperatingSystem::Debian,
            _ => OperatingSystem::Linux,
        }

    } else {
        OperatingSystem::Other
    }
}

pub enum OperatingSystem {
    Macos,
    Windows,
    Linux,
    Alpine,
    Fedora,
    Ubuntu,
    Arch,
    Debian,
    Other,
}

pub fn get_os_icon(os_name: OperatingSystem) -> &'static str {
    match os_name {
        OperatingSystem::Macos => "",
        OperatingSystem::Windows => "",
        OperatingSystem::Linux => "",
        OperatingSystem::Alpine => "",
        OperatingSystem::Fedora => "",
        OperatingSystem::Ubuntu => "",
        OperatingSystem::Arch => "󰣇",
        OperatingSystem::Debian => "",
        OperatingSystem::Other => "",
    }
}
