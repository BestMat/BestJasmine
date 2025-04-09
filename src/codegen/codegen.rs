// ©2025 - BestJasmine - BestMat - All rights reserved.

#[path="./types.rs"] pub mod types;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use crate::codegen::types::CType;
use std::sync::Once;
use std::mem::MaybeUninit;

// const C_LIBS: [CLibrary; 2] = [
//     CLibrary {
//         lib_name: "stdio.h".to_string(),
//         lib_link: true,
//     },
//     CLibrary {
//         lib_name: "stdlib.h".to_string(),
//         lib_link: true,
//     }
// ];


static mut CHAR_PTR_TYPE: MaybeUninit<CType> = MaybeUninit::uninit();
static INIT_CHAR_PTR: Once = Once::new();

const INDENT: &str = "    ";

#[derive(Debug, Clone)]
pub struct Codegen {
    pub c_file: CFile,                  // C File
    pub c_program: CProgramNode,        // C Program
}
#[derive(Debug, Clone)]
pub struct CFile {
    pub file_name: String,              // Name of the C File
    pub file_path: String,              // File path of the C File
    pub headers: Vec<CLibrary>,         // C Header Libraries
}

impl CFile {
    pub fn get_file_path(&self) -> String {
        format!(
            "{}/{}",
            self.file_path,
            self.file_name
        )
    }
}

/* TODO: Implement `Display` for CLibrary */
#[derive(Debug, Clone)]
pub struct CLibrary {
    pub lib_name: String,               // Name of the C Library
    pub lib_link: bool,                 // Is the C Library Linked
}

/* TODO: Implement `Display` for CProgramNode and its children */
#[derive(Debug, Clone)]
pub enum CProgramNode {
    CStatement(CStatement),             // C Statements
    CExpression(CExpression),           // C Expressions
    CProgram(Vec<CProgramNode>),        // C Program
}

#[derive(Debug, Clone)]
pub enum CStatement {
    /* Classic Statements */
    VariableStatement(CVariableStatement),// int x = 21;
    AssignmentStatement(CAssignmentStatement),// x = 27;
    ReturnStatement(CReturnStatement),// return x;
    FunctionCallStatement(CFunctionCallStatement),// printf("Hello, world!\n");
    BreakStatement,                     // break;

    /* Block Statements */
    BlockStatement(CBlockStatement),    // {}
    IfStatement,                        // if (x == 27) {}
    WhileStatement,                     // while (1) {}
    ForStatement,                       // for (int i = 0; i < 27; i++) {}
    DoWhileStatement,                   // do {} while (x == 21)
    FunctionStatement(CFunctionStatement),// int main() {}

    /* Header Statements */
    IncludeStatement(CIncludeStatement),// #include <stdio.h>
    DefineStatement,                    // #define PI 3.14
    PragmaStatement,                    // #pragma
    TypedefStatement,                   // typedef int JavaScript_Number;
}

#[derive(Debug, Clone)]
pub struct CVariableStatement {
    pub var_type: CType,
    pub var_name: String,
    pub var_value: CExpression,
}

#[derive(Debug, Clone)]
pub struct CAssignmentStatement {
    pub var_name: String,
    pub var_value: CExpression,
}

#[derive(Debug, Clone)]
pub struct CReturnStatement {
    pub value: CExpression,
}

#[derive(Debug, Clone)]
pub enum CExpression {
    /* Basic Expressions */
    NumberExpression(CNumberExpression),// 21
    StringExpression(CStringExpression),// "Hello, world!\n"
    IdentifierExpression(CIdentifierExpression),// x
    DecimalExpression(CDecimalExpression),// 3.14159
    ArrayExpression(CArrayExpression),// [1, 2, 3]
    CharExpression(CCharExpression),// "x"
    BooleanExpression(CBooleanExpression),// true/false (typeof _Bool - C99+)
    HexExpression(CNumberExpression),// 0xCAFEBABE
    OctalExpression(CNumberExpression),// 0123

