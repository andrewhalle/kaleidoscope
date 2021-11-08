use std::collections::HashMap;

use crate::parser::{ExprAstNode, NumberExprAstNode};

extern "C" {
    pub type Value;
    type LlvmContext;
    type IrBuilder;
    type Module;

    fn get_context() -> *mut LlvmContext;
    fn get_builder(context: *mut LlvmContext) -> *mut IrBuilder;
    fn get_module(context: *mut LlvmContext) -> *mut Module;
    fn get_constant_fp(context: *mut LlvmContext, value: f64) -> *mut Value;
    fn print_value(value: *mut Value);
}

pub fn print_value_rust(value: *mut Value) {
    unsafe { print_value(value) }
}

pub struct CodegenContext {
    context: *mut LlvmContext,
    _builder: *mut IrBuilder,
    _module: *mut Module,
    _named_values: HashMap<String, *mut Value>,
}

impl CodegenContext {
    pub fn codegen(&mut self, node: ExprAstNode) -> *mut Value {
        unsafe {
            match node {
                ExprAstNode::Number(NumberExprAstNode { value }) => {
                    get_constant_fp(self.context, value)
                }
                _ => todo!(),
            }
        }
    }

    pub fn new() -> Self {
        unsafe {
            let context = get_context();
            let module = get_module(context);
            let builder = get_builder(context);

            CodegenContext {
                context,
                _builder: builder,
                _module: module,
                _named_values: HashMap::new(),
            }
        }
    }
}
