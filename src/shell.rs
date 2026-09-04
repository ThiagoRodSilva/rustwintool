pub const SHELL: &[(&str, &str)] = &[
        ("sfc", "sfc /scannow"),
        ("chkdsk", "chkdsk C: /f /r"),
        ("DISM", "dism /online /cleanup-image /restorehealth"),
        ("Repair-WindowsImage", "Repair-WindowsImage -Online -RestoreHealth")
    ];