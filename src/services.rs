use std::collections::HashMap;

pub fn comands(cmd: Vec<(&str, &str)>) {
    let mut comands: HashMap<String, String> = HashMap::new();
    for (command, execution) in cmd {
        comands.insert(command.into(), execution.into());
    }
}

pub fn execute(command: &str) {
    use std::process::Command;
        Command::new("powershell.exe")
            .args(&["/C", command])
            .status()
            .expect("Falha ao executar o comando");
}

pub fn seq_execute(commands: Vec<&str>) {
    for command in commands {
        execute(command);
    }
}