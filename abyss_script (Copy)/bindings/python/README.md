# Abyss Script - Dynamic Language Builder 🚀

Build your own programming language with minimal code! Abyss Script is an extremely dynamic scripting engine that lets you define syntax rules at runtime and create custom languages with just a few lines of Python.

## Features ✨

- **Runtime Syntax Definition**: Define lexer and parser rules dynamically
- **Pattern-Based Grammar**: Use simple, magical patterns like `:expr + :expr` to define syntax
- **Pratt Parsing**: Built-in support for operator precedence
- **Python Integration**: Inject Python functions into your custom language
- **Compiled Execution**: Your language compiles to IR and runs on a fast VM

## Installation

```bash
cd bindings/python
maturin develop
```

## Quick Start

```python
from abyss_python import Abyss, IR

# Your custom language source code
source = """
let x = 10;
let y = 20;
print(x + y);
"""

# Create the language engine
lang = Abyss(source)

# Define tokens (lexer)
lang.token("Number", r"\d+")
lang.token("Let", r"let")
lang.token("Ident", r"\a\w*")
lang.token("Space", r"\s+")
lang.ignore("Space")

# Define how to parse numbers and identifiers
lang.number("Number", lambda text: IR.int(int(text)))
lang.ident("Ident", lambda text: IR.var(text))

# Define expressions with precedence
lang.expr(":l + :r", 10, lambda ctx: IR.add(ctx.node("l"), ctx.node("r")))

# Define statements
lang.stmt("let @name = :val", lambda ctx: IR.var_decl(ctx.ident("name"), ctx.node("val")))

# Function calls
lang.expr(":func ( $(:args),* )", 30, lambda ctx: IR.call(ctx.node("func"), ctx.nodes("args")))

# Inject Python function
def print_func(args):
    print(f"Output: {args[0]}")

# Run it!
lang.run({"print": print_func})
```

## Pattern Syntax

### Holes (Placeholders)

- `:name` - Expression hole (matches any expression)
- `@name` - Identifier hole (matches an identifier token)

### Repetition

- `$(:items)*` - Zero or more items (no separator)
- `$(:items ,)*` - Zero or more items separated by commas
- `$(:items ;)*` - Zero or more items separated by semicolons

### Examples

```python
# Binary operators
lang.expr(":l + :r", 10, lambda ctx: IR.add(ctx.node("l"), ctx.node("r")))

# Function calls with arguments
lang.expr(":func ( $(:args),* )", 30, lambda ctx: IR.call(ctx.node("func"), ctx.nodes("args")))

# Variable declaration
lang.stmt("let @name = :val", lambda ctx: IR.var_decl(ctx.ident("name"), ctx.node("val")))

# While loop with body
lang.stmt("while ( :cond ) { $(:body);* }", 
    lambda ctx: IR.while_stmt(ctx.node("cond"), ctx.nodes("body")))

# If-else statement
lang.stmt("if ( :cond ) { $(:then);* } else { $(:else);* }", 
    lambda ctx: IR.if_stmt(ctx.node("cond"), ctx.nodes("then"), ctx.nodes("else")))
```

## API Reference

### Abyss Class

```python
lang = Abyss(source_code)
```

#### Methods

- `token(name, regex)` - Define a token type with regex pattern
- `ignore(name)` - Ignore a token type (e.g., whitespace)
- `number(token_name, callback)` - Define how to parse number tokens
- `ident(token_name, callback)` - Define how to parse identifier tokens
- `expr(pattern, precedence, callback)` - Define an expression rule
- `stmt(pattern, callback)` - Define a statement rule
- `run(host_functions)` - Parse and execute the program

### IR Class

Static methods for building IR nodes:

- `IR.int(value)` - Integer literal
- `IR.bool_val(value)` - Boolean literal
- `IR.var(name)` - Variable reference
- `IR.add(left, right)` - Addition
- `IR.sub(left, right)` - Subtraction
- `IR.mul(left, right)` - Multiplication
- `IR.div(left, right)` - Division
- `IR.eq(left, right)` - Equality
- `IR.lt(left, right)` - Less than
- `IR.gt(left, right)` - Greater than
- `IR.call(func, args)` - Function call
- `IR.var_decl(name, value)` - Variable declaration
- `IR.assign(target, value)` - Assignment
- `IR.if_stmt(cond, then_body, else_body)` - If statement
- `IR.while_stmt(cond, body)` - While loop
- `IR.expr_stmt(expr)` - Expression statement

### Context (ctx) Object

Available in pattern callbacks:

- `ctx.node(name)` - Get a single node by name
- `ctx.nodes(name)` - Get a list of nodes by name
- `ctx.ident(name)` - Get an identifier string by name

## Examples

See `example.py` and `demo.py` for complete working examples including:
- Variables and assignments
- Arithmetic operations
- Conditionals (if/else)
- Loops (while)
- Function calls
- Python function injection

## How It Works

1. **Lexer**: Tokenizes source code using regex patterns
2. **Parser**: Uses Pratt parsing with dynamic rules defined via patterns
3. **IR**: Converts parsed AST to intermediate representation
4. **Compiler**: Compiles IR to bytecode
5. **VM**: Executes bytecode with host function support

## License

Part of the Abyss compiler project.
