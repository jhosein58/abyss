#!/usr/bin/env python3
"""
Abyss Script - Dynamic Language Builder
Create your own programming language with minimal code!
"""

from abyss_python import Abyss, IR

# Source code in our custom language
source = """
let x = 10;
let y = 20;

print(x + y);

let counter = 0;
while (counter < 5) {
    print(counter);
    counter = counter + 1;
}

if (x > y) {
    print(999);
} else {
    print(888);
}
"""

# Create the language engine
lang = Abyss(source)

# Define tokens (lexer rules)
lang.token("Number", r"\d+")
lang.token("Let", r"let")
lang.token("If", r"if")
lang.token("Else", r"else")
lang.token("While", r"while")
lang.token("Ident", r"\a\w*")
lang.token("Space", r"\s+")

lang.ignore("Space")

# Define how numbers and identifiers are parsed
lang.number("Number", lambda text: IR.int(int(text)))
lang.ident("Ident", lambda text: IR.var(text))

# Define expressions with precedence
lang.expr(":l + :r", 10, lambda ctx: IR.add(ctx.node("l"), ctx.node("r")))
lang.expr(":l - :r", 10, lambda ctx: IR.sub(ctx.node("l"), ctx.node("r")))
lang.expr(":l * :r", 20, lambda ctx: IR.mul(ctx.node("l"), ctx.node("r")))
lang.expr(":l / :r", 20, lambda ctx: IR.div(ctx.node("l"), ctx.node("r")))
lang.expr(":l < :r", 5, lambda ctx: IR.lt(ctx.node("l"), ctx.node("r")))
lang.expr(":l > :r", 5, lambda ctx: IR.gt(ctx.node("l"), ctx.node("r")))
lang.expr(":l == :r", 5, lambda ctx: IR.eq(ctx.node("l"), ctx.node("r")))

# Assignment
lang.expr(":target = :val", 2, lambda ctx: IR.assign(ctx.node("target"), ctx.node("val")))

# Function calls
lang.expr(":func ( $(:args),* )", 30, lambda ctx: IR.call(ctx.node("func"), ctx.nodes("args")))

# Define statements
lang.stmt("let @name = :val", lambda ctx: IR.var_decl(ctx.ident("name"), ctx.node("val")))

lang.stmt("if ( :cond ) { $(:then_body);* } else { $(:else_body);* }", 
    lambda ctx: IR.if_stmt(
        ctx.node("cond"),
        ctx.nodes("then_body"),
        ctx.nodes("else_body")
    ))

lang.stmt("while ( :cond ) { $(:body);* }",
    lambda ctx: IR.while_stmt(ctx.node("cond"), ctx.nodes("body")))

# Define host function (injected from Python)
def print_func(args):
    print(f"🖨️  Output: {args[0]}")

# Run the program!
print("🚀 Running custom language...\n")
lang.run({"print": print_func})
print("\n✅ Done!")
