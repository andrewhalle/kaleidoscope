use std::collections::HashMap;
use std::ffi::CString;

use crate::lexer::Token;
use crate::parser::{BinaryExprAstNode, ExprAstNode, NumberExprAstNode, VariableExprAstNode};

mod llvm {
    extern "C" {
        pub type Value;
        pub type LlvmContext;
        pub type IrBuilder;
        pub type Module;

        pub fn get_context() -> *mut LlvmContext;
        pub fn get_builder(context: *mut LlvmContext) -> *mut IrBuilder;
        pub fn get_module(context: *mut LlvmContext) -> *mut Module;
        pub fn get_constant_fp(context: *mut LlvmContext, value: f64) -> *mut Value;
        pub fn print_value(value: *mut Value);
        pub fn builder_create_f_add(
            builder: *mut IrBuilder,
            lhs: *mut Value,
            rhs: *mut Value,
            op: *const i8,
        ) -> *mut Value;
        pub fn builder_create_f_sub(
            builder: *mut IrBuilder,
            lhs: *mut Value,
            rhs: *mut Value,
            op: *const i8,
        ) -> *mut Value;
        pub fn builder_create_f_mul(
            builder: *mut IrBuilder,
            lhs: *mut Value,
            rhs: *mut Value,
            op: *const i8,
        ) -> *mut Value;
        pub fn builder_create_f_cmp_lt(
            context: *mut LlvmContext,
            builder: *mut IrBuilder,
            lhs: *mut Value,
            rhs: *mut Value,
            op: *const i8,
        ) -> *mut Value;
    }
}

struct IrBuilder {
    inner: *mut llvm::IrBuilder,
}

impl IrBuilder {
    fn create_f_add(
        &self,
        lhs: *mut llvm::Value,
        rhs: *mut llvm::Value,
        op: &'static str,
    ) -> *mut llvm::Value {
        unsafe {
            let s = CString::new(op).unwrap();
            llvm::builder_create_f_add(self.inner, lhs, rhs, s.as_ptr())
        }
    }

    fn create_f_sub(
        &self,
        lhs: *mut llvm::Value,
        rhs: *mut llvm::Value,
        op: &'static str,
    ) -> *mut llvm::Value {
        unsafe {
            let s = CString::new(op).unwrap();
            llvm::builder_create_f_sub(self.inner, lhs, rhs, s.as_ptr())
        }
    }

    fn create_f_mul(
        &self,
        lhs: *mut llvm::Value,
        rhs: *mut llvm::Value,
        op: &'static str,
    ) -> *mut llvm::Value {
        unsafe {
            let s = CString::new(op).unwrap();
            llvm::builder_create_f_mul(self.inner, lhs, rhs, s.as_ptr())
        }
    }

    fn create_f_cmp_lt(
        &self,
        context: *mut llvm::LlvmContext,
        lhs: *mut llvm::Value,
        rhs: *mut llvm::Value,
        op: &'static str,
    ) -> *mut llvm::Value {
        unsafe {
            let s = CString::new(op).unwrap();
            llvm::builder_create_f_cmp_lt(context, self.inner, lhs, rhs, s.as_ptr())
        }
    }
}

pub fn print_value(value: *mut llvm::Value) {
    unsafe { llvm::print_value(value) }
}

pub struct CodegenContext {
    context: *mut llvm::LlvmContext,
    builder: IrBuilder,
    _module: *mut llvm::Module,
    named_values: HashMap<String, *mut llvm::Value>,
}

impl CodegenContext {
    pub fn codegen(&mut self, node: ExprAstNode) -> *mut llvm::Value {
        unsafe {
            match node {
                ExprAstNode::Number(NumberExprAstNode { value }) => {
                    llvm::get_constant_fp(self.context, value)
                }
                ExprAstNode::Variable(VariableExprAstNode { name }) => self.named_values[&name],
                ExprAstNode::Binary(BinaryExprAstNode { lhs, rhs, op }) => {
                    let lhs = self.codegen(*lhs);
                    let rhs = self.codegen(*rhs);
                    match op {
                        Token::Plus => self.builder.create_f_add(lhs, rhs, "addtmp"),
                        Token::Minus => self.builder.create_f_sub(lhs, rhs, "subtmp"),
                        Token::Star => self.builder.create_f_mul(lhs, rhs, "multmp"),
                        Token::LessThan => {
                            self.builder
                                .create_f_cmp_lt(self.context, lhs, rhs, "booltmp")
                        }
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
    }

    pub fn new() -> Self {
        unsafe {
            let context = llvm::get_context();
            let module = llvm::get_module(context);
            let builder = IrBuilder {
                inner: llvm::get_builder(context),
            };

            CodegenContext {
                context,
                builder,
                _module: module,
                named_values: HashMap::new(),
            }
        }
    }
}
