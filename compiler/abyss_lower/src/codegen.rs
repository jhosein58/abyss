use std::collections::HashSet;

use abyss_nexus::nexus::{NameId, Nexus, SymbolId, TypeId};

use crate::lowerer::lower_type;

#[derive(Clone, Debug)]
pub struct CValue(pub String);

impl CValue {
    pub fn empty() -> Self {
        CValue(String::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CType {
    Void,
    Bool,

    // Signed Integers
    I8,
    I16,
    I32,
    I64,
    I128,

    // Unsigned Integers
    U8,
    U16,
    U32,
    U64,
    U128,

    // Floats
    F16,
    F32,
    F64,
    F128,

    Struct(String),

    Ptr(Box<CType>),
}

impl CType {
    pub fn to_string(&self) -> String {
        match self {
            CType::Void => "void".to_string(),
            CType::Bool => "bool".to_string(),

            CType::I8 => "int8_t".to_string(),
            CType::I16 => "int16_t".to_string(),
            CType::I32 => "int32_t".to_string(),
            CType::I64 => "int64_t".to_string(),
            CType::I128 => "__int128".to_string(),

            CType::U8 => "uint8_t".to_string(),
            CType::U16 => "uint16_t".to_string(),
            CType::U32 => "uint32_t".to_string(),
            CType::U64 => "uint64_t".to_string(),
            CType::U128 => "unsigned __int128".to_string(),

            CType::F16 => "_Float16".to_string(),
            CType::F32 => "float".to_string(),
            CType::F64 => "double".to_string(),
            CType::F128 => "__float128".to_string(),

            CType::Struct(s) => s.to_owned(),

            CType::Ptr(ptree) => format!("{}*", ptree.to_string()),
        }
    }
}

#[derive(Default)]
pub struct CCodeGen {
    func_forward_decl: String,
    struct_forward_decl: String,
    struct_body: String,
    code: String,
    indent_level: usize,
    temp_counter: usize,
    abyss_main: String,
}

impl CCodeGen {
    pub fn new() -> Self {
        Self::default()
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn new_temp_var(&mut self) -> String {
        let name = format!("_tmp{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn format_args(args: &[(&str, CType)]) -> String {
        if args.is_empty() {
            "void".to_string()
        } else {
            args.iter()
                .map(|(name, ctype)| format!("{} {}", ctype.to_string(), name))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn declare_function(&mut self, name: &str, ret_type: CType, args: &[(&str, CType)]) {
        let args_str = Self::format_args(args);
        let decl = format!("{} {}({});\n", ret_type.to_string(), name, args_str);
        self.func_forward_decl.push_str(&decl);
    }

    pub fn start_function(&mut self, name: &str, ret_type: CType, args: &[(&str, CType)]) {
        self.declare_function(name, ret_type.clone(), args);
        let args_str = Self::format_args(args);
        self.code.push_str(&format!(
            "{} {}({}) {{\n",
            ret_type.to_string(),
            name,
            args_str
        ));
        self.indent_level += 1;
    }

    pub fn end_function(&mut self) {
        self.indent_level -= 1;
        self.code.push_str(&format!("{}}}\n\n", self.indent()));
    }

    pub fn call(&self, func_expr: CValue, args: &[CValue]) -> CValue {
        let args_str = args
            .iter()
            .map(|v| v.0.clone())
            .collect::<Vec<_>>()
            .join(", ");

        CValue(format!("{}({})", func_expr.0, args_str))
    }

    pub fn literal(&self, val: &str) -> CValue {
        CValue(val.to_string())
    }

    pub fn gen_return(&mut self, value: Option<CValue>) {
        self.code.push_str(&format!(
            "{}return {};\n",
            self.indent(),
            value.map(|v| v.0).unwrap_or("".to_string())
        ));
    }

    pub fn create_variable(
        &mut self,
        name: &str,
        var_type: CType,
        init_val: Option<CValue>,
    ) -> CValue {
        if let Some(iv) = init_val {
            self.code.push_str(&format!(
                "{}{} {} = {};\n",
                self.indent(),
                var_type.to_string(),
                name,
                iv.0
            ));
        } else {
            self.code.push_str(&format!(
                "{}{} {};\n",
                self.indent(),
                var_type.to_string(),
                name,
            ));
        }

        CValue(name.to_string())
    }

    pub fn expr(&mut self, e: Option<CValue>) {
        if let Some(CValue(s)) = e {
            self.code.push_str(&format!("{}{};\n", self.indent(), s));
        }
    }

    pub fn gen_while<F>(&mut self, cond: CValue, mut body: F) -> CValue
    where
        F: FnMut(&mut Self),
    {
        self.code
            .push_str(&format!("{}while ({}) {{\n", self.indent(), cond.0));
        self.indent_level += 1;

        body(self);

        self.indent_level -= 1;
        self.code.push_str(&format!("{}}}\n", self.indent()));

        CValue::empty()
    }

    pub fn gen_if_else<F1, F2>(
        &mut self,
        db: &mut Nexus,
        queue: &mut HashSet<SymbolId>,
        type_queue: &mut HashSet<TypeId>,
        condition: CValue,
        result_type: CType,
        mut then_block: F1,
        mut else_block: F2,
    ) -> CValue
    where
        F1: FnMut(&mut Self, &mut Nexus, &mut HashSet<SymbolId>, &mut HashSet<TypeId>) -> CValue,
        F2: FnMut(&mut Self, &mut Nexus, &mut HashSet<SymbolId>, &mut HashSet<TypeId>) -> CValue,
    {
        let mut result_var_name = String::new();

        if result_type != CType::Void {
            result_var_name = self.new_temp_var();
            self.code.push_str(&format!(
                "{}{} {};\n",
                self.indent(),
                result_type.to_string(),
                result_var_name
            ));
        }

        self.code
            .push_str(&format!("{}if ({}) {{\n", self.indent(), condition.0));
        self.indent_level += 1;

        let then_val = then_block(self, db, queue, type_queue);

        if result_type != CType::Void {
            self.code.push_str(&format!(
                "{}{} = {};\n",
                self.indent(),
                result_var_name,
                then_val.0
            ));
        } else {
            self.code
                .push_str(&format!("{}{};\n", self.indent(), then_val.0));
        }

        self.indent_level -= 1;
        self.code
            .push_str(&format!("{}}} else {{\n", self.indent()));
        self.indent_level += 1;

        let else_val = else_block(self, db, queue, type_queue);
        if result_type != CType::Void {
            self.code.push_str(&format!(
                "{}{} = {};\n",
                self.indent(),
                result_var_name,
                else_val.0
            ));
        }

        self.indent_level -= 1;
        self.code.push_str(&format!("{}}}\n", self.indent()));

        CValue(result_var_name)
    }

    pub fn add(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} + {})", lhs.0, rhs.0))
    }

    pub fn sub(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} - {})", lhs.0, rhs.0))
    }

    pub fn mul(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} * {})", lhs.0, rhs.0))
    }

    pub fn div(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} / {})", lhs.0, rhs.0))
    }

    pub fn assign(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        self.code
            .push_str(&format!("{}{} = {};\n", self.indent(), lhs.0, rhs.0));
        CValue(format!("{}", lhs.0))
    }

    pub fn addrof(&mut self, inner: CValue) -> CValue {
        CValue(format!("&({})", inner.0))
    }

    pub fn deref(&mut self, inner: CValue) -> CValue {
        CValue(format!("*({})", inner.0))
    }

    pub fn cmp_lt(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} < {})", lhs.0, rhs.0))
    }

    pub fn cmp_lte(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} <= {})", lhs.0, rhs.0))
    }

    pub fn cmp_gt(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} > {})", lhs.0, rhs.0))
    }

    pub fn cmp_gte(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} >= {})", lhs.0, rhs.0))
    }

    pub fn cmp_eq(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} == {})", lhs.0, rhs.0))
    }

    pub fn cmp_neq(&mut self, lhs: CValue, rhs: CValue) -> CValue {
        CValue(format!("({} != {})", lhs.0, rhs.0))
    }

    pub fn finish(&self) -> String {
        let includes = "#include <stdio.h>\n#include <stdint.h>\n#include <stdbool.h>\n\n";

        let abyss_prelude = r#"

void print_i32(int32_t v) {
    printf("%d\n", v);
}

void print_star() {
    printf("*");
}

void print_new_line() {
    printf("\n");
}

"#;

        let mut res = format!(
            "{}{}\n// Forward Declarations\n{}\n\n{}\n\n{}\n// Implementations\n{}",
            includes,
            abyss_prelude,
            self.struct_forward_decl,
            self.struct_body,
            self.func_forward_decl,
            self.code
        );

        if self.abyss_main != "" {
            res.push_str(&format!(
                "int main(void) {{\n\t{}();\n\treturn 0;\n}}",
                self.abyss_main
            ));
        }

        res
    }

    pub fn abyss_main(&mut self, name: &str) {
        self.abyss_main = name.to_owned();
    }

    pub fn decl_struct(&mut self, name: &str) {
        self.struct_forward_decl
            .push_str(&format!("typedef struct {} {};\n", name, name));
    }

    pub fn def_struct(&mut self, db: &mut Nexus, id: TypeId) {
        let name = db.types.name(id);
        self.decl_struct(&name);

        self.struct_body.push_str(&format!("struct {} {{\n", &name));
        self.indent_level += 1;

        let fields = db.types.get_struct_fields(id);

        let mut queue = HashSet::new();

        for (n, t) in fields {
            let t = lower_type(db, t, &mut queue);

            self.struct_body
                .push_str(&format!("{}{} _f{};\n", self.indent(), t.to_string(), n.0));
        }

        self.indent_level -= 1;

        self.struct_body.push_str("};\n\n");
    }

    #[inline(always)]
    pub fn gen_struct_init(&mut self, fields: &[u32], vals: &[CValue], ty: CType) -> CValue {
        let tmp_name = self.new_temp_var();

        let mut fields_buf = String::from("{\n");

        self.indent_level += 1;

        for (f, v) in fields.iter().zip(vals) {
            fields_buf.push_str(&format!("{}._f{} = {},\n", self.indent(), f, v.0));
        }

        self.indent_level -= 1;

        fields_buf.push_str(&format!("{}}}", self.indent()));

        self.code.push_str(&format!(
            "{}{} {} = {};\n",
            self.indent(),
            ty.to_string(),
            tmp_name,
            fields_buf
        ));

        CValue(tmp_name)
    }

    #[inline(always)]
    pub fn gen_csat(&mut self, lhs: CValue, ty: CType) -> CValue {
        CValue(format!("({}){}", ty.to_string(), lhs.0))
    }

    #[inline(always)]
    pub fn gen_member(&mut self, lhs: CValue, access: NameId) -> CValue {
        CValue(format!("({})._f{}", lhs.0, access.0))
    }
}
