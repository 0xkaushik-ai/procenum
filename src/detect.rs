
pub fn is_suspicious(parent: &str, child: &str) -> bool {
    let office_apps = ["soffice.bin", "libreoffice", "winword.exe", "excel.exe", "powerpnt.exe", "outlook.exe"];
    let shells = ["bash", "zsh", "sh", "cmd.exe", "powershell.exe", "python3", "nc"];

    office_apps.contains(&parent) && shells.contains(&child)
}