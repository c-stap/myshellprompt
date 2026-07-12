pub enum OperatingSystem {
    Apple,
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
            "Apple" => OperatingSystem::Apple,
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
        OperatingSystem::Apple => "",
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
