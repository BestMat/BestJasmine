// ©2025 - BestJasmine - BestMat - All rights reserved.

use std::env::consts::OS;
use std::process::Command;
use crate::codegen::{CFile, CLibrary};

// TODO: Add "any" compiler (any custom C Compiler)
pub enum Compilers {
    Gcc,            // GNU Compiler Collection
    Clang,          // Clang Compiler (LLVM)
    Gpp,            // GNU C++ Compiler (UNSUPPORTED FOR NOW)
}

pub enum OperatingSystem {
    BestMat,        // The BestMat Operating System
    MacOS,          // MacOS
    Linux,          // Linux (WSL Included)
    Windows,        // Windows
    Unknown,        // Unknown OS
}

pub struct JasmineBuilder {
    pub files: Vec<CFile>,
    pub headers: Vec<CLibrary>,
    pub compiler: Compilers,
    pub os: OperatingSystem,
}

impl JasmineBuilder {
    pub fn new(compiler: Compilers) -> Self {
        let mut os;

        match OS {
            "macos" => {
                os = OperatingSystem::MacOS;
            }

            "linux" => {
                os = OperatingSystem::Linux;
            }

            "windows" => {
                os = OperatingSystem::Windows;
            }

            _ => {
                os = OperatingSystem::Unknown;
            }
        }

        Self {
            files: Vec::new(),
            headers: Vec::new(),
            compiler,
            os,
        }
    }

    pub fn add_file(&mut self, file: CFile) {
        for library in file.headers.clone() {
            // Check if the header file/library is not a LibC file:

            if library.lib_link == false {
                self.headers.push(library);
            }
        }

        self.files.push(file);
    }

    pub fn build(&self, executable_name: &str) {
        let mut argfiles = String::new();
        let mut file_no = 0;

        for file in &self.files {
            file_no += 1;

            if file_no == self.files.len() {
                argfiles.push_str(format!("{}", file.get_file_path()).as_str());
            } else {
                argfiles.push_str(format!("{},", file.get_file_path()).as_str());
            }
        }

        match self.compiler {
            Compilers::Gcc => {
                let output = Command::new("gcc")
                    .args(&[
                        argfiles.as_str(),      // Files
                        "-o",                   // Converts to executable
                        executable_name,        // Executable name
                        "-O3",                  // Highest optimization
                        // "-Wall",             // All warnings
                        // "-Wextra",           // Extra warnings
                        // "-Werror",           // Treat warnings as errors
                        "-march=native",        // Optimize for the current CPU
                        "-flto",                // Link Time Optimization
                    ]).output().expect("Failed to run GCC Compiler.");
            }

            Compilers::Clang => {
                let output = Command::new("clang")
                    .args(&[
                        argfiles.as_str(),      // Files
                        "-o",                   // Converts to executable
                        executable_name,        // Executable name
                        "-O3",                  // Highest optimization
                        // "-Wall",             // All warnings
                        // "-Wextra",           // Extra warnings
                        // "-Werror",           // Treat warnings as errors
                        "-march=native",        // Optimize for the current CPU
                        "-flto",                // Link Time Optimization
                    ]).output().expect("Failed to run GCC Compiler.");
            }

            Compilers::Gpp => {
                todo!("Not implemented as of BestJasmine Version Alpha.");
            }
        }
    }
}