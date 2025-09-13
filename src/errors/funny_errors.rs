use colored::*;
use rand::Rng;

pub struct FunnyErrorGenerator {
    syntax_errors: Vec<&'static str>,
    type_errors: Vec<&'static str>,
    undefined_errors: Vec<&'static str>,
    general_errors: Vec<&'static str>,
    encouragements: Vec<&'static str>,
}

impl FunnyErrorGenerator {
    pub fn new() -> Self {
        FunnyErrorGenerator {
            syntax_errors: vec![
                "Oops! Your syntax is more twisted than a Bollywood plot! 🎬",
                "Syntax error detected! Even Google Translate couldn't understand this! 🤖",
                "Your code is speaking a language that even PaneerLang doesn't recognize! 🤔",
                "Syntax mishap! Did you accidentally write this while eating spicy food? 🌶️",
                "Code syntax error! This looks like it was written during a power cut! 💡",
                "Syntax confusion! Your code is more mixed up than a Mumbai local train route! 🚂",
                "Parse error! This code is harder to read than a doctor's handwriting! 👨‍⚕️",
                "Syntax issue! Your brackets are more unbalanced than a rickshaw! 🛺",
            ],
            type_errors: vec![
                "Type mismatch! You're mixing types like mixing dal with ice cream! 🍨",
                "Wrong type detected! This is like putting sugar in sambar! 🍯",
                "Type error! You can't fit an elephant into a rickshaw! 🐘🛺",
                "Type confusion! This is like trying to charge your phone with a banana! 🍌📱",
                "Type mismatch! You're mixing data types like mixing Hindi and Klingon! 🖖",
                "Wrong type! This is like wearing flip-flops to a wedding! 👡💒",
                "Type error! You can't put a string where an int belongs - it's like putting paneer in chai! ☕",
                "Type mismatch! This is more confusing than choosing a movie on Netflix! 🎭",
            ],
            undefined_errors: vec![
                "Variable not found! It's hiding better than your TV remote! 📺",
                "Undefined variable! It vanished like your money after shopping! 💸",
                "Variable missing! Did it go to get milk and never come back? 🥛",
                "Undefined! This variable is more elusive than an auto-rickshaw in the rain! 🌧️🛺",
                "Variable not found! It's playing hide and seek like a cat! 🐱",
                "Undefined variable! It disappeared faster than free food at an office party! 🍕",
                "Variable missing! It's gone like your patience in traffic! 🚗💨",
                "Undefined! This variable is more lost than a tourist in Old Delhi! 🗺️",
            ],
            general_errors: vec![
                "Something went wrong! Even the code is confused! 😵",
                "Error detected! Time for some debugging chai! ☕🐛",
                "Oops! Your code needs some TLC (Tender Loving Code)! 💝",
                "Error found! Don't worry, even Sharma ji's son makes mistakes! 👨‍💻",
                "Something's not right! This error is more unexpected than rain in summer! 🌧️☀️",
                "Code error! Time to channel your inner detective! 🕵️‍♂️",
                "Oops! Your code is having a bad hair day! 💇‍♀️",
                "Error alert! Even the best cooks burn the dal sometimes! 🍲",
            ],
            encouragements: vec![
                "Don't worry, debugging is just problem-solving in disguise! 🎭",
                "Every error is a step closer to success! Keep going! 💪",
                "Remember: Even the best programmers started with 'Hello World' errors! 🌍",
                "Bugs are just features waiting to be fixed! 🐛➡️✨",
                "You're doing great! Rome wasn't built in a day, and neither is good code! 🏛️",
                "Keep calm and code on! You've got this! 🧘‍♂️",
                "Errors are just the universe's way of teaching you patience! 🌌",
                "Every expert was once a beginner who never gave up! 🌱➡️🌳",
            ],
        }
    }

