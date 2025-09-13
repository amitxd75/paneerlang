# 🧀 PaneerLang

> **Programming with a Desi Twist!** 🇮🇳

A toy programming language with Hindi-inspired syntax, built in Rust.

## 🎬 Showcase

![PaneerLang Demo](./assets/demo.gif)

> **📹 See PaneerLang in action!** Hindi keywords, syntax highlighting, and automatic type conversion.

## 🎯 What is this?

PaneerLang is an interpreted programming language that uses Hindi keywords. A project for exploring language design and interpreter implementation in Rust.

## ✨ Features

- **Hindi Keywords**: `ye` (let), `agar` (if), `varna` (else), `paneer.bol()` (print)
- **Strong Type System**: `int`, `float`, `string`, `bool`, `array<T>` with type safety
- **Automatic Type Conversion**: Print any type, concatenate strings with numbers
- **Functions**: With parameters, return types, and recursion support
- **Control Flow**: If/else statements, while loops, and for loops
- **Array Operations**: Declaration, access, and iteration
- **REPL**: Interactive mode for quick testing
- **Syntax Highlighting**: Beautiful colored output in debug mode
- **Helpful Errors**: Hinglish error messages that actually help

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/amitxd75/paneerlang
cd paneerlang
cargo build

# Run an example
cargo run example.paneer

# Try the REPL
cargo run -- --repl

# Debug mode (with syntax highlighting)
cargo run -- example.paneer --debug
```

## 📝 Basic Syntax

### Variables
```paneer
ye name: string = "Amit";
ye age: int = 25;
ye height: float = 5.8;
ye isAwesome: bool = true;
ye numbers: array<int> = [1, 2, 3, 4, 5];
```

### Functions
```paneer
func add(a int, b int) int {
    return a + b;
}

// Hindi return syntax
func multiply(x int, y int) int {
    wapas kar x * y;  // "wapas kar" = return
}
```

### Control Flow
```paneer
agar age >= 18 {
    paneer.bol("You can vote!");
} varna {
    paneer.bol("Too young!");
}

// While loops
ye counter: int = 0;
jabtak counter < 3 {
    paneer.bol("Count: " + counter);
    counter = counter + 1;  // Note: assignment not yet supported
}
```

### Arrays and Loops
```paneer
ye numbers: array<int> = [1, 2, 3, 4, 5];
ye names: array<string> = ["Amit", "Alex", "Steve"];

// Array access
ye first: int = numbers[0];        // Gets 1
ye second: string = names[1];      // Gets "Alex"

// Array iteration
har num mein numbers {  // "har...mein" = for...in
    paneer.bol("Number: " + num);   // Automatic type conversion!
}
```

## 🎮 Examples

Check out the examples:
- `example.paneer` - Comprehensive demo
- `examples/calculator.paneer` - Basic calculator
- `examples/strings.paneer` - String operations
- `examples/advanced_features.paneer` - Advanced language features

### 🎥 Quick Demo

```bash
# Try this quick demo to see PaneerLang in action:
cargo run -- --repl

# Then type:
ye naam: string = "PaneerLang";
ye version: int = 1;
paneer.bol("Hello from " + naam + " v" + version + "!");
paneer.bol(naam);     // Direct printing with auto-conversion
paneer.bol(version);  // Prints: 1

# Or run the full example:
cargo run example.paneer
```

## 🛠️ CLI Commands

```bash
cargo run <file.paneer>           # Run a file
cargo run -- --repl              # Interactive mode
cargo run -- <file> --debug      # Debug with syntax highlighting
```

## 🏗️ How it works

1. **Lexer** - Tokenizes source using the `logos` crate
2. **Parser** - Recursive descent parser builds an AST
3. **Interpreter** - Tree-walking interpreter executes the code

Built with:
- **Rust** - For memory safety and performance
- **logos** - Fast lexical analysis
- **clap** - CLI argument parsing
- **colored** - Terminal colors

## 🐛 Error Messages

Get helpful errors in Hinglish:

```
🚨 PaneerLang Error!
💬 Arre yaar, semicolon (;) lagana bhool gaye!
   Line khatam karne ke liye semicolon zaroori hai.

📁 File: example.paneer
📍 Line: 5
```

## 🎨 Syntax Highlighting

Debug mode shows beautiful colored syntax:
- Keywords in blue
- Strings in green
- Numbers in cyan
- Comments in gray
- And more!

## ✨ Automatic Type Conversion

PaneerLang makes printing and string operations easy with automatic type conversion:

```paneer
ye number: int = 42;
ye pi: float = 3.14;
ye flag: bool = true;
ye items: array<int> = [1, 2, 3];

// Direct printing - any type!
paneer.bol(number);  // Prints: 42
paneer.bol(pi);      // Prints: 3.14
paneer.bol(flag);    // Prints: true
paneer.bol(items);   // Prints: [1, 2, 3]

// String concatenation with any type
paneer.bol("Number: " + number);    // Prints: Number: 42
paneer.bol("Pi: " + pi);            // Prints: Pi: 3.14
paneer.bol("Flag: " + flag);        // Prints: Flag: true
paneer.bol("Items: " + items);      // Prints: Items: [1, 2, 3]
```

## 🤔 Why?

- Learn about programming language design
- Practice Rust programming
- Explore Hindi-inspired syntax

## 📝 Project Structure

```
src/
├── main.rs              # CLI and entry point
├── lexer.rs             # Tokenization
├── parser.rs            # Parsing to AST
├── ast.rs               # Abstract syntax tree
├── debug.rs             # Debug output
├── ui.rs                # User interface
├── interpreter/         # Code execution
│   └── mod.rs
├── errors/              # Error handling
│   ├── mod.rs
│   ├── funny_errors.rs
│   └── hinglish_errors.rs
└── utils/               # Utilities
    ├── mod.rs
    ├── colors.rs
    └── syntax_highlighter.rs
```

## 🎯 Language Reference

### Keywords
- `ye` - variable declaration
- `agar` - if statement
- `varna` - else
- `func` - function
- `return` / `wapas kar` - return (Hindi style)
- `jabtak` - while loop
- `har...mein` - for loop
- `paneer.bol()` - print with auto-conversion

### Types
- `int` - integers (auto-converts to string)
- `float` - floating point (auto-converts to string)
- `string` - text
- `bool` - true/false (auto-converts to string)
- `array<T>` - typed arrays with access and iteration

### Operators
- Arithmetic: `+`, `-`, `*`, `/`
- String concatenation: `+` (with automatic type conversion)
- Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Logical: `!`
- Array access: `array[index]`

A simple toy language for learning and experimentation.

---

**Made with ❤️ and 🧀**
