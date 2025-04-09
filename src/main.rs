// ©2025 - BestJasmine - BestMat - All rights reserved.

use crate::codegen::{CAssignmentStatement, CBlockStatement, CExpression, CFunctionCallStatement, CFunctionStatement, CLibrary, CNumberExpression, CReturnStatement, CStatement, CStringExpression, CVariableStatement, Codegen};
use crate::codegen::types::CType;
use crate::compiler::{Compilers, JasmineBuilder};

#[path="./codegen/codegen.rs"] mod codegen;
#[path="./compilers/compiler.rs"] mod compiler;

fn main() {
    let mut codegen = Codegen::new();

    codegen.add_include_statement(CLibrary::stdio());

    codegen.add_function_statement(CFunctionStatement {
        function_type: CType::Int,
        function_name: "main".to_string(),
        function_block: CBlockStatement {
            block: vec![CStatement::VariableStatement(CVariableStatement {
                var_type: CType::Int,
                var_name: "x".to_string(),
                var_value: CExpression::NumberExpression(CNumberExpression::new(21)),
            }), CStatement::AssignmentStatement(CAssignmentStatement {
                var_name: "x".to_string(),
                var_value: CExpression::NumberExpression(CNumberExpression::new(27)),
            }), CStatement::FunctionCallStatement(CFunctionCallStatement {
                function: "printf".to_string(),
                args: vec![CExpression::StringExpression(CStringExpression::new("Hello, world!\\n"))],
            }), CStatement::ReturnStatement(CReturnStatement {
                value: CExpression::NumberExpression(CNumberExpression::new(0)),
            })],
        },
        function_args: Vec::new(),
    });

    codegen.generate_code();

    let mut builder = JasmineBuilder::new(Compilers::Gcc);
    builder.add_file(codegen.get_c_file());
    builder.build("main");
}