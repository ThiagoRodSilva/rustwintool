use std::collections::HashMap;

pub fn comands(cmd: Vec<(&str, &str)>) {
    let mut comands: HashMap<String, String> = HashMap::new();
    for (command, execution) in cmd {
        comands.insert(command.into(), execution.into());
    }
}

pub fn execute(command: &str) {
    use std::process::Command;
    
    if command == "Repair-WindowsImage -Online -RestoreHealth" {
        Command::new("powershell.exe")
            .args(&["/C", command])
            .status()
            .expect("Falha ao executar o comando");
    } else {
        Command::new("cmd.exe")
            .args(&["/C", command])
            .status()
            .expect("Falha ao executar o comando");
    }
}
