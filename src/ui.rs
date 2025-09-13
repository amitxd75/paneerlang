use crate::utils::syntax_highlighter::*;
use colored::*;

pub fn print_banner() {
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                    🧀 PaneerLang Interpreter                 ║".cyan()
    );
    println!(
        "{}",
        "║                   Programming with a desi twist!            ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

pub fn print_repl_banner() {
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                    🧀 PaneerLang REPL                       ║".cyan()
    );
    println!(
        "{}",
        "║                   Interactive Programming                    ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
    println!(
        "{} {}",
        "🎯 Version:".green().bold(),
        "PaneerLang v0.1.0".blue()
    );
    println!(
        "{} {}",
        "💡 Commands:".green().bold(),
        "'exit' to quit, 'help' for help".yellow()
    );
    println!(
        "{} {}",
        "📝 Syntax:".green().bold(),
        "Hindi-inspired programming language".magenta()
    );
    println!();
}

pub fn print_file_info(file_path: &str, debug: bool) {
    print_banner();
    println!(
        "{} {}",
        "📁 Running file:".green().bold(),
        file_path.yellow()
    );
    println!(
        "{} {}",
        "🚀 Interpreter:".green().bold(),
        "PaneerLang v0.1.0".blue()
    );
    if debug {
        println!("{} {}", "🐛 Debug mode:".red().bold(), "ENABLED".red());
    }
    println!();
}

pub fn print_execution_start() {
    println!("{}", "▶️  Executing...".green());
    println!("{}", "─".repeat(60).bright_black());
}

pub fn print_error_banner() {
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗".red()
    );
    println!(
        "{}",
        "║                    ❌ PaneerLang Error                        ║".red()
    );
    println!(
        "{}",
        "║                   Missing required arguments!               ║".red()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝".red()
    );
    println!();
}

pub fn print_usage() {
    println!("{}", "📝 Usage:".yellow().bold());
    println!("  📁 {} {}", "paneerlang".green(), "<file.paneer>".cyan());
    println!("  💬 {} {}", "paneerlang".green(), "--repl".cyan());
    println!(
        "  🐛 {} {} {}",
        "paneerlang".green(),
        "<file.paneer>".cyan(),
        "--debug".red()
    );
    println!();
    println!("{}", "💡 Examples:".yellow().bold());
    println!("  {} {}", "cargo run".blue(), "example.paneer".cyan());
    println!("  {} {}", "cargo run --".blue(), "--repl".cyan());
    println!(
        "  {} {} {}",
        "cargo run --".blue(),
        "example.paneer".cyan(),
        "--debug".red()
    );
    println!();
}

pub fn print_help() {
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗".green()
    );
    println!(
        "{}",
        "║                    📚 PaneerLang Help Guide                   ║".green()
    );
    println!(
        "{}",
        "║                   Your desi programming companion!           ║".green()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝".green()
    );
    println!();
    println!("{}", "📝 Basic Syntax:".yellow().bold());
    println!(
        "  📌 {} - Variable declaration",
        "ye name: string = \"value\";".cyan()
    );
    println!(
        "  🖨 {} - Print statement",
        "paneer.bol(\"Hello World\");".cyan()
    );
    println!(
        "  ❓ {} - If statement",
        "agar condition { ... } varna { ... }".cyan()
    );
    println!(
        "  ⚙️ {} - Function declaration",
        "func add(a int, b int) int { return a + b; }".cyan()
    );
    println!("  🔁 {} - While loop", "jabtak condition { ... }".cyan());
    println!("  🔄 {} - For loop", "har item mein array { ... }".cyan());
    println!(
        "  📊 {} - Array declaration",
        "ye arr: array<int> = [1, 2, 3];".cyan()
    );
    println!();
    println!("{}", "🏷️ Types:".yellow().bold());
    println!("  🔢 {} - Integer numbers", "int".cyan());
    println!("  🔢 {} - Floating point numbers", "float".cyan());
    println!("  📝 {} - Text strings", "string".cyan());
    println!("  ✅ {} - Boolean values (true/false)", "bool".cyan());
    println!("  📊 {} - Arrays of elements", "array<type>".cyan());
    println!();
    println!("{}", "⚙️ Operators:".yellow().bold());
    println!("  🧮 {} - Arithmetic", "+ - * /".cyan());
    println!("  ⚖️ {} - Comparison", "== != > < >= <=".cyan());
    println!("  🧠 {} - Logical", "!".cyan());
    println!();
    println!("{}", "🔑 Keywords (Hindi-inspired):".yellow().bold());
    println!("  📌 {} - Variable declaration (let/var)", "ye".cyan());
    println!("  ❓ {} - If statement (if)", "agar".cyan());
    println!("  ❌ {} - Else statement (else)", "varna".cyan());
    println!("  ⚙️ {} - Function declaration (function)", "func".cyan());
    println!(
        "  ↩️ {} - Return statement (return)",
        "return / wapas kar".cyan()
    );
    println!("  🔁 {} - While loop (while)", "jabtak".cyan());
    println!("  🔄 {} - For loop (for...in)", "har...mein".cyan());
    println!("  🖨 {} - Print function (print)", "paneer.bol()".cyan());
    println!();
    println!(
        "{}",
        "🎆 Example with Syntax Highlighting:".magenta().bold()
    );
    let example_code = "ye numbers: array<int> = [1, 2, 3];\nhar num mein numbers {\n    paneer.bol(\"Number: \" + num);\n}\nwapas kar true;";
    print_code_block("Sample PaneerLang Code", example_code);
    println!();
}

pub fn print_debug_info(source: &str) {
    println!("{}", "🐛 Debug Information:".red().bold());
    println!("{}", "─".repeat(40).bright_black());
    println!("{} {} bytes", "📏 Source size:".yellow(), source.len());
    println!(
        "{} {} lines",
        "📄 Line count:".yellow(),
        source.lines().count()
    );
    println!("{}", "─".repeat(40).bright_black());
}