    /* Complex Expressions */
    TernaryExpression,                  // x == 21 ? {} : {}
    CastExpression,                     // (int) "123"
    StructExpression,                   // { fieldName: fieldValue }

    /* Arithmetic Expressions */
    BinaryExpression,                   // 1 + 1
    LogicalExpression,                  // 1 == 1

    /* Access Expressions */
    MemberExpression,                   // a->b, a.b
    IndexExpression,                    // a[b]
    PointerExpression,                  // *a, &b

    /* Function Call Expression */
    FunctionCallExpression(CFunctionCallStatement),// printf("Hello, world!\n");
}

fn get_char_ptr_type() -> &'static CType {
    unsafe {
        INIT_CHAR_PTR.call_once(|| {
            let ptr = CType::Pointer(Box::new(CType::Char));
            CHAR_PTR_TYPE.write(ptr);
        });
        CHAR_PTR_TYPE.assume_init_ref()
    }
}

impl CExpression {
    pub fn get_type(&self) -> Option<&CType> {
        match &self {
            CExpression::NumberExpression(_) => Some(&CType::Int),
            CExpression::StringExpression(_) => Some(get_char_ptr_type()),
            CExpression::IdentifierExpression(_) => None,
            CExpression::DecimalExpression(decimal) => Some(&decimal.ctype),
            CExpression::ArrayExpression(array) => Some(&array.ctype),
            CExpression::CharExpression(_) => Some(&CType::Char),
            CExpression::BooleanExpression(_) => Some(&CType::_Bool),
            CExpression::HexExpression(_) => Some(&CType::Int),
            CExpression::OctalExpression(_) => Some(&CType::Int),
            CExpression::TernaryExpression => None,
            CExpression::CastExpression => None, // TODO
            CExpression::StructExpression => None, // TODO
            CExpression::BinaryExpression => None, // TODO
            CExpression::LogicalExpression => None, // TODO
            CExpression::MemberExpression => None, // TODO
            CExpression::IndexExpression => None, // TODO
            CExpression::PointerExpression => None, // TODO
            CExpression::FunctionCallExpression(_) => None, // TODO
        }
    }
}

#[derive(Debug, Clone)]
pub struct CNumberExpression {
    pub cvalue: i32,
}

impl CNumberExpression {
    pub fn new(number: i32) -> Self {
        Self { cvalue: number }
    }
}

#[derive(Debug, Clone)]
pub struct CStringExpression {
    pub cvalue: String,
    pub length: usize,
}

impl CStringExpression {
    pub fn new(string: &str) -> Self {
        Self { cvalue: string.to_string(), length: string.len() }
    }
}

#[derive(Debug, Clone)]
pub struct CIdentifierExpression {
    pub cvalue: String,
}

