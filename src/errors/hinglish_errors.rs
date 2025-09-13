use crate::utils::colors::PaneerColors;
use rand::Rng;

pub struct HinglishErrorGenerator;

impl HinglishErrorGenerator {
    pub fn new() -> Self {
        HinglishErrorGenerator
    }

    pub fn translate_error(&self, original_error: &str) -> String {
        // Convert technical errors to Hinglish
        if original_error.contains("Expected ';'") {
            self.get_semicolon_error()
        } else if original_error.contains("Expected '}'") {
            self.get_brace_error()
        } else if original_error.contains("Expected ')'") {
            self.get_paren_error()
        } else if original_error.contains("Undefined variable") {
            self.get_undefined_var_error()
        } else if original_error.contains("Type mismatch") {
            self.get_type_error()
        } else if original_error.contains("Expected expression") {
            self.get_expression_error()
        } else if original_error.contains("Expected variable name") {
            self.get_var_name_error()
        } else if original_error.contains("Expected function name") {
            self.get_func_name_error()
        } else if original_error.contains("Division by zero") {
            self.get_division_error()
        } else if original_error.contains("Array index out of bounds") {
            self.get_array_bounds_error()
        } else {
            self.get_general_error()
        }
    }

    fn get_semicolon_error(&self) -> String {
        let messages = vec![
            "Arre yaar, semicolon (;) lagana bhool gaye! Line khatam karne ke liye semicolon zaroori hai.",
            "Bhai semicolon (;) missing hai! PaneerLang mein har statement ke baad semicolon lagana padta hai.",
            "Oops! Semicolon (;) nahi dala. Statement complete karne ke liye semicolon add karo.",
            "Semicolon (;) kahan hai bhai? Code mein har line ke end mein semicolon lagao.",
            "Arre semicolon (;) toh lagao! Bina semicolon ke statement incomplete hai.",
        ];
        self.random_message(&messages)
    }

    fn get_brace_error(&self) -> String {
        let messages = vec![
            "Closing brace (}) missing hai! Har opening brace { ke liye closing brace } zaroori hai.",
            "Bhai, brace (}) band karna bhool gaye! Block complete karne ke liye } lagao.",
            "Arre closing brace (}) kahan hai? Code block properly close karo.",
            "Brace (}) nahi mila! Opening { ke baad closing } lagana zaroori hai.",
            "Code block incomplete hai - closing brace (}) add karo.",
        ];
        self.random_message(&messages)
    }

    fn get_paren_error(&self) -> String {
        let messages = vec![
            "Closing parenthesis ()) missing hai! Har ( ke liye ) lagana zaroori hai.",
            "Bhai parenthesis ()) band karna bhool gaye! Function call complete karo.",
            "Arre closing bracket ()) kahan hai? Expression properly close karo.",
            "Parenthesis ()) balance nahi hai! Opening ( ke baad closing ) lagao.",
            "Function call incomplete hai - closing parenthesis ()) add karo.",
        ];
        self.random_message(&messages)
    }

    fn get_undefined_var_error(&self) -> String {
        let messages = vec![
            "Ye variable define nahi hai bhai! Pehle 'ye variableName: type = value;' se declare karo.",
            "Variable ka naam galat hai ya define nahi kiya! Check karo spelling aur declaration.",
            "Arre ye variable kahan se aaya? Pehle declare karo, phir use karo.",
            "Variable not found! 'ye' keyword se pehle declare karna padega.",
            "Bhai ye variable exist nahi karta! Pehle create karo, phir access karo.",
        ];
        self.random_message(&messages)
    }

    fn get_type_error(&self) -> String {
        let messages = vec![
            "Type match nahi kar raha! Int mein string ya string mein int nahi dal sakte.",
            "Galat type assign kar rahe ho! Variable ka type check karo.",
            "Type mismatch hai bhai! Expected aur actual type different hai.",
            "Arre type confusion hai! Variable ka declared type aur value ka type same hona chahiye.",
            "Wrong type diya hai! Variable ke type ke according value do.",
        ];
        self.random_message(&messages)
    }

    fn get_expression_error(&self) -> String {
        let messages = vec![
            "Expression expected tha, kuch aur mil gaya! Proper value ya calculation do.",
            "Yahan expression chahiye tha! Variable name, number, ya calculation lagao.",
            "Koi valid expression nahi mila! Value assign karne ke liye proper expression do.",
            "Expression missing hai! Kuch value ya calculation lagao.",
            "Invalid expression hai! Proper value, variable, ya operation lagao.",
        ];
        self.random_message(&messages)
    }

