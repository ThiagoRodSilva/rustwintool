use crate::services::{execute, comands};
use crate::shell::SHELL;
use std::io;


pub fn ui() {
    loop {
        execute("cls");
        println!("RustWinTool");
        println!("----------------");

        let mut n: u8 = 1;
        for i in SHELL {
            println!("{} - {}", n, i.0);
            n += 1;
        }
        println!("Digite (S) para sair.");
        println!("Execute esse comando para configurações mais avançadas: irm christitus.com/win | iex");
        println!("----------------");
        println!("Escolha um comando: ");
        let mut r_user = String::new();

        io::stdin()
            .read_line(&mut r_user)
            .expect("Numero inválido");

         if r_user.trim() == "S" || r_user.trim() == "s" {
            println!("Saindo...");
            break;
        }

        let n_input: u8 = r_user.trim().parse().expect("Numero inválido");

        if n_input >= 1 && n_input <= n {
            println!("Executando comando: {}", SHELL[(n_input - 1) as usize].0);
            execute(SHELL[(n_input - 1) as usize].1);
            
        } else {
            println!("Numero inválido, tente novamente.");
            continue;
        }

        execute("cls");
        comands(SHELL.to_vec());
    }
    
}