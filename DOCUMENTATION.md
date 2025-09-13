# 📚 PaneerLang Documentation


## 📝 Language Syntax

```paneer
// Comments start with //
// This is a comment

// Variable declaration
ye variableName: type = value;

// Function declaration
func functionName(param1 type1, param2 type2) returnType {
    // function body
    return value;
}

// Main execution
paneer.bol("Hello, World!");
```

### Keywords Reference

| PaneerLang | English | Usage |
|------------|---------|-------|
| `ye` | let/var | Variable declaration |
| `agar` | if | Conditional statement |
| `varna` | else | Alternative branch |
| `func` | function | Function declaration |
| `return` | return | Return statement |
| `wapas kar` | return | Hindi return statement |
| `jabtak` | while | While loop |
| `har...mein` | for...in | For loop |
| `paneer.bol()` | print() | Print function |

## 🏷️ Data Types

### Primitive Types

#### Integer (`int`)
```paneer
ye age: int = 25;
ye negative: int = -10;
ye zero: int = 0;
```

#### Float (`float`)
```paneer
ye pi: float = 3.14159;
ye temperature: float = -2.5;
```

#### String (`string`)
```paneer
ye name: string = "Amit";
ye greeting: string = "Namaste!";
ye empty: string = "";
```

#### Boolean (`bool`)
```paneer
ye isTrue: bool = true;
ye isFalse: bool = false;
```

### Composite Types

#### Arrays (`array<T>`)
```paneer
ye numbers: array<int> = [1, 2, 3, 4, 5];
ye names: array<string> = ["Amit", "Priya", "Raj"];
ye flags: array<bool> = [true, false, true];
```

## 📦 Variables

### Declaration Syntax
```paneer
ye variableName: type = initialValue;
```

### Examples
```paneer
// Basic declarations
ye name: string = "PaneerLang";
ye version: int = 1;
ye isStable: bool = false;

// Array declarations
ye scores: array<int> = [95, 87, 92];
ye languages: array<string> = ["Hindi", "English"];
```

### Scope Rules
- Variables are scoped to their declaration block
- Function parameters create new scope
- Loop variables are scoped to the loop body

## ⚙️ Functions

### Function Declaration
```paneer
func functionName(param1 type1, param2 type2) returnType {
    // function body
    return value;
}
```

### Examples

#### Basic Function
```paneer
func add(a int, b int) int {
    return a + b;
}
```

#### Function with Hindi Return
```paneer
func multiply(x int, y int) int {
    ye result: int = x * y;
    wapas kar result;  // Hindi return syntax
}
```

#### String Function
```paneer
func greet(name string) string {
    return "Namaste, " + name + "!";
}
```

#### Recursive Function
```paneer
func factorial(n int) int {
    agar n <= 1 {
        return 1;
    } varna {
        return n * factorial(n - 1);
    }
}
```

### Function Calls
```paneer
ye sum: int = add(5, 3);
ye greeting: string = greet("Amit");
ye fact: int = factorial(5);
```

## 🔀 Control Flow

### If-Else Statements

#### Basic If
```paneer
agar condition {
    // code block
}
```

#### If-Else
```paneer
agar condition {
    // if block
} varna {
    // else block
}
```

#### Nested If-Else
```paneer
agar score >= 90 {
    paneer.bol("Grade: A");
} varna {
    agar score >= 80 {
        paneer.bol("Grade: B");
    } varna {
        agar score >= 70 {
            paneer.bol("Grade: C");
        } varna {
            paneer.bol("Grade: F");
        }
    }
}
```

### Loops

#### While Loop (`jabtak`)
```paneer
ye counter: int = 0;
jabtak counter < 5 {
    paneer.bol("Count: " + counter);
    counter = counter + 1;  // Note: Assignment not yet implemented
}
```

#### For Loop (`har...mein`)
```paneer
ye numbers: array<int> = [1, 2, 3, 4, 5];
har num mein numbers {
    paneer.bol("Number: " + num);
}
```

## 📊 Arrays

### Declaration
```paneer
ye arrayName: array<type> = [element1, element2, element3];
```

### Examples
```paneer
ye numbers: array<int> = [1, 2, 3, 4, 5];
ye names: array<string> = ["Alice", "Bob", "Charlie"];
ye flags: array<bool> = [true, false, true];
```

### Array Access
```paneer
ye firstNumber: int = numbers[0];
ye secondName: string = names[1];
```

### Array Iteration
```paneer
har element mein arrayName {
    // process element
    paneer.bol("Element: " + element);
}
```

## 🛠️ Built-in Functions

### Built-in Functions (`paneer.bol()`)

The primary output function in PaneerLang with automatic type conversion.

