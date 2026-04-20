from abyss_python import AbyssEngine, Air, VmBuilder, PyIrExpr, PyIrStmt

def build_language():
    engine = AbyssEngine()


    engine.add_token("Number", r"\d+")
    engine.add_token("Ident", r"\a\w*")

    engine.add_token("Space", r"\s+")
    engine.ignore_token("Space")

    engine.register_number_rule("Number", lambda text: Air.int(int(text)))
    engine.register_ident_rule("Ident", lambda text: Air.var(text))

    engine.define_expr("true", 0, lambda ctx: Air.bool_val(True))
    engine.define_expr("false", 0, lambda ctx: Air.bool_val(False))

    engine.define_expr("{target: expr} = {val: expr}", 2,
                       lambda ctx: Air.assign_from_expr(ctx.get_node("target"), ctx.get_node("val")))

    engine.define_expr("{l: expr} == {r: expr}", 5, lambda ctx: Air.eq(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} != {r: expr}", 5, lambda ctx: Air.neq(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} < {r: expr}", 5, lambda ctx: Air.lt(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} > {r: expr}", 5, lambda ctx: Air.gt(ctx.get_node("l"), ctx.get_node("r")))

    engine.define_expr("{l: expr} + {r: expr}", 10, lambda ctx: Air.add(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} - {r: expr}", 10, lambda ctx: Air.sub(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} * {r: expr}", 20, lambda ctx: Air.mul(ctx.get_node("l"), ctx.get_node("r")))
    engine.define_expr("{l: expr} / {r: expr}", 20, lambda ctx: Air.div(ctx.get_node("l"), ctx.get_node("r")))

    engine.define_expr("{func: expr} ( $({args: expr}) , * )", 30,
                       lambda ctx: Air.call_expr(ctx.get_node("func"), ctx.get_node_list("args")))

    engine.define_stmt("let {name: ident} = {val: expr} ;",
                       lambda ctx: Air.var_dec(ctx.get_ident("name"), ctx.get_node("val")))

    engine.define_stmt("if ( {cond: expr} ) { $({then: stmt}) } else { $({els: stmt}) }",
                       lambda ctx: Air.if_stmt(ctx.get_node("cond"), ctx.get_node_list("then"), ctx.get_node_list("els")))

    engine.define_stmt("if ( {cond: expr} ) { $({then: stmt}) }",
                       lambda ctx: Air.if_stmt(ctx.get_node("cond"), ctx.get_node_list("then"), []))

    engine.define_stmt("while ( {cond: expr} ) do $({body: stmt}) end",
                       lambda ctx: Air.while_stmt(ctx.get_node("cond"), ctx.get_node_list("body")))

    return engine

def run_script():
    engine = build_language()

    source_code = """
        let limit = 5;
        let counter = 0;
        let is_running = true;
        while (is_running) do

        end

    """

    print("Parsing Source Code...")
    raw_ast = engine.parse(source_code)

    compiled_stmts = []
    for node in raw_ast:
        if isinstance(node, PyIrExpr):
            compiled_stmts.append(Air.expr_stmt(node))
        elif isinstance(node, PyIrStmt):
            compiled_stmts.append(node)
        else:
            raise TypeError("Unknown AST Node Type")

    print("Parsed Successfully. Building VM...")

    vm = VmBuilder()

    def host_print(num, is_final):
        if is_final:
            print(f"🛑 Final Value Reached: {num}")
        else:
            print(f"🔄 Counting... {num}")
        return 0

    vm.register_function("print_msg", ["int", "bool"], "int", host_print)

    vm.compile_and_run(compiled_stmts)

if __name__ == "__main__":
    run_script()