    pub fn get_funny_syntax_error(&self) -> &'static str {
        let mut rng = rand::rng();
        self.syntax_errors[rng.random_range(0..self.syntax_errors.len())]
    }

    pub fn get_funny_type_error(&self) -> &'static str {
        let mut rng = rand::rng();
        self.type_errors[rng.random_range(0..self.type_errors.len())]
    }

    pub fn get_funny_undefined_error(&self) -> &'static str {
        let mut rng = rand::rng();
        self.undefined_errors[rng.random_range(0..self.undefined_errors.len())]
    }

    pub fn get_funny_general_error(&self) -> &'static str {
        let mut rng = rand::rng();
        self.general_errors[rng.random_range(0..self.general_errors.len())]
    }

    pub fn get_encouragement(&self) -> &'static str {
        let mut rng = rand::rng();
        self.encouragements[rng.random_range(0..self.encouragements.len())]
    }

    pub fn format_error(
        &self,
        error_type: &str,
        original_error: &str,
        file: Option<&str>,
        line: Option<usize>,
    ) -> String {
        let funny_message = match error_type.to_lowercase().as_str() {
            "syntax" | "parse" => self.get_funny_syntax_error(),
            "type" => self.get_funny_type_error(),
            "undefined" | "variable" => self.get_funny_undefined_error(),
            _ => self.get_funny_general_error(),
        };

        let encouragement = self.get_encouragement();

        let mut result = String::new();

        // Error header
        result.push_str(&format!(
            "{}\n",
            "╔══════════════════════════════════════════════════════════════╗".red()
        ));
        result.push_str(&format!(
            "{}\n",
            "║                    🚨 PaneerLang Error Alert! 🚨             ║".red()
        ));
        result.push_str(&format!(
            "{}\n",
            "╚══════════════════════════════════════════════════════════════╝".red()
        ));
        result.push('\n');

        // Funny message
        result.push_str(&format!("{} {}\n", "😅".yellow(), funny_message.yellow()));
        result.push('\n');

        // Technical details
        result.push_str(&format!(
            "{} {}\n",
            "🔍 Technical Details:".red().bold(),
            original_error.white()
        ));

        if let Some(file_name) = file {
            result.push_str(&format!("{} {}\n", "📁 File:".blue(), file_name.cyan()));
        }

        if let Some(line_num) = line {
            result.push_str(&format!(
                "{} {}\n",
                "📍 Line:".blue(),
                line_num.to_string().cyan()
            ));
        }

        result.push('\n');

        // Encouragement
        result.push_str(&format!("{} {}\n", "💡".green(), encouragement.green()));
        result.push('\n');

        // Tips
        result.push_str(&format!("{}\n", "🎯 Quick Tips:".yellow().bold()));
        result.push_str(&format!("  {} Check your syntax carefully\n", "•".yellow()));
        result.push_str(&format!(
            "  {} Make sure all brackets are balanced\n",
            "•".yellow()
        ));
        result.push_str(&format!(
            "  {} Verify variable names and types\n",
            "•".yellow()
        ));
        result.push_str(&format!("  {} Don't forget semicolons!\n", "•".yellow()));
        result.push('\n');

        result
    }

    pub fn format_success_message(&self) -> String {
        let success_messages = [
            "🎉 Shabash! Your code ran successfully!",
            "🌟 Excellent! Your PaneerLang skills are improving!",
            "🚀 Fantastic! Your code is smoother than butter chicken!",
            "✨ Wonderful! Your programming is as sweet as gulab jamun!",
            "🎊 Brilliant! You're coding like a true PaneerLang master!",
            "🏆 Outstanding! Your code is more perfect than a cricket shot!",
            "💫 Superb! Your logic flows like the Ganges!",
            "🎯 Perfect! Your code hits the target like an expert archer!",
        ];

        let mut rng = rand::rng();
        let message = success_messages[rng.random_range(0..success_messages.len())];

        format!("{} {}", "✅".green(), message.green().bold())
    }
}
