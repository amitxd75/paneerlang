use crate::ast::*;
use crate::lexer::Lexer;
use crate::utils::colors::PaneerColors;

pub struct DebugInfo {
    pub enabled: bool,
}

impl DebugInfo {
    pub fn new(enabled: bool) -> Self {
        DebugInfo { enabled }
    }

    pub fn print_phase(&self, phase: &str) {
        if self.enabled {
            println!(
                "🔧 {}",
                PaneerColors::debug_phase(&format!("Phase: {}", phase))
            );
        }
    }

    pub fn print_lexer_info(&self, source: &str) {
        if !self.enabled {
            return;
        }

        println!("{}", PaneerColors::debug_phase("=== LEXER ANALYSIS ==="));

        match Lexer::new(source) {
            Ok(mut lexer) => {
                let mut token_count = 0;
                let mut tokens = Vec::new();

                // Collect all tokens for analysis
                while !lexer.is_at_end() {
                    if let Some(token) = lexer.advance() {
                        tokens.push(token.clone());
                        token_count += 1;
                    }
                }

                println!(
                    "{} {}",
                    PaneerColors::debug_info("📊 Total tokens:"),
                    PaneerColors::number_literal(&token_count.to_string())
                );

                // Show first few tokens
                println!(
                    "{}",
                    PaneerColors::debug_info("🔍 Token stream (first 10):")
                );
                for (i, token) in tokens.iter().take(10).enumerate() {
                    println!(
                        "  {} {} {:?}",
                        PaneerColors::number_literal(&format!("{}:", i + 1)),
                        PaneerColors::debug_info("→"),
                        token
                    );
                }

                if tokens.len() > 10 {
                    println!(
                        "  {} {} more tokens...",
                        PaneerColors::debug_info("..."),
                        PaneerColors::number_literal(&(tokens.len() - 10).to_string())
                    );
                }

                println!(
                    "{} {}",
                    PaneerColors::debug_success("✅ Lexer:"),
                    PaneerColors::debug_success("PASSED")
                );
            }
            Err(e) => {
                println!(
                    "{} {}",
                    PaneerColors::error("❌ Lexer:"),
                    PaneerColors::error(&format!("FAILED - {}", e))
                );
            }
        }
        println!();
    }

    pub fn print_parser_info(&self, success: bool, ast_nodes: Option<usize>) {
        if !self.enabled {
            return;
        }

        println!("{}", PaneerColors::debug_phase("=== PARSER ANALYSIS ==="));

        if success {
            if let Some(node_count) = ast_nodes {
                println!(
                    "{} {}",
                    PaneerColors::debug_info("🌳 AST nodes created:"),
                    PaneerColors::number_literal(&node_count.to_string())
                );
            }
            println!(
                "{} {}",
                PaneerColors::debug_success("✅ Parser:"),
                PaneerColors::debug_success("PASSED")
            );
        } else {
            println!(
                "{} {}",
                PaneerColors::error("❌ Parser:"),
                PaneerColors::error("FAILED")
            );
        }
        println!();
    }

    pub fn print_interpreter_info(&self, success: bool) {
        if !self.enabled {
            return;
        }

        println!(
            "{}",
            PaneerColors::debug_phase("=== INTERPRETER ANALYSIS ===")
        );

        if success {
            println!(
                "{} {}",
                PaneerColors::debug_success("✅ Interpreter:"),
                PaneerColors::debug_success("PASSED")
            );
        } else {
            println!(
                "{} {}",
                PaneerColors::error("❌ Interpreter:"),
                PaneerColors::error("FAILED")
            );
        }
        println!();
    }

    pub fn print_ast_structure(&self, program: &Program) {
        if !self.enabled {
            return;
        }

        println!("{}", PaneerColors::debug_phase("=== AST STRUCTURE ==="));
        println!(
            "{} {}",
            PaneerColors::debug_info("📋 Statements:"),
            PaneerColors::number_literal(&program.statements.len().to_string())
        );

        for (i, stmt) in program.statements.iter().enumerate() {
            let stmt_type = match stmt {
                Statement::VarDecl { .. } => "Variable Declaration",
                Statement::FuncDecl { .. } => "Function Declaration",
                Statement::ExprStmt { .. } => "Expression Statement",
                Statement::IfStmt { .. } => "If Statement",
                Statement::ReturnStmt { .. } => "Return Statement",
                Statement::WhileStmt { .. } => "While Loop",
                Statement::ForStmt { .. } => "For Loop",
            };

            println!(
                "  {} {} {}",
                PaneerColors::number_literal(&format!("{}:", i + 1)),
                PaneerColors::debug_info("→"),
                PaneerColors::highlight(stmt_type)
            );
        }
        println!();
    }

    pub fn print_execution_summary(&self, success: bool, duration: Option<std::time::Duration>) {
        if !self.enabled {
            return;
        }

        println!("{}", PaneerColors::debug_phase("=== EXECUTION SUMMARY ==="));

        if let Some(dur) = duration {
            println!(
                "{} {}ms",
                PaneerColors::debug_info("⏱️  Execution time:"),
                PaneerColors::number_literal(&dur.as_millis().to_string())
            );
        }

        if success {
            println!(
                "{} {}",
                PaneerColors::debug_success("🎯 Overall result:"),
                PaneerColors::debug_success("SUCCESS")
            );
        } else {
            println!(
                "{} {}",
                PaneerColors::error("💥 Overall result:"),
                PaneerColors::error("FAILED")
            );
        }
        println!();
    }
}
