mod config;
mod state;
mod token;
mod ui;
mod user;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "Trash Reward System")]
#[command(about = "Sistema de recompensa por recolección de residuos basado en tokens")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Registrar usuario
    Register {
        /// Nombre del usuario
        #[arg(short, long)]
        name: String,

        /// Estado del usuario (morelos, morelia)
        #[arg(short, long)]
        state: String,
    },
    /// Transferir tokens a otro usuario
    Transfer {
        /// ID del usuario que envía
        #[arg(short, long)]
        from: String,

        /// ID del usuario que recibe
        #[arg(short, long)]
        to: String,

        /// Cantidad de tokens
        #[arg(short, long)]
        amount: f64,
    },
    /// Recibir tokens por basura recolectada
    Receive {
        /// ID del usuario
        #[arg(short, long)]
        user_id: String,

        /// Kilogramos de basura recolectada
        #[arg(short, long)]
        kg: f64,
    },
    /// Convertir kilogramos a tokens
    Convert {
        /// Kilogramos de basura
        #[arg(short, long)]
        kg: f64,
    },
    /// Ver balance del usuario
    Balance {
        /// ID del usuario
        #[arg(short, long)]
        user_id: String,
    },
    /// Ver historial de transacciones
    History {
        /// ID del usuario
        #[arg(short, long)]
        user_id: Option<String>,
    },
    /// Ver información del usuario
    Info {
        /// ID del usuario
        #[arg(short, long)]
        user_id: String,
    },
}

fn main() {
    ui::print_banner();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Register { name, state }) => {
            handle_register(&name, &state);
        }
        Some(Commands::Transfer { from, to, amount }) => {
            handle_transfer(&from, &to, amount);
        }
        Some(Commands::Receive { user_id, kg }) => {
            handle_receive(&user_id, kg);
        }
        Some(Commands::Convert { kg }) => {
            handle_convert(kg);
        }
        Some(Commands::Balance { user_id }) => {
            handle_balance(&user_id);
        }
        Some(Commands::History { user_id }) => {
            handle_history(user_id);
        }
        Some(Commands::Info { user_id }) => {
            handle_info(&user_id);
        }
        None => {
            interactive_menu();
        }
    }
}

fn interactive_menu() {
    loop {
        println!("\n{}", "=== MENÚ PRINCIPAL ===".bold().green());
        println!("1. Registrar nuevo usuario");
        println!("2. Recibir tokens (reportar basura)");
        println!("3. Transferir tokens");
        println!("4. Ver balance");
        println!("5. Ver historial");
        println!("6. Convertir kilogramos a tokens");
        println!("7. Ver información de usuario");
        println!("8. Salir");
        print!("\n{}", "Selecciona una opción: ".cyan());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error al leer entrada");

        match choice.trim() {
            "1" => menu_register(),
            "2" => menu_receive(),
            "3" => menu_transfer(),
            "4" => menu_balance(),
            "5" => menu_history(),
            "6" => menu_convert(),
            "7" => menu_info(),
            "8" => {
                println!("{}", "\n¡Gracias por usar Trash Reward System!".bold().green());
                break;
            }
            _ => println!("{}", "Opción inválida".red()),
        }
    }
}

fn menu_register() {
    print!("{}", "Nombre del usuario: ".cyan());
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    print!("{}", "Estado (morelos/morelia): ".cyan());
    io::stdout().flush().unwrap();
    let mut state = String::new();
    io::stdin().read_line(&mut state).unwrap();

    handle_register(name.trim(), state.trim());
}

fn menu_receive() {
    print!("{}", "ID del usuario: ".cyan());
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).unwrap();

    print!("{}", "Kilogramos de basura recolectada: ".cyan());
    io::stdout().flush().unwrap();
    let mut kg_str = String::new();
    io::stdin().read_line(&mut kg_str).unwrap();

    if let Ok(kg) = kg_str.trim().parse::<f64>() {
        handle_receive(user_id.trim(), kg);
    } else {
        println!("{}", "Error: Ingresa un número válido".red());
    }
}

fn menu_transfer() {
    print!("{}", "ID del usuario que envía: ".cyan());
    io::stdout().flush().unwrap();
    let mut from = String::new();
    io::stdin().read_line(&mut from).unwrap();

    print!("{}", "ID del usuario que recibe: ".cyan());
    io::stdout().flush().unwrap();
    let mut to = String::new();
    io::stdin().read_line(&mut to).unwrap();

    print!("{}", "Cantidad de tokens: ".cyan());
    io::stdout().flush().unwrap();
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).unwrap();

    if let Ok(amount) = amount_str.trim().parse::<f64>() {
        handle_transfer(from.trim(), to.trim(), amount);
    } else {
        println!("{}", "Error: Ingresa un número válido".red());
    }
}

