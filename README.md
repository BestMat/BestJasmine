# BestJasmine - BestMat

🌼 BestJasmine is a **C Codegen Library** written in Rust. This repository consists of the main codegen library as well as the builder for building and compiling the C Code generated.

BestJasmine consists of mainly 2 parts:

- **Code Generator:** The code generator provides high-level functions to generate **memory-safe<sup>*</sup>** code.
- **Builder:** The builder builds all the C File generated with the help of **any** desired compiler. Any compiler can be used. I have implemented as of now **GCC** and **Clang** as compilers for building the C Files into a compiled executable. Any custom C Compiler can also be used.

**Current Version**: BestJasmine Alpha

\* **Memory Safety:** When I say that the program generated is memory-safe, the code tries to avoid C Memory Leaks and Unfreed Memory at all costs. Eg:

```c
#include <stdlib.h>

int main() {
    int* x = malloc(sizeof(int));
    *x = 21;
    
    free(x); // This will be done by BestJasmine
    return 0;
}
```

## Example of Codegen (Beta):

This is an example of printing "Hello World" to the console with BestJasmine. This is the goal (I haven't implemented this yet, this will be Beta).

```rust
let jasmine = JasmineCodegen::new();
let file = jasmine.create_module("main");
file.include("stdio.h");

let main = main.add_function(JasmineStandardType::Int32, "main");

main.call_function("printf", "Hello, world!\\n");
main.return_value(0);

let mut builder = JasmineBuilder::new(Compilers::Gcc);
builder.build(vec![main]);

```

## Example of Codegen (Current version - Alpha):

Currently I have not implemented the code example above. As of now I can do the above like (not memory-safe, more like a library that provides just C Functions):

```rust
let mut codegen = Codegen::new();

codegen.add_include_statement(CLibrary::stdio());

codegen.add_function_statement(CFunctionStatement {
    function_type: CType::Int,
    function_name: "main".to_string(),
    function_block: CBlockStatement {
        block: vec![
            CStatement::FunctionCallStatement(CFunctionCallStatement {
                function: "printf".to_string(),
                args: vec![CExpression::StringExpression(CStringExpression::new("Hello, world!\\n"))],
            }), CStatement::ReturnStatement(CReturnStatement {
                value: CExpression::NumberExpression(CNumberExpression::new(0)),
            })
        ],
    },
    function_args: Vec::new(),
});

let mut builder = JasmineBuilder::new(Compilers::Gcc);
builder.add_file(codegen.get_c_file());
builder.build("main");
```