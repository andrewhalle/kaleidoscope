use std::collections::HashMap;
use std::ffi::CString;

use crate::lexer::Token;
use crate::parser::{
    BinaryExprAstNode, CallExprAstNode, ExprAstNode, NumberExprAstNode, VariableExprAstNode,
};

mod llvm {
    extern "C" {
        pub type Value;
        pub type LlvmContext;
        pub type IrBuilder;
        pub type Module;
        pub type Function;

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
        pub fn module_get_function(module: *mut Module, name: *const i8) -> *mut Function;
        pub fn builder_create_call(
            builder: *mut IrBuilder,
            function: *mut Function,
            arg_buf: *mut *mut Value,
            arg_size: usize,
            name: *const i8,
        ) -> *mut Value;
    }
}

struct Module {
    inner: *mut llvm::Module,
}

impl Module {
    fn get_function(&mut self, name: &str) -> *mut llvm::Function {
        unsafe {
            let s = CString::new(name).unwrap();
            llvm::module_get_function(self.inner, s.as_ptr())
        }
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

    fn create_call(
        &self,
        function: *mut llvm::Function,
        mut args: Vec<*mut llvm::Value>,
    ) -> *mut llvm::Value {
        unsafe {
            let s = CString::new("calltmp").unwrap();
            llvm::builder_create_call(
                self.inner,
                function,
                args.as_mut_ptr(),
                args.len(),
                s.as_ptr(),
            )
        }
    }
}

pub fn print_value(value: *mut llvm::Value) {
    unsafe { llvm::print_value(value) }
}

pub struct CodegenContext {
    context: *mut llvm::LlvmContext,
    builder: IrBuilder,
    module: Module,
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
                        _ => unreachable!(),
                    }
                }
                ExprAstNode::Call(CallExprAstNode { callee, args }) => {
                    let function = self.module.get_function(&callee);
                    let args: Vec<*mut llvm::Value> =
                        args.into_iter().map(|arg| self.codegen(arg)).collect();

                    self.builder.create_call(function, args)
                }
            }
        }
    }

    pub fn new() -> Self {
        unsafe {
            let context = llvm::get_context();
            let module = Module {
                inner: llvm::get_module(context),
            };
            let builder = IrBuilder {
                inner: llvm::get_builder(context),
            };

            CodegenContext {
                context,
                builder,
                module,
                named_values: HashMap::new(),
            }
        }
    }
}
