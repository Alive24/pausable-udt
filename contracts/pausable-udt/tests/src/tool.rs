use std::ffi::CStr;
use std::ffi::CString;

use ckb_ssri_std::public_module_traits::udt::UDTPausableData;
use ckb_std::ckb_types::packed::Bytes as PackedBytes;
use ckb_std::ckb_types::packed::BytesBuilder as PackedBytesBuilder;
use ckb_std::ckb_types::packed::BytesVec;
use ckb_std::ckb_types::packed::BytesVecBuilder;
use ckb_std::ckb_types::packed::Script;
use ckb_std::ckb_types::packed::ScriptOpt;
use ckb_std::ckb_types::packed::ScriptOptBuilder;
use ckb_std::ckb_types::packed::Uint64;
use ckb_std::{
    ckb_types::packed::{CellOutputBuilder, Uint128},
    high_level::{decode_hex, encode_hex},
};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, Pack, Unpack};
use serde_molecule::to_vec;

// #[test]
// fn decode_hex_tool() {
//     let hex = "";
//     let bytes = decode_hex(hex);
//     println!("Decoded Bytes: {:?}", bytes);
// }

// #[test]
// fn encode_hex_tool() {
//     let data = "";
//     let hex = encode_hex(data.as_bytes());
//     println!("Encoded Hex: {:?}", hex);
// }

#[test]
fn generic_test() {
    let cstring = CString::new("0x0000").unwrap();
    let cstr = cstring.as_c_str();
    println!("CStr: {:?}", cstr);
    let decoded_hex = decode_hex(&cstr[2..]).unwrap();
        
    println!("Decoded Hex: {:?}", decoded_hex);
    // let offset = u64::from_le_bytes(decode_hex(CString::new("0x000000").unwrap().as_c_str()).unwrap().try_into().unwrap_or_default());
    // println!("Offset: {:?}", offset);
    return;
}
