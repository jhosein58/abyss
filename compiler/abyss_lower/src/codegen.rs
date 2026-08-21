#[derive(Clone, Debug)]
pub struct CValue(pub String);

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
        }
    }
}

#[derive(Default)]
pub struct CCodeGen {
    func_forward_decl: String,
    code: String,
    indent_level: usize,
    temp_counter: usize,
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

    pub fn declare_function(&mut self, name: &str, ret_type: CType) {
        let decl = format!("{} {}();\n", ret_type.to_string(), name);
        self.func_forward_decl.push_str(&decl);
    }

    pub fn start_function(&mut self, name: &str, ret_type: CType) {
        self.declare_function(name, ret_type.clone());
        self.code
            .push_str(&format!("{} {}() {{\n", ret_type.to_string(), name));
        self.indent_level += 1;
    }

    pub fn end_function(&mut self) {
        self.indent_level -= 1;
        self.code.push_str(&format!("{}}}\n\n", self.indent()));
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

    pub fn gen_if_else<F1, F2>(
        &mut self,
        condition: CValue,
        result_type: CType,
        mut then_block: F1,
        mut else_block: F2,
    ) -> CValue
    where
        F1: FnMut(&mut Self) -> CValue,
        F2: FnMut(&mut Self) -> CValue,
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

        let then_val = then_block(self);
        if result_type != CType::Void {
            self.code.push_str(&format!(
                "{}{} = {};\n",
                self.indent(),
                result_var_name,
                then_val.0
            ));
        }

        self.indent_level -= 1;
        self.code
            .push_str(&format!("{}}} else {{\n", self.indent()));
        self.indent_level += 1;

        let else_val = else_block(self);
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

    pub fn finish(&self) -> String {
        format!(
            "// Forward Declarations\n{}\n// Implementations\n{}",
            self.func_forward_decl, self.code
        )
    }
}