```paneer
// Print string literals
paneer.bol("Hello, World!");

// Print variables of any type
ye name: string = "Amit";
ye age: int = 25;
ye pi: float = 3.14;
ye flag: bool = true;

paneer.bol(name);    // Prints: Amit
paneer.bol(age);     // Prints: 25
paneer.bol(pi);      // Prints: 3.14
paneer.bol(flag);    // Prints: true

// Print with automatic type conversion in concatenation
paneer.bol("Name: " + name);     // String + String
paneer.bol("Age: " + age);       // String + Int (auto-converted)
paneer.bol("Pi: " + pi);         // String + Float (auto-converted)
paneer.bol("Flag: " + flag);     // String + Bool (auto-converted)

// Print arrays
ye numbers: array<int> = [1, 2, 3];
paneer.bol(numbers);  // Prints: [1, 2, 3]
```

### Features
- **Automatic type conversion** for all primitive types
- **String concatenation** with any type using `+` operator
- **Array printing** with formatted output
- Single argument only

## 💻 CLI Reference

### Basic Usage
```bash
paneerlang [OPTIONS] [FILE]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version |
| `--repl` | `-r` | Start interactive REPL |
| `--debug` | `-d` | Enable debug mode |

### Examples

```bash
# Run a PaneerLang file
cargo run example.paneer

# Start REPL mode
cargo run -- --repl

# Run with debug output
cargo run -- example.paneer --debug

# Show help
cargo run -- --help
```

## 🔄 REPL Guide

The Read-Eval-Print Loop (REPL) allows interactive programming.

### Starting REPL
```bash
cargo run -- --repl
```

### REPL Commands

```paneer
paneer> ye name: string = "Amit";
paneer> paneer.bol("Hello, " + name + "!");
Hello, Amit!

paneer> func square(x int) int { return x * x; }
paneer> ye result: int = square(5);
paneer> paneer.bol("Result: 25");
Result: 25

paneer> help
# Shows help information

paneer> exit
Goodbye!
```

### REPL Features
- **Auto-semicolon** - Adds semicolons for single expressions
- **Persistent state** - Variables and functions persist across commands
- **Error recovery** - Continues after errors
- **Help system** - Built-in help command

## 📚 Examples

### Hello World
```paneer
paneer.bol("Namaste, World!");
```

### Variables and Types
```paneer
ye name: string = "PaneerLang";
ye version: int = 1;
ye isAwesome: bool = true;

paneer.bol("Language: " + name);
paneer.bol("Version: 1");
paneer.bol("Awesome: true");
```

### Functions
```paneer
func calculateArea(length int, width int) int {
    return length * width;
}

ye area: int = calculateArea(10, 5);
paneer.bol("Area: 50");
```

### Control Flow
```paneer
ye age: int = 18;

agar age >= 18 {
    paneer.bol("You can vote!");
} varna {
    paneer.bol("Too young to vote.");
}
```

### Arrays and Loops
```paneer
ye fruits: array<string> = ["apple", "banana", "orange"];

har fruit mein fruits {
    paneer.bol("Fruit: " + fruit);
}
```

### Recursive Functions
```paneer
func fibonacci(n int) int {
    agar n <= 1 {
        return n;
    } varna {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

ye fib: int = fibonacci(10);
paneer.bol("Fibonacci(10): 55");
```

## 🏗️ Architecture

### Components

1. **Lexer** (`src/lexer.rs`)
   - Tokenizes source code using the `logos` crate
   - Converts text into tokens for parsing

2. **Parser** (`src/parser.rs`)
   - Recursive descent parser
   - Builds Abstract Syntax Tree (AST)

3. **AST** (`src/ast.rs`)
   - Defines language constructs
   - Type definitions and implementations

4. **Interpreter** (`src/interpreter.rs`)
   - Tree-walking interpreter
   - Executes the AST directly

5. **Error Handling** (`src/errors/`)
   - Funny and Hinglish error messages
   - User-friendly error reporting

6. **Syntax Highlighting** (`src/syntax_highlighter.rs`)
   - Colorized output for debug mode
   - Improves code readability

### Data Flow

```
Source Code → Lexer → Tokens → Parser → AST → Interpreter → Output
```

### Dependencies

- **logos** - Fast lexical analysis
- **anyhow** - Error handling
- **clap** - CLI argument parsing
- **colored** - Terminal colors
- **rand** - Random number generation

### Current Limitations

1. **No Variable Assignment** - Only declarations supported
2. **Limited Built-ins** - Only `paneer.bol()` available
3. **No Modules** - Single file programs only
4. **No Standard Library** - Minimal built-in functionality
5. **No Nested Arrays** - Arrays of arrays not fully supported
