// impl<'a> Parser<'a> {
//     pub fn parse_struct_def(&mut self, is_pub: bool) -> Option<StructDef> {
//         self.consume_func(TokenKind::Struct)?;

//         let name = self.read_ident()?;

//         let mut generics = Vec::new();
//         if self.stream.is(TokenKind::Lt) {
//             self.advance();
//             while !self.stream.is(TokenKind::Gt) && !self.stream.is_at_end() {
//                 let gen_name = self.read_ident()?;
//                 generics.push(gen_name);

//                 if self.stream.is(TokenKind::Comma) {
//                     self.advance();
//                 } else {
//                     break;
//                 }
//             }
//             self.consume_func(TokenKind::Gt)?;
//         }

//         self.consume_func(TokenKind::OBrace)?;

//         let mut fields = Vec::new();

//         while !self.stream.is(TokenKind::CBrace) && !self.stream.is_at_end() {
//             while self.stream.is(TokenKind::Newline) {
//                 self.advance();
//             }
//             if self.stream.is(TokenKind::CBrace) {
//                 break;
//             }

//             let field_name = self.read_ident()?;

//             self.consume_func(TokenKind::Colon)?;
//             let field_type = self.parse_type()?;

//             fields.push((field_name, field_type));

//             if self.stream.is(TokenKind::Comma) {
//                 self.advance();
//             }

//             while self.stream.is(TokenKind::Newline) {
//                 self.advance();
//             }
//         }

//         self.consume_func(TokenKind::CBrace)?;

//         Some(StructDef {
//             is_pub,
//             name,
//             fields,
//             generics,
//         })
//     }

//     pub fn parse_static_def(&mut self, is_pub: bool) -> Option<StaticDef> {
//         self.consume_func(TokenKind::Static)?;

//         let name = self.read_ident()?;

//         self.consume_func(TokenKind::Colon)?;
//         let ty = self.parse_type()?;

//         let init_value = if self.stream.is(TokenKind::Eq) {
//             self.advance();
//             Some(self.parse_expr(0)?)
//         } else {
//             None
//         };

//         self.consume_func(TokenKind::Semi)?;

//         Some(StaticDef {
//             is_pub,
//             name,
//             ty,
//             init_value,
//         })
//     }
// }