fn menu_balance() {
    print!("{}", "ID del usuario: ".cyan());
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).unwrap();
    handle_balance(user_id.trim());
}

fn menu_history() {
    print!("{}", "ID del usuario (enter para ver todos): ".cyan());
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).unwrap();

    let user_id = if user_id.trim().is_empty() {
        None
    } else {
        Some(user_id.trim().to_string())
    };

    handle_history(user_id);
}

fn menu_convert() {
    print!("{}", "Kilogramos de basura: ".cyan());
    io::stdout().flush().unwrap();
    let mut kg_str = String::new();
    io::stdin().read_line(&mut kg_str).unwrap();

    if let Ok(kg) = kg_str.trim().parse::<f64>() {
        handle_convert(kg);
    } else {
        println!("{}", "Error: Ingresa un número válido".red());
    }
}

fn menu_info() {
    print!("{}", "ID del usuario: ".cyan());
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).unwrap();
    handle_info(user_id.trim());
}

fn handle_register(name: &str, state: &str) {
    let user = user::User::new(name.to_string(), state.to_string());

    if state.to_lowercase() == "morelos" || state.to_lowercase() == "morelia" {
        println!("\n{}", "=== USUARIO REGISTRADO ===".bold().green());
        println!("ID: {}", user.id.bold());
        println!("Nombre: {}", user.name.bold());
        println!("Estado: {}", user.state.bold());
        println!("Tokens: {}", user.tokens.to_string().bold().yellow());
        println!(
            "{}",
            format!("\n{}", ui::get_state_animal(&user.state)).italic()
        );
    } else {
        println!(
            "{}",
            "Error: Estado no válido. Usa 'morelos' o 'morelia'".red()
        );
    }
}

fn handle_transfer(from_id: &str, to_id: &str, amount: f64) {
    println!("\n{}", "=== TRANSFERENCIA DE TOKENS ===".bold().green());

    if amount <= 0.0 {
        println!("{}", "Error: La cantidad debe ser mayor a 0".red());
        return;
    }

    println!("De: {}", from_id.bold());
    println!("Para: {}", to_id.bold());
    println!("Cantidad: {} tokens", amount.to_string().bold().yellow());

    let usd_value = amount * 8.0;
    println!("Valor en USD: ${}", usd_value.to_string().bold().cyan());

    println!("\n{}", "✓ Transacción completada".bold().green());
}

fn handle_receive(user_id: &str, kg: f64) {
    println!("\n{}", "=== RECEPCIÓN DE TOKENS ===".bold().green());

    if kg <= 0.0 {
        println!("{}", "Error: Debes ingresar una cantidad positiva".red());
        return;
    }

    let tokens = token::kg_to_tokens(kg);
    let usd_value = tokens * 8.0;

    println!("Usuario ID: {}", user_id.bold());
    println!("Basura recolectada: {} kg", kg.to_string().bold().cyan());
    println!("Tokens generados: {}", tokens.to_string().bold().yellow());
    println!("Valor en USD: ${}", usd_value.to_string().bold().green());

    println!("\n{}", "✓ Tokens agregados a tu cuenta".bold().green());
}

fn handle_convert(kg: f64) {
    println!("\n{}", "=== CONVERTIDOR DE BASURA A TOKENS ===".bold().green());

    if kg <= 0.0 {
        println!("{}", "Error: Ingresa una cantidad válida".red());
        return;
    }

    let tokens = token::kg_to_tokens(kg);
    let usd_value = tokens * 8.0;

    println!("Basura: {} kg", kg.to_string().bold().cyan());
    println!("Tokens equivalentes: {}", tokens.to_string().bold().yellow());
    println!(
        "Valor en USD: ${}",
        usd_value.to_string().bold().green()
    );

    println!(
        "\n{}",
        "Nota: Cada token representa 15 kg de basura recolectada".italic()
    );
}

fn handle_balance(user_id: &str) {
    println!("\n{}", "=== BALANCE DEL USUARIO ===".bold().green());
    println!("Usuario ID: {}", user_id.bold());
    println!("Tokens: {}", "0.0".yellow().bold());
    println!("Valor USD: ${}", "0.0".green().bold());
}

fn handle_history(user_id: Option<String>) {
    println!("\n{}", "=== HISTORIAL DE TRANSACCIONES ===".bold().green());

    if let Some(id) = user_id {
        println!("Transacciones del usuario: {}", id.bold());
    } else {
        println!("Todas las transacciones del sistema");
    }

    println!("\n{}", "[Sin transacciones registradas aún]".italic());
}

fn handle_info(user_id: &str) {
    println!("\n{}", "=== INFORMACIÓN DEL USUARIO ===".bold().green());
    println!("Usuario ID: {}", user_id.bold());
    println!("Estado: {}", "morelos".bold());
    println!("Animal simbólico: {}", "Salmón".yellow());
    println!("Tokens: {}", "0.0".yellow().bold());
}