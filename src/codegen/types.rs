// ©2025 - BestJasmine - BestMat - All rights reserved.

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    /* Basic C Types */
    Int,
    Char,
    Float,
    Double,
    Void,
    _Bool,
    _Complex,
    _Imaginary,

    /* Derived C Types */
    Pointer(Box<CType>),
    Array(Box<CType>),

    /* Qualifier C Types */
    Const(Box<CType>),
    Volatile(Box<CType>),
    Restrict(Box<CType>),
    Short(Box<CType>),
    Long(Box<CType>),
    Unsigned(Box<CType>),
    Signed(Box<CType>),

    /* Custom C Types */
    CustomTypedefType(String),
    CustomStructType(String),
    CustomEnumType(String),
    CustomUnionType(String),
}

impl Display for CType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CType::Int => write!(f, "int"),
            CType::Char => write!(f, "char"),
            CType::Float => write!(f, "float"),
            CType::Double => write!(f, "double"),
            CType::Void => write!(f, "void"),
            CType::_Bool => write!(f, "_Bool"),
            CType::_Complex => write!(f, "_Complex"),
            CType::_Imaginary => write!(f, "_Imaginary"),
            CType::Pointer(ctype) => write!(f, "{}*", *ctype),
            CType::Array(ctype) => write!(f, "{}[]", *ctype),
            CType::Const(ctype) => write!(f, "const {}", *ctype),
            CType::Volatile(ctype) => write!(f, "volatile {}", *ctype),
            CType::Restrict(ctype) => write!(f, "restrict {}", *ctype),
            CType::Short(ctype) => write!(f, "short {}", *ctype),
            CType::Long(ctype) => write!(f, "long {}", *ctype),
            CType::Unsigned(ctype) => write!(f, "unsigned {}", *ctype),
            CType::Signed(ctype) => write!(f, "signed {}", *ctype),
            CType::CustomTypedefType(ctype) => write!(f, "{}", ctype),
            CType::CustomStructType(ctype) => write!(f, "{}", ctype),
            CType::CustomEnumType(ctype) => write!(f, "{}", ctype),
            CType::CustomUnionType(ctype) => write!(f, "{}", ctype),
        }
    }
}