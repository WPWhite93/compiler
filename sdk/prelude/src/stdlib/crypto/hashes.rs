use crate::Felt;

#[link(wasm_import_module = "miden:stdlib/crypto_hashes")]
extern "C" {
    #[link_name = "blake3_hash_1to1<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_blake3_hash_1to1(
        e1: Felt,
        e2: Felt,
        e3: Felt,
        e4: Felt,
        e5: Felt,
        e6: Felt,
        e7: Felt,
        e8: Felt,
        ptr: i32,
    );

    #[link_name = "blake3_hash_2to1<0x0000000000000000000000000000000000000000000000000000000000000000>"]
    fn extern_blake3_hash_2to1(
        e1: Felt,
        e2: Felt,
        e3: Felt,
        e4: Felt,
        e5: Felt,
        e6: Felt,
        e7: Felt,
        e8: Felt,
        e9: Felt,
        e10: Felt,
        e11: Felt,
        e12: Felt,
        e13: Felt,
        e14: Felt,
        e15: Felt,
        e16: Felt,
        ptr: i32,
    );
}

/// Hashes a 32-byte input to a 32-byte output using the BLAKE3 hash function.
#[inline(always)]
pub fn blake3_hash_1to1(input: [u8; 32]) -> [u8; 32] {
    unsafe {
        struct RetArea([Felt; 8]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr = ret_area.as_mut_ptr() as i32;
        let mut felts_input = [Felt::from_u64_unchecked(0); 8];
        for i in 0..8 {
            felts_input[i] = Felt::from_u64_unchecked(u32::from_le_bytes(
                input[i * 4..(i + 1) * 4].try_into().unwrap(),
            ) as u64);
        }
        extern_blake3_hash_1to1(
            felts_input[0],
            felts_input[1],
            felts_input[2],
            felts_input[3],
            felts_input[4],
            felts_input[5],
            felts_input[6],
            felts_input[7],
            ptr,
        );
        // make an array from the ptr (points fo 8 Felts)
        let felts_out = ret_area.assume_init().0;
        let mut result = [0u8; 32];
        for i in 0..8 {
            let bytes = felts_out[i].as_u64().to_le_bytes();
            result[i * 4..(i + 1) * 4].copy_from_slice(&bytes[0..4]);
        }
        result
    }
}

#[inline(always)]
pub fn blake3_hash_2to1(input: [u8; 64]) -> [u8; 32] {
    unsafe {
        struct RetArea([Felt; 16]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr = ret_area.as_mut_ptr() as i32;
        let mut felts_input = [Felt::from_u64_unchecked(0); 16];
        for i in 0..16 {
            felts_input[i] = Felt::from_u64_unchecked(u32::from_le_bytes(
                input[i * 4..(i + 1) * 4].try_into().unwrap(),
            ) as u64);
        }
        extern_blake3_hash_2to1(
            felts_input[0],
            felts_input[1],
            felts_input[2],
            felts_input[3],
            felts_input[4],
            felts_input[5],
            felts_input[6],
            felts_input[7],
            felts_input[8],
            felts_input[9],
            felts_input[10],
            felts_input[11],
            felts_input[12],
            felts_input[13],
            felts_input[14],
            felts_input[15],
            ptr,
        );
        // make an array from the ptr (points fo 8 Felts)
        let felts_out = ret_area.assume_init().0;
        let mut result = [0u8; 32];
        for i in 0..8 {
            let bytes = felts_out[i].as_u64().to_le_bytes();
            result[i * 4..(i + 1) * 4].copy_from_slice(&bytes[0..4]);
        }
        result
    }
}
