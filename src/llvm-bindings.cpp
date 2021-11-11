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

  Function* module_create_function(
      LLVMContext* context,
      Module* mod,
      const char* name,
      const char** args,
      size_t arg_size
  ) {
    std::vector<Type*> Doubles(arg_size, Type::getDoubleTy(*context));
    FunctionType* FT = FunctionType::get(Type::getDoubleTy(*context), Doubles, false);
    Function* F = Function::Create(FT, Function::ExternalLinkage, name, mod);

    // Set names for all arguments.
    unsigned Idx = 0;
    for (auto &Arg : F->args())
      Arg.setName(args[Idx++]);

    return F;
  }

  void create_function_body(LLVMContext* context, Function* function, IRBuilder<>* builder) {
    BasicBlock* BB = BasicBlock::Create(*context, "entry", function);
    builder->SetInsertPoint(BB);
  }

  // I'm going to leak ALL THE MEMORY
  Value** get_function_args(Function* function) {
    std::vector<Value*>* args = new std::vector<Value*>;
    for (auto& arg : function->args()) {
      args->push_back(&arg);
    }

    return args->data();
  }

  void builder_create_ret(IRBuilder<>* builder, Value* value) {
    builder->CreateRet(value);
  }

  void print_function(Function* function) {
    function->print(outs(), nullptr, false, true);
  }
}
