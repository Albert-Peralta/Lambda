use colored::Colorize;
use std::fs;

pub fn print_banner() {
    println!("\n{}", "╔════════════════════════════════════════╗".green());
    println!("{}", "║   TRASH REWARD SYSTEM - TOKEN v0.1.0   ║".bold().green());
    println!("{}", "║  Sistema de Recompensa por Basura      ║".green());
    println!("{}", "║  Powered by Stellar & Soroban         ║".green());
    println!("{}", "╚════════════════════════════════════════╝".green());
    println!();
}

pub fn get_state_animal(state: &str) -> String {
    match state.to_lowercase().as_str() {
        "morelos" => get_morelos_animal(),
        "morelia" => get_morelia_animal(),
        _ => "Estado no reconocido".to_string(),
    }
}

fn get_morelos_animal() -> String {
    r#"
    🐟 MORELOS - SALMÓN 🐟
    
    ╔══════════════════════════════╗
    ║    Token de Morelos          ║
    ║    Animal Simbólico: Salmón  ║
    ║    Valor: $8 USD             ║
    ║    Equivalencia: 15 kg        ║
    ╚══════════════════════════════╝
    "#
    .to_string()
}

fn get_morelia_animal() -> String {
    r#"
    🐢 MORELIA - TORTUGA 🐢
    
    ╔══════════════════════════════╗
    ║    Token de Morelia          ║
    ║    Animal Simbólico: Tortuga ║
    ║    Valor: $8 USD             ║
    ║    Equivalencia: 15 kg        ║
    ╚══════════════════════════════╝
    "#
    .to_string()
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".bold().green(), message.green());
}

pub fn print_error(message: &str) {
    println!("{} {}", "✗".bold().red(), message.red());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".bold().cyan(), message.cyan());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".bold().yellow(), message.yellow());
}