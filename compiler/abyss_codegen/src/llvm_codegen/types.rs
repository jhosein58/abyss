use super::AbyssCompiler;
use abyss_ir::ir::IrType;
use inkwell::AddressSpace;
use inkwell::types::{BasicType, BasicTypeEnum};

impl<'ctx> AbyssCompiler<'ctx> {
    pub fn compile_type(&self, ty: &IrType) -> BasicTypeEnum<'ctx> {
        match ty {
            IrType::I8 | IrType::U8 => self.context.i8_type().into(),
            IrType::I16 | IrType::U16 => self.context.i16_type().into(),
            IrType::I32 | IrType::U32 => self.context.i32_type().into(),
            IrType::I64 | IrType::U64 => self.context.i64_type().into(),

            IrType::F32 => self.context.f32_type().into(),
            IrType::F64 => self.context.f64_type().into(),

            IrType::I1 | IrType::Bool => self.context.bool_type().into(),

            IrType::Unit => self.context.struct_type(&[], false).into(),
            IrType::Ptr(_) => self.context.ptr_type(AddressSpace::default()).into(),
            IrType::Array(inner, size) => self.compile_type(inner).array_type(*size as u32).into(),
            IrType::Struct(fields) => {
                let field_types: Vec<BasicTypeEnum> =
                    fields.iter().map(|f| self.compile_type(f)).collect();
                self.context.struct_type(&field_types, false).into()
            }
        }
    }
}
