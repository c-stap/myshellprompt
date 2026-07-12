use os_info::Type;

pub fn get_os_icon(os_name: os_info::Type) -> &'static str {
    match os_name {
        Type::Macos => "",
        Type::Windows => "",
        Type::Linux => "",
        Type::Alpine => "",
        Type::Fedora => "",
        Type::Ubuntu => "",
        Type::Arch => "󰣇",
        Type::Debian => "",
        _ => "",
    }
}
