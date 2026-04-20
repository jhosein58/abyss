#!/usr/bin/env python3
"""
Advanced Abyss Demo - Build a complete scripting language in ~80 lines!
"""

from abyss_python import Abyss, IR

# More complex program with nested logic
source = """
let fibonacci = 10;
let a = 0;
let b = 1;
let counter = 0;

print(a);
print(b);

while (counter < fibonacci) {
    let temp = a + b;
    a = b;
    b = temp;
    print(temp);
    counter = counter + 1;
}

print(999);
"""

print("=" * 60)
print("🌟 Abyss Script - Dynamic Language Engine")
print("=" * 60)
print("\n📝 Source Code:")
print(source)
print("\n" + "=" * 60)

# Build the language
lang = Abyss(source)

# Lexer: Define all tokens
lang.token("Number", r"\d+")
lang.token("Ident", r"\a\w*")
lang.token("Space", r"\s+")
lang.ignore("Space")

# Parser: Literals
lang.number("Number", lambda text: IR.int(int(text)))
lang.ident("Ident", lambda text: IR.var(text))

# Parser: Binary operators (with precedence)
lang.expr(":l + :r", 10, lambda ctx: IR.add(ctx.node("l"), ctx.node("r")))
lang.expr(":l - :r", 10, lambda ctx: IR.sub(ctx.node("l"), ctx.node("r")))
lang.expr(":l * :r", 20, lambda ctx: IR.mul(ctx.node("l"), ctx.node("r")))
lang.expr(":l / :r", 20, lambda ctx: IR.div(ctx.node("l"), ctx.node("r")))
lang.expr(":l < :r", 5, lambda ctx: IR.lt(ctx.node("l"), ctx.node("r")))
lang.expr(":l > :r", 5, lambda ctx: IR.gt(ctx.node("l"), ctx.node("r")))
lang.expr(":l == :r", 5, lambda ctx: IR.eq(ctx.node("l"), ctx.node("r")))

# Parser: Assignment
lang.expr(":target = :val", 2, lambda ctx: IR.assign(ctx.node("target"), ctx.node("val")))

# Parser: Function calls
lang.expr(":func ( $(:args),* )", 30, lambda ctx: IR.call(ctx.node("func"), ctx.nodes("args")))

# Parser: Statements
lang.stmt("let @name = :val", lambda ctx: IR.var_decl(ctx.ident("name"), ctx.node("val")))

lang.stmt("if ( :cond ) { $(:then_body);* } else { $(:else_body);* }",
    lambda ctx: IR.if_stmt(ctx.node("cond"), ctx.nodes("then_body"), ctx.nodes("else_body")))
lang.stmt("while ( :cond ) { $(:body);* }",
    lambda ctx: IR.while_stmt(ctx.node("cond"), ctx.nodes("body")))

# Host function injected from Python
def print_func(args):
    print(f"  → {args[0]}")

# Execute!
print("\n🚀 Execution Output:")
print("-" * 60)
lang.run({"print": print_func})
print("-" * 60)
print("\n✅ Execution completed successfully!")
print("\n💡 You just built and ran a complete scripting language!")
