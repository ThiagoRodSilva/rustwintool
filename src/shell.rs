pub const SHELL: &[(&str, &str)] = &[
        ("Winget (Atualizar programas instalados)", "winget upgrade --all"),
        ("sfc", "sfc /scannow"),
        ("chkdsk", "chkdsk C: /f /r"),
        ("DISM", "dism /online /cleanup-image /restorehealth"),
        ("Repair-WindowsImage", "Repair-WindowsImage -Online -RestoreHealth"),
        ("Limpar DNS", "ipconfig /flushdns"),
    ];