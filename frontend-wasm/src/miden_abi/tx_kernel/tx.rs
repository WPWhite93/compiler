use miden_hir::FunctionType;
use miden_hir_type::Type::*;

use crate::miden_abi::{FunctionTypeMap, ModuleFunctionTypeMap};

pub const CREATE_NOTE: &str = "create_note";

pub(crate) fn signatures() -> ModuleFunctionTypeMap {
    let mut m: ModuleFunctionTypeMap = Default::default();
    let mut note: FunctionTypeMap = Default::default();
    note.insert(
        CREATE_NOTE,
        FunctionType::new([Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt, Felt], [Felt]),
    );
    m.insert("miden:tx_kernel/tx", note);
    m
}