impl CIdentifierExpression {
    pub fn new(identifier: &str) -> Self {
        Self { cvalue: identifier.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct CDecimalExpression {
    pub cvalue: i32,
    pub ctype: CType,                   // Float or Double
}

impl CDecimalExpression {
    pub fn new(decimal: i32, ctype: CType) -> Self {
        Self { cvalue: decimal, ctype }
    }
}

#[derive(Debug, Clone)]
pub struct CArrayExpression {
    pub cvalue: Vec<CExpression>,
    pub ctype: CType,
    pub length: usize,
    pub capacity: usize,
}

impl CArrayExpression {
    pub fn new(value: Vec<CExpression>, ctype: CType) -> Self {
        // To avoid cloning the array as it doubles the memory for the array stored in the heap:
        let len = &value.len();

        Self {
            cvalue: value,
            ctype,
            length: *len,
            capacity: *len + 1,
        }
    }

    pub fn push(&mut self, item: CExpression) {
        match item.get_type() {
            Some(ctype) => {
                if (*ctype == self.ctype) {
                    self.cvalue.push(item);
                    self.length += 1;
                    self.capacity += 1;
                } else {
                    // TODO: Use thiserror
                    panic!("Type error: expected type {:#?}, but found {}", self.ctype, *ctype);
                }
            }

            None => {}
        }
    }

    // TODO: Add all other array methods.
}

#[derive(Debug, Clone)]
pub struct CCharExpression {
    pub cvalue: char,
}

impl CCharExpression {
    pub fn new(char: char) -> Self {
        Self { cvalue: char }
    }
}

#[derive(Debug, Clone)]
pub struct CBooleanExpression {
    pub cvalue: char,
}

#[derive(Debug, Clone)]
pub struct CFunctionCallStatement {
    pub function: String,
    pub args: Vec<CExpression>,
}

#[derive(Debug, Clone)]
pub struct CIncludeStatement {
    pub library: CLibrary,
}

#[derive(Debug, Clone)]
pub struct CBlockStatement {
    pub block: Vec<CStatement>,
}

#[derive(Debug, Clone)]
pub struct CFunctionStatement {
    pub function_type: CType,
    pub function_name: String,
    pub function_args: Vec<CFunctionArg>,
    pub function_block: CBlockStatement,
}
#[derive(Debug, Clone)]
pub struct CFunctionArg {
    pub r#type: CType,
    pub name: String,
}

impl CFile {
    pub fn new(file_name: String, file_path: String, headers: Vec<CLibrary>) -> Self {
        Self { file_name, file_path, headers }
    }
}

impl Display for CFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, 
                 "File Name: {}\nFile Path: {}\nHeaders: {:#?}",
                 self.file_name,
                 self.file_path,
                 self.headers
        )
    }
}

impl CLibrary {
    pub fn stdio() -> Self {
        Self {
            lib_name: "stdio.h".to_string(),
            lib_link: true
        }
    }

    pub fn stdlib() -> Self {
        Self {
            lib_name: "stdlib.h".to_string(),
            lib_link: true
        }
    }
}

impl Display for CFunctionArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.r#type, self.name)
    }
}

impl Codegen {
    /* Creates a new instance of BestJasmine Codegen */
    pub fn new() -> Self {
        let c_program: CProgramNode = CProgramNode::CProgram(Vec::new());
        
        Self {
            c_file: CFile::new("main.c".to_string(), ".".to_string(), Vec::new()),
            c_program,
        }
    }

    /* Appends an include statement to the C Program (#include) */
    pub fn add_include_statement(&mut self, library: CLibrary) {
        match &mut self.c_program {
            CProgramNode::CProgram(ref mut program) => {
                program.push(
                    CProgramNode::CStatement(
                        CStatement::IncludeStatement(CIncludeStatement {
                            library: library.clone()
                        })
                    )
                );
            }
            _ => {}
        }

        self.c_file.headers.push(library);
    }

    /* Appends a function to the C Program */
    pub fn add_function_statement(&mut self, function: CFunctionStatement) {
        match &mut self.c_program {
            CProgramNode::CProgram(ref mut program) => {
                program.push(
                    CProgramNode::CStatement(
                        CStatement::FunctionStatement(function)
                    )
                );
            }
            _ => {}
        }
    }
    
