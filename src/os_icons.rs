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

impl OperatingSystem {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Macos" => OperatingSystem::Macos,
            "Windows" => OperatingSystem::Windows,
            "Linux" => OperatingSystem::Linux,
            "Alpine" => OperatingSystem::Alpine,
            "Fedora" => OperatingSystem::Fedora,
            "Ubuntu" => OperatingSystem::Ubuntu,
            "Arch" => OperatingSystem::Arch,
            "Debian" => OperatingSystem::Debian,
            _ => OperatingSystem::Other,
        }
    }
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
