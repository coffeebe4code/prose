use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

#[derive(Debug, Eq, Copy, Clone, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Op {
    NoOp = 0,
    RetVoid,
    RetVal,
    Load,
    Store,
    Phi,
    BitOp,
    IfOp,
    RouteMatch,
    CharOp,
    StringOp,
    ForOp,
    LoopOp,
    AllocOp,
    FreeOp,
    Struct,
    Array,
    SwitchOp,
    BreakOp,
    CSInvoke,
    CInvoke,
    Access,
    PtrOp,
    AddrOp,
    AsyncOp,
    AwaitOp,
    FfiOp,

    F64Const,
    F64Mul,
    F64Sub,
    F64Div,
    F64Add,
    F64Mod,

    F32Const,
    F32Mul,
    F32Sub,
    F32Div,
    F32Add,
    F32Mod,

    D64Const,
    D64Mul,
    D64Sub,
    D64Div,
    D64Add,
    D64Mod,

    D32Const,
    D32Mul,
    D32Sub,
    D32Div,
    D32Add,
    D32Mod,

    I64Const,
    I64Mul,
    I64Sub,
    I64Div,
    I64Add,
    I64Mod,
    I32Const,
    I32Mul,
    I32Sub,
    I32Div,
    I32Add,
    I32Mod,
    I16Const,
    I16Mul,
    I16Sub,
    I16Div,
    I16Add,
    I16Mod,
    I8Const,
    I8Mul,
    I8Sub,
    I8Div,
    I8Add,
    I8Mod,

    U64Const,
    U64Mul,
    U64Sub,
    U64Div,
    U64Add,
    U64Mod,
    U32Const,
    U32Mul,
    U32Sub,
    U32Div,
    U32Add,
    U32Mod,
    U16Const,
    U16Mul,
    U16Sub,
    U16Div,
    U16Add,
    U16Mod,
    U8Const,
    U8Mul,
    U8Sub,
    U8Div,
    U8Add,
    U8Mod,
    #[num_enum(default)]
    OpError,
}

impl Op {
    pub fn from32(bytes: u8) -> Op {
        return Op::try_from(bytes).unwrap();
    }
}

#[macro_export]
macro_rules! bin_op {
    ($val:ident) => {
        Op::$val
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Op::from32(99), Op::OpError);
        assert_eq!(Op::from32(0), Op::NoOp);
    }
}