    pub fn generate_code(&self) -> String {
        let mut code = String::new();

        match &self.c_program {
            CProgramNode::CProgram(program) => {
                for node in program {
                    match &node {
                        CProgramNode::CStatement(statement) => {
                            let statement = self.generate_statement(&statement);
                            code.push_str(statement.as_str());
                        }

                        CProgramNode::CExpression(expression) => {
                            let expression = self.generate_expression(&expression);
                            code.push_str(expression.as_str());
                        }

                        _ => {}
                    }
                }
            }

            _ => {}
        }

        /* Save to file */

        match File::create(
            self.c_file.get_file_path()
        ) {
            // TODO: Use thiserror for error handling
            Ok(mut file) => {
                match file.write_all(format!("{}", code).as_bytes()) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Failed to write to file: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to create or overwrite file: {}", e),
        }

        code
    }

    pub fn generate_statement(&self, node: &CStatement) -> String {
        match node {
            CStatement::VariableStatement(stmt) => {
                self.generate_c_variable_statement(stmt)
            }

            CStatement::AssignmentStatement(stmt) => {
                self.generate_c_assignment_statement(stmt)
            }

            CStatement::ReturnStatement(stmt) => {
                self.generate_c_return_statement(stmt)
            }

            CStatement::FunctionCallStatement(expr) => {
                self.generate_c_function_call_statement(expr)
            }

            CStatement::IncludeStatement(stmt) => {
                self.generate_include_statement(stmt)
            }

            CStatement::FunctionStatement(stmt) => {
                self.generate_function_statement(stmt)
            }

            CStatement::BlockStatement(stmt) => {
                self.generate_block_statement(stmt)
            }

            _ => { todo!() }
        }
    }

    pub fn generate_expression(&self, node: &CExpression) -> String {
        match &node {
            CExpression::NumberExpression(expr) => {
                self.generate_c_number_expression(expr)
            }

            CExpression::StringExpression(expr) => {
                self.generate_c_string_expression(expr)
            }

            CExpression::HexExpression(expr) => {
                self.generate_c_number_expression(expr)
            }

            CExpression::OctalExpression(expr) => {
                self.generate_c_number_expression(expr)
            }

            CExpression::FunctionCallExpression(expr) => {
                self.generate_c_function_call_statement(expr)
            }

            _ => { todo!() }
        }
    }

    pub fn generate_c_number_expression(&self, expr: &CNumberExpression) -> String {
        format!("{}", expr.cvalue)
    }

    pub fn generate_c_string_expression(&self, expr: &CStringExpression) -> String {
        format!("\"{}\"", expr.cvalue)
    }

    pub fn generate_c_variable_statement(&self, node: &CVariableStatement) -> String {
        match node.var_value.get_type() {
            Some(node_type) => {
                if &node.var_type != node_type {
                    // TODO: use thiserror for error
                    panic!("The type {} does not match with the type {}", node.var_type, node_type);
                }
            }

            None => {}
        }

        format!("{INDENT}{} {} = {};\n", node.var_type, node.var_name, self.generate_expression(&node.var_value))
    }

    pub fn generate_c_assignment_statement(&self, node: &CAssignmentStatement) -> String {
        format!("{INDENT}{} = {};\n", node.var_name, self.generate_expression(&node.var_value))
    }

    pub fn generate_c_return_statement(&self, node: &CReturnStatement) -> String {
        format!("{INDENT}return {};", self.generate_expression(&node.value))
    }

    pub fn generate_c_function_call_statement(&self, expr: &CFunctionCallStatement) -> String {
        format!("{INDENT}{}({});\n", expr.function, expr.args.iter().map(|x| self.generate_expression(x)).collect::<Vec<_>>().join(", "))
    }

    pub fn generate_include_statement(&self, node: &CIncludeStatement) -> String {
        let library = &node.library;

        if library.lib_link {
            format!("#include <{}>", library.lib_name)
        } else {
            format!("#include \"{}\"", library.lib_name)
        }
    }

    pub fn generate_block_statement(&self, block_node: &CBlockStatement) -> String {
        let mut code = String::new();

        for node in &block_node.block {
            code.push_str(self.generate_statement(node).as_str());
        }

        code
    }

    pub fn generate_function_statement(&self, node: &CFunctionStatement) -> String {
        format!(r#"
{} {}({}) {{
{}
}}"#, node.function_type, node.function_name, node.function_args
            .iter().map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(", "), self.generate_block_statement(&node.function_block))
    }

    pub fn get_c_file(&self) -> CFile {
        self.c_file.clone()
    }
}