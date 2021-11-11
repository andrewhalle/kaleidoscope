#include <iostream>

#include <llvm/ADT/APFloat.h>
#include <llvm/IR/Value.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>

using namespace llvm;

extern "C" {
  Value* get_constant_fp(LLVMContext* context, double value) {
    return ConstantFP::get(*context, APFloat(value));
  }

  LLVMContext* get_context() {
    return new LLVMContext();
  }

  IRBuilder<>* get_builder(LLVMContext* context) {
    return new IRBuilder<>(*context);
  }

  Module* get_module(LLVMContext* context) {
    return new Module("my cool jit", *context);
  }

  void print_value(Value* value) {
    value->print(outs(), true);
  }

  Value* builder_create_f_add(IRBuilder<>* builder, Value* lhs, Value* rhs, const char* op) {
    return builder->CreateFAdd(lhs, rhs, op);
  }

  Value* builder_create_f_sub(IRBuilder<>* builder, Value* lhs, Value* rhs, const char* op) {
    return builder->CreateFSub(lhs, rhs, op);
  }

  Value* builder_create_f_mul(IRBuilder<>* builder, Value* lhs, Value* rhs, const char* op) {
    return builder->CreateFMul(lhs, rhs, op);
  }

  // TODO split this into two functions and call both from Rust
  Value* builder_create_f_cmp_lt(
      LLVMContext* context,
      IRBuilder<>* builder,
      Value* lhs,
      Value* rhs,
      char const* op
  ) {
    lhs = builder->CreateFCmpULT(lhs, rhs, "cmptmp");
    return builder->CreateUIToFP(lhs, Type::getDoubleTy(*context), op);
  }

  Function* module_get_function(Module* module, const char* name) {
    return module->getFunction(name);
  }

  Value* builder_create_call(
      IRBuilder<>* builder,
      Function* function,
      Value** arg_buf,
      size_t arg_size,
      const char* name
  ) {
    return builder->CreateCall(function, ArrayRef<Value*>(arg_buf, arg_size), name);
  }
}