    fn get_var_name_error(&self) -> String {
        let messages = vec![
            "Variable ka naam nahi diya! 'ye' ke baad variable name lagao.",
            "Variable name missing hai! Proper identifier use karo.",
            "Arre variable ka naam kya hai? 'ye' ke baad name specify karo.",
            "Variable declaration mein name nahi hai! Valid name do.",
            "Variable name chahiye! 'ye' keyword ke baad identifier lagao.",
        ];
        self.random_message(&messages)
    }

    fn get_func_name_error(&self) -> String {
        let messages = vec![
            "Function ka naam nahi diya! 'func' ke baad function name lagao.",
            "Function name missing hai! Proper identifier use karo.",
            "Arre function ka naam kya hai? 'func' ke baad name specify karo.",
            "Function declaration mein name nahi hai! Valid name do.",
            "Function name chahiye! 'func' keyword ke baad identifier lagao.",
        ];
        self.random_message(&messages)
    }

    fn get_division_error(&self) -> String {
        let messages = vec![
            "Zero se divide nahi kar sakte bhai! Mathematics mein ye allowed nahi hai.",
            "Division by zero error! Kisi number ko 0 se divide karna invalid hai.",
            "Arre zero se divide kar rahe ho! Ye operation possible nahi hai.",
            "Math error: Zero se division! Denominator zero nahi hona chahiye.",
            "Invalid division! Zero se koi number divide nahi hota.",
        ];
        self.random_message(&messages)
    }

    fn get_array_bounds_error(&self) -> String {
        let messages = vec![
            "Array index out of range! Array ke size se zyada index access kar rahe ho.",
            "Array bounds error! Index array ke length se kam hona chahiye.",
            "Galat array index! Array mein utne elements nahi hai.",
            "Array index invalid hai! 0 se array.length-1 tak ka index use karo.",
            "Array access error! Index array ke size ke andar hona chahiye.",
        ];
        self.random_message(&messages)
    }

    fn get_general_error(&self) -> String {
        let messages = vec![
            "Kuch toh gadbad hai! Code check karo aur phir se try karo.",
            "Error aa gaya hai! Syntax aur logic check karo.",
            "Something went wrong! Code review karo aur fix karo.",
            "Koi problem hai code mein! Carefully check karo.",
            "Error detected! Code ko dhyan se dekho aur correct karo.",
        ];
        self.random_message(&messages)
    }

    fn random_message(&self, messages: &[&str]) -> String {
        let mut rng = rand::rng();
        let index = rng.random_range(0..messages.len());
        messages[index].to_string()
    }

    pub fn format_hinglish_error(
        &self,
        original_error: &str,
        file: Option<&str>,
        line: Option<usize>,
    ) -> String {
        let hinglish_msg = self.translate_error(original_error);

        let mut result = String::new();

        // Error header
        result.push_str(&format!(
            "{}\n",
            PaneerColors::error("╔══════════════════════════════════════════════════════════════╗")
        ));
        result.push_str(&format!(
            "{}\n",
            PaneerColors::error("║                    🚨 PaneerLang Error! 🚨                   ║")
        ));
        result.push_str(&format!(
            "{}\n",
            PaneerColors::error("╚══════════════════════════════════════════════════════════════╝")
        ));
        result.push('\n');

        // Hinglish message
        result.push_str(&format!(
            "{} {}\n",
            "💬",
            PaneerColors::warning(&hinglish_msg)
        ));
        result.push('\n');

        // File info
        if let Some(file_name) = file {
            result.push_str(&format!(
                "{} {}\n",
                PaneerColors::info("📁 File:"),
                PaneerColors::highlight(file_name)
            ));
        }

        if let Some(line_num) = line {
            result.push_str(&format!(
                "{} {}\n",
                PaneerColors::info("📍 Line:"),
                PaneerColors::number_literal(&line_num.to_string())
            ));
        }

        result.push('\n');

        // Quick fix suggestions
        result.push_str(&format!("{}\n", PaneerColors::info("💡 Quick Fix:")));
        result.push_str(&format!(
            "  {} Code carefully check karo\n",
            PaneerColors::debug("•")
        ));
        result.push_str(&format!(
            "  {} Syntax rules follow karo\n",
            PaneerColors::debug("•")
        ));
        result.push_str(&format!(
            "  {} Examples dekho documentation mein\n",
            PaneerColors::debug("•")
        ));
        result.push('\n');

        result
    }
}
