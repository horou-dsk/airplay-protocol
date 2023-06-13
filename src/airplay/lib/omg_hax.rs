use super::{
    get_i32_le, modified_md5,
    omg_hax_const::{
        DEFAULT_SAP, INDEX_MANGLE, INITIAL_SESSION_KEY, MESSAGE_IV, MESSAGE_KEY, STATIC_SOURCE_1,
        STATIC_SOURCE_2, TABLE_S1, TABLE_S10, TABLE_S2, TABLE_S3, TABLE_S4, TABLE_S5, TABLE_S6,
        TABLE_S7, TABLE_S8, TABLE_S9, T_KEY, X_KEY, Z_KEY,
    },
    sap_hash, write_i32_le,
};

pub(super) fn decrypt_aes_key(message3: &[u8], cipher_text: &[u8], key_out: &mut [u8]) {
    let chunk1 = &cipher_text[16..cipher_text.len()];
    let chunk2 = &cipher_text[56..cipher_text.len()];
    let mut block_in = [0; 16];
    let mut sap_key = [0; 16];
    let mut key_schedule = [[0; 4]; 11];
    generate_session_key(&DEFAULT_SAP, message3, &mut sap_key);
    {
        let mut key_schedule_mut: Vec<&mut [i32]> =
            key_schedule.iter_mut().map(|v| &mut v[..]).collect();
        generate_key_schedule(&sap_key, &mut key_schedule_mut);
    }
    z_xor(chunk2, &mut block_in, 1);
    let key_schedule_ref: Vec<&[i32]> = key_schedule.iter().map(|v| &v[..]).collect();
    cycle(&mut block_in, &key_schedule_ref);
    key_out[..16]
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = block_in[i] ^ chunk1[i]);
    x_xor_f(1, |(index, key)| {
        let v = key_out[index] ^ key;
        key_out[index] = v;
    });
    z_xor_f(1, |(index, key)| {
        let v = key_out[index] ^ key;
        key_out[index] = v;
    });
}

fn decrypt_message(message_in: &[u8], decrypted_message: &mut [u8]) {
    let mut buffer = [0; 16];
    let mut tmp;
    let mode = message_in[12] as usize;
    for i in 0..8 {
        for j in 0..16 {
            if mode == 3 {
                buffer[j] = message_in[(0x80 - 0x10 * i) + j];
            } else if mode == 2 || mode == 1 || mode == 0 {
                buffer[j] = message_in[(0x10 * (i + 1)) + j];
            }
        }

        for j in 0..9 {
            let base = 0x80 - 0x10 * j;

            buffer[0x0] = message_table_index(base)[buffer[0x0] as usize] ^ MESSAGE_KEY[mode][base];
            buffer[0x4] = message_table_index(base + 0x4)[buffer[0x4] as usize]
                ^ MESSAGE_KEY[mode][base + 0x4];
            buffer[0x8] = message_table_index(base + 0x8)[buffer[0x8] as usize]
                ^ MESSAGE_KEY[mode][base + 0x8];
            buffer[0xc] = message_table_index(base + 0xc)[buffer[0xc] as usize]
                ^ MESSAGE_KEY[mode][base + 0xc];

            tmp = buffer[0x0d];
            buffer[0xd] = message_table_index(base + 0xd)[buffer[0x9] as usize]
                ^ MESSAGE_KEY[mode][base + 0xd];
            buffer[0x9] = message_table_index(base + 0x9)[buffer[0x5] as usize]
                ^ MESSAGE_KEY[mode][base + 0x9];
            buffer[0x5] = message_table_index(base + 0x5)[buffer[0x1] as usize]
                ^ MESSAGE_KEY[mode][base + 0x5];
            buffer[0x1] =
                message_table_index(base + 0x1)[tmp as usize] ^ MESSAGE_KEY[mode][base + 0x1];

            tmp = buffer[0x02];
            buffer[0x2] = message_table_index(base + 0x2)[buffer[0xa] as usize]
                ^ MESSAGE_KEY[mode][base + 0x2];
            buffer[0xa] =
                message_table_index(base + 0xa)[tmp as usize] ^ MESSAGE_KEY[mode][base + 0xa];
            tmp = buffer[0x06];
            buffer[0x6] = message_table_index(base + 0x6)[buffer[0xe] as usize]
                ^ MESSAGE_KEY[mode][base + 0x6];
            buffer[0xe] =
                message_table_index(base + 0xe)[tmp as usize] ^ MESSAGE_KEY[mode][base + 0xe];

            tmp = buffer[0x3];
            buffer[0x3] = message_table_index(base + 0x3)[buffer[0x7] as usize]
                ^ MESSAGE_KEY[mode][base + 0x3];
            buffer[0x7] = message_table_index(base + 0x7)[buffer[0xb] as usize]
                ^ MESSAGE_KEY[mode][base + 0x7];
            buffer[0xb] = message_table_index(base + 0xb)[buffer[0xf] as usize]
                ^ MESSAGE_KEY[mode][base + 0xb];
            buffer[0xf] =
                message_table_index(base + 0xf)[tmp as usize] ^ MESSAGE_KEY[mode][base + 0xf];

            let v = TABLE_S9[buffer[0x0] as usize]
                ^ TABLE_S9[0x100 + (buffer[0x1] as usize)]
                ^ TABLE_S9[0x200 + (buffer[0x2] as usize)]
                ^ TABLE_S9[0x300 + (buffer[0x3] as usize)];
            write_i32_le(&mut buffer, 0, v);
            let v = TABLE_S9[buffer[0x4] as usize]
                ^ TABLE_S9[0x100 + (buffer[0x5] as usize)]
                ^ TABLE_S9[0x200 + (buffer[0x6] as usize)]
                ^ TABLE_S9[0x300 + (buffer[0x7] as usize)];
            write_i32_le(&mut buffer, 4, v);
            let v = TABLE_S9[buffer[0x8] as usize]
                ^ TABLE_S9[0x100 + (buffer[0x9] as usize)]
                ^ TABLE_S9[0x200 + (buffer[0xa] as usize)]
                ^ TABLE_S9[0x300 + (buffer[0xb] as usize)];
            write_i32_le(&mut buffer, 8, v);
            let v = TABLE_S9[buffer[0xc] as usize]
                ^ TABLE_S9[0x100 + (buffer[0xd] as usize)]
                ^ TABLE_S9[0x200 + (buffer[0xe] as usize)]
                ^ TABLE_S9[0x300 + (buffer[0xf] as usize)];
            write_i32_le(&mut buffer, 12, v);
        }

        // Next, another permute with a different table
        buffer[0x0] = TABLE_S10[buffer[0x0] as usize];
        buffer[0x4] = TABLE_S10[(0x4 << 8) + (buffer[0x4] as usize)];
        buffer[0x8] = TABLE_S10[(0x8 << 8) + (buffer[0x8] as usize)];
        buffer[0xc] = TABLE_S10[(0xc << 8) + (buffer[0xc] as usize)];

        tmp = buffer[0x0d];
        buffer[0xd] = TABLE_S10[(0xd << 8) + (buffer[0x9] as usize)];
        buffer[0x9] = TABLE_S10[(0x9 << 8) + (buffer[0x5] as usize)];
        buffer[0x5] = TABLE_S10[(0x5 << 8) + (buffer[0x1] as usize)];
        buffer[0x1] = TABLE_S10[(0x1 << 8) + (tmp as usize)];

        tmp = buffer[0x02];
        buffer[0x2] = TABLE_S10[(0x2 << 8) + (buffer[0xa] as usize)];
        buffer[0xa] = TABLE_S10[(0xa << 8) + (tmp as usize)];
        tmp = buffer[0x06];
        buffer[0x6] = TABLE_S10[(0x6 << 8) + (buffer[0xe] as usize)];
        buffer[0xe] = TABLE_S10[(0xe << 8) + (tmp as usize)];

        tmp = buffer[0x3];
        buffer[0x3] = TABLE_S10[(0x3 << 8) + (buffer[0x7] as usize)];
        buffer[0x7] = TABLE_S10[(0x7 << 8) + (buffer[0xb] as usize)];
        buffer[0xb] = TABLE_S10[(0xb << 8) + (buffer[0xf] as usize)];
        buffer[0xf] = TABLE_S10[(0xf << 8) + (tmp as usize)];

        let mut xor_result = [0; 16];

        if mode == 2 || mode == 1 || mode == 0 {
            if i > 0 {
                xor_blocks(
                    &buffer,
                    &message_in[0x10 * i..0x10 * i + 16],
                    &mut xor_result,
                ); // remember that the first 0x10 bytes are the header
                decrypted_message[0x10 * i..0x10 * i + 16].copy_from_slice(&xor_result);
            } else {
                xor_blocks(&buffer, &MESSAGE_IV[mode], &mut xor_result);
                decrypted_message[0x10 * i..0x10 * i + 16].copy_from_slice(&xor_result);
            }
        } else if i < 7 {
            xor_blocks(
                &buffer,
                &message_in[0x70 - 0x10 * i..(0x70 - 0x10 * i) + 16],
                &mut xor_result,
            );
            decrypted_message[0x70 - 0x10 * i..(0x70 - 0x10 * i) + 16].copy_from_slice(&xor_result);
        } else {
            xor_blocks(&buffer, &MESSAGE_IV[mode], &mut xor_result);
            decrypted_message[0x70 - 0x10 * i..(0x70 - 0x10 * i) + 16].copy_from_slice(&xor_result);
        }
    }
}

fn generate_key_schedule(key_material: &[u8], key_schedule: &mut [&mut [i32]]) {
    let mut key_data = [0; 4];
    key_schedule[..11].iter_mut().for_each(|v| {
        v[0] = 0xdeadbeefusize as i32;
        v[1] = 0xdeadbeefusize as i32;
        v[2] = 0xdeadbeefusize as i32;
        v[3] = 0xdeadbeefusize as i32;
    });
    let mut buffer = [0; 16];
    let mut ti = 0;
    // G
    t_xor(key_material, &mut buffer);

    key_data[..4]
        .iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = get_i32_le(&buffer, i * 4));
    // for i in 0..4 {
    //     key_data[i] = get_i32_le(&buffer, i * 4);
    // }

    for round in 0..11 {
        // H
        key_schedule[round][0] = key_data[0];
        // I
        let table1 = table_index(ti);
        let table2 = table_index(ti + 1);
        let table3 = table_index(ti + 2);
        let table4 = table_index(ti + 3);
        ti += 4;

        buffer[0] ^= table1[buffer[0x0d] as usize] ^ INDEX_MANGLE[round];
        buffer[1] ^= table2[buffer[0x0e] as usize];
        buffer[2] ^= table3[buffer[0x0f] as usize];
        buffer[3] ^= table4[buffer[0x0c] as usize];

        key_data[0] = get_i32_le(&buffer, 0);

        // H
        key_schedule[round][1] = key_data[1];
        // J
        key_data[1] ^= key_data[0];
        write_i32_le(&mut buffer, 4, key_data[1]);
        // H
        key_schedule[round][2] = key_data[2];
        // J
        key_data[2] ^= key_data[1];
        write_i32_le(&mut buffer, 8, key_data[2]);
        // K and L
        // Implement K and L to fill in other bits of the key schedule
        key_schedule[round][3] = key_data[3];
        // J again
        key_data[3] ^= key_data[2];
        write_i32_le(&mut buffer, 12, key_data[3]);
    }

    // for i in 0..11 {
    //     let mut tmp = [0; 16];
    //     for j in 0..4 {
    //         write_i32_le(&mut tmp, j * 4, key_schedule[i][j]);
    //     }
    // }
}

fn generate_session_key(old_sap: &[u8], message_in: &[u8], session_key: &mut [u8]) {
    let mut decrypted_message = [0; 128];
    let mut new_sap = [0; 320];
    // let mut round;
    let mut md5 = [0; 16];

    decrypt_message(message_in, &mut decrypted_message);

    new_sap[0..0x11].copy_from_slice(&STATIC_SOURCE_1);
    new_sap[0x11..0x11 + 0x80].copy_from_slice(&decrypted_message);
    new_sap[0x091..0x091 + 0x80].copy_from_slice(&old_sap[0x80..0x80 + 0x80]);
    new_sap[0x111..0x111 + 0x2f].copy_from_slice(&STATIC_SOURCE_2);
    session_key.copy_from_slice(&INITIAL_SESSION_KEY);

    for round in 0..5 {
        let base = &new_sap[round * 64..new_sap.len()];
        modified_md5::modified_md5(base, session_key, &mut md5);
        sap_hash::sap_hash(base, session_key);

        for i in 0..4 {
            let v = ((get_i32_le(session_key, i * 4) + get_i32_le(&md5, i * 4)) as usize
                & 0xffffffff) as i32;
            write_i32_le(session_key, i * 4, v);
        }
    }

    for i in 0..4 {
        let i = i * 4;
        session_key.swap(i, i + 3);
        session_key.swap(i + 1, i + 2);
    }

    session_key.iter_mut().for_each(|v| *v ^= 121);
}

fn cycle(block: &mut [u8], key_schedule: &[&[i32]]) {
    let (mut ptr1, mut ptr2, mut ptr3, mut ptr4, mut ab);
    let i0 = get_i32_le(block, 0) ^ key_schedule[10][0];
    let i1 = get_i32_le(block, 4) ^ key_schedule[10][1];
    let i2 = get_i32_le(block, 8) ^ key_schedule[10][2];
    let i3 = get_i32_le(block, 12) ^ key_schedule[10][3];
    write_i32_le(block, 0, i0);
    write_i32_le(block, 4, i1);
    write_i32_le(block, 8, i2);
    write_i32_le(block, 12, i3);

    permute_block_1(block);

    for round in 0..9 {
        let mut key = [0; 16];
        for i in 0..4 {
            write_i32_le(&mut key, i * 4, key_schedule[9 - round][i]);
        }
        ptr1 = TABLE_S5[(block[3] ^ key[3]) as usize];
        ptr2 = TABLE_S6[(block[2] ^ key[2]) as usize];
        ptr3 = TABLE_S8[(block[0] ^ key[0]) as usize];
        ptr4 = TABLE_S7[(block[1] ^ key[1]) as usize];

        // A B
        ab = ptr1 ^ ptr2 ^ ptr3 ^ ptr4;

        write_i32_le(block, 0, ab);

        ptr2 = TABLE_S5[(block[7] ^ key[7]) as usize];
        ptr1 = TABLE_S6[(block[6] ^ key[6]) as usize];
        ptr4 = TABLE_S7[(block[5] ^ key[5]) as usize];
        ptr3 = TABLE_S8[(block[4] ^ key[4]) as usize];

        // A B again
        ab = ptr1 ^ ptr2 ^ ptr3 ^ ptr4;
        write_i32_le(block, 4, ab);

        write_i32_le(
            block,
            8,
            TABLE_S5[(block[11] ^ key[11]) as usize]
                ^ TABLE_S6[(block[10] ^ key[10]) as usize]
                ^ TABLE_S7[(block[9] ^ key[9]) as usize]
                ^ TABLE_S8[(block[8] ^ key[8]) as usize],
        );

        write_i32_le(
            block,
            12,
            TABLE_S5[(block[15] ^ key[15]) as usize]
                ^ TABLE_S6[(block[14] ^ key[14]) as usize]
                ^ TABLE_S7[(block[13] ^ key[13]) as usize]
                ^ TABLE_S8[(block[12] ^ key[12]) as usize],
        );

        permute_block_2(block, 8 - round);
    }

    let i0 = get_i32_le(block, 0) ^ key_schedule[0][0];
    let i1 = get_i32_le(block, 4) ^ key_schedule[0][1];
    let i2 = get_i32_le(block, 8) ^ key_schedule[0][2];
    let i3 = get_i32_le(block, 12) ^ key_schedule[0][3];
    write_i32_le(block, 0, i0);
    write_i32_le(block, 4, i1);
    write_i32_le(block, 8, i2);
    write_i32_le(block, 12, i3);
}

fn xor_blocks(a: &[u8], b: &[u8], out: &mut [u8]) {
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
}

fn z_xor_f<F>(blocks: usize, mut f: F)
where
    F: FnMut((usize, u8)),
{
    for j in 0..blocks {
        for (i, v) in Z_KEY.iter().enumerate() {
            f((j * 16 + i, *v));
        }
    }
}

fn z_xor(inner: &[u8], out: &mut [u8], blocks: usize) {
    for j in 0..blocks {
        for i in 0..16 {
            out[j * 16 + i] = inner[j * 16 + i] ^ Z_KEY[i];
        }
    }
}

fn x_xor_f<F>(blocks: usize, mut f: F)
where
    F: FnMut((usize, u8)),
{
    for j in 0..blocks {
        for (i, v) in X_KEY.iter().enumerate() {
            f((j * 16 + i, *v));
        }
    }
}

// fn x_xor(inner: &[u8], out: &mut [u8], blocks: usize) {
//     for j in 0..blocks {
//         for i in 0..16 {
//             out[j * 16 + i] = inner[j * 16 + i] ^ X_KEY[i];
//         }
//     }
// }

fn t_xor(inner: &[u8], out: &mut [u8]) {
    for i in 0..16 {
        out[i] = inner[i] ^ T_KEY[i];
    }
}

fn table_index(i: usize) -> &'static [u8] {
    &TABLE_S1[((31 * i) % 0x28) << 8..TABLE_S1.len()]
}

fn message_table_index(i: usize) -> &'static [u8] {
    &TABLE_S2[(97 * i % 144) << 8..TABLE_S2.len()]
}

fn permute_block_1(block: &mut [u8]) {
    block[0] = TABLE_S3[block[0] as usize];
    block[4] = TABLE_S3[0x400 + (block[4] as usize)];
    block[8] = TABLE_S3[0x800 + (block[8] as usize)];
    block[12] = TABLE_S3[0xc00 + (block[12] as usize)];

    let mut tmp = block[13];
    block[13] = TABLE_S3[0x100 + (block[9] as usize)];
    block[9] = TABLE_S3[0xd00 + (block[5] as usize)];
    block[5] = TABLE_S3[0x900 + (block[1] as usize)];
    block[1] = TABLE_S3[0x500 + (tmp as usize)];

    tmp = block[2];
    block[2] = TABLE_S3[0xa00 + (block[10] as usize)];
    block[10] = TABLE_S3[0x200 + (tmp as usize)];
    tmp = block[6];
    block[6] = TABLE_S3[0xe00 + (block[14] as usize)];
    block[14] = TABLE_S3[0x600 + (tmp as usize)];

    tmp = block[3];
    block[3] = TABLE_S3[0xf00 + (block[7] as usize)];
    block[7] = TABLE_S3[0x300 + (block[11] as usize)];
    block[11] = TABLE_S3[0x700 + (block[15] as usize)];
    block[15] = TABLE_S3[0xb00 + (tmp as usize)];
}

fn permute_table_2(i: usize) -> &'static [u8] {
    &TABLE_S4[((71 * i) % 144) << 8..TABLE_S4.len()]
}

fn permute_block_2(block: &mut [u8], round: usize) {
    block[0] = permute_table_2(round * 16)[block[0] as usize];
    block[4] = permute_table_2(round * 16 + 4)[block[4] as usize];
    block[8] = permute_table_2(round * 16 + 8)[block[8] as usize];
    block[12] = permute_table_2(round * 16 + 12)[block[12] as usize];

    let mut tmp = block[13];
    block[13] = permute_table_2(round * 16 + 13)[block[9] as usize];
    block[9] = permute_table_2(round * 16 + 9)[block[5] as usize];
    block[5] = permute_table_2(round * 16 + 5)[block[1] as usize];
    block[1] = permute_table_2(round * 16 + 1)[tmp as usize];

    tmp = block[2];
    block[2] = permute_table_2(round * 16 + 2)[block[10] as usize];
    block[10] = permute_table_2(round * 16 + 10)[tmp as usize];
    tmp = block[6];
    block[6] = permute_table_2(round * 16 + 6)[block[14] as usize];
    block[14] = permute_table_2(round * 16 + 14)[tmp as usize];

    tmp = block[3];
    block[3] = permute_table_2(round * 16 + 3)[block[7] as usize];
    block[7] = permute_table_2(round * 16 + 7)[block[11] as usize];
    block[11] = permute_table_2(round * 16 + 11)[block[15] as usize];
    block[15] = permute_table_2(round * 16 + 15)[tmp as usize];
}

#[cfg(test)]
mod tests {
    use tp_macro::jb_to_rb;

    use crate::airplay::lib::omg_hax_const::{DEFAULT_SAP, INITIAL_SESSION_KEY};

    #[test]
    fn generate_session_key() {
        let message_in = jb_to_rb!([
            70, 80, 76, 89, 3, 1, 3, 0, 0, 0, 0, -104, 0, -113, 26, -100, -40, -92, -10, 52, 109,
            20, 120, 6, -62, -67, -118, 75, -47, -71, -109, -45, -61, 106, -95, 1, 36, -104, -7,
            78, -1, -13, 70, 123, -49, 27, 49, -104, 98, 92, -94, 69, -114, 62, -48, 30, -35, 53,
            -25, 41, 53, 125, -7, 75, -128, -51, 10, -50, 35, 84, -42, -116, -29, 127, 94, 24, -16,
            -49, -46, 109, 65, 103, 21, 63, -64, -76, 54, 35, 22, 111, 8, -58, 111, -45, 1, 56, 14,
            -80, -98, -97, -115, -24, 59, -46, -82, -57, -92, 1, -15, -5, -67, -13, 46, 10, -43,
            81, -24, 121, 63, -25, -63, 25, 35, 51, -103, -91, 53, 76, -59, 67, 7, 30, -68, -50,
            -32, -84, -123, 34, -82, 27, -85, 51, -44, 65, -60, 120, -11, 99, -50, -3, 66, 117, -5,
            85, 90, 58, -29, 58, -40, -71, -7, -108, -7, -75
        ]);
        let mut session_key = [0; 16];

        super::generate_session_key(&DEFAULT_SAP, &message_in, &mut session_key);

        assert_eq!(
            jb_to_rb!([39, 110, -67, 89, -58, 116, 70, 37, 101, -9, -9, -68, -58, 68, 4, 50]),
            session_key
        );
    }

    #[test]
    fn decrypt_message1_text() {
        let message_in = jb_to_rb!([
            70, 80, 76, 89, 3, 1, 3, 0, 0, 0, 0, -104, 0, -113, 26, -100, -40, -92, -10, 52, 109,
            20, 120, 6, -62, -67, -118, 75, -47, -71, -109, -45, -61, 106, -95, 1, 36, -104, -7,
            78, -1, -13, 70, 123, -49, 27, 49, -104, 98, 92, -94, 69, -114, 62, -48, 30, -35, 53,
            -25, 41, 53, 125, -7, 75, -128, -51, 10, -50, 35, 84, -42, -116, -29, 127, 94, 24, -16,
            -49, -46, 109, 65, 103, 21, 63, -64, -76, 54, 35, 22, 111, 8, -58, 111, -45, 1, 56, 14,
            -80, -98, -97, -115, -24, 59, -46, -82, -57, -92, 1, -15, -5, -67, -13, 46, 10, -43,
            81, -24, 121, 63, -25, -63, 25, 35, 51, -103, -91, 53, 76, -59, 67, 7, 30, -68, -50,
            -32, -84, -123, 34, -82, 27, -85, 51, -44, 65, -60, 120, -11, 99, -50, -3, 66, 117, -5,
            85, 90, 58, -29, 58, -40, -71, -7, -108, -7, -75
        ]);
        let mut decrypted_message = [0; 128];
        super::decrypt_message(&message_in, &mut decrypted_message);

        assert_eq!(
            jb_to_rb!([
                0, 1, 30, -125, 126, 116, 70, 103, -73, -27, 121, -8, 114, 3, 1, -121, -45, -63,
                15, 43, -41, 99, -50, 92, -84, 43, 121, -76, 55, -51, -97, 5, -32, -7, -68, 100,
                108, -14, 29, 86, 64, -60, 37, -93, -125, -111, 113, -55, -25, 98, -103, -49, 98,
                -55, 82, 124, 29, 52, -57, -3, 12, 58, -42, -121, 115, -63, 99, 90, 126, -108, 28,
                -48, -9, -19, -94, 72, 6, 99, -48, -78, -113, 35, -5, -47, -114, 63, -36, 66, -126,
                -32, -93, -99, -29, 111, 57, -106, -45, 126, -1, 112, 28, -83, 46, -74, -87, -118,
                -108, 112, 71, -112, 100, -80, -83, 20, 79, -94, 8, 111, 16, -1, -65, -36, -17, 69,
                -70, 120, 121, 8
            ]),
            decrypted_message
        );
    }

    #[test]
    fn decrypt_message2_text() {
        let message_in = jb_to_rb!([
            70, 80, 76, 89, 3, 1, 3, 0, 0, 0, 0, -104, 3, -113, 26, -100, 55, -15, -52, -74, -24,
            121, 37, 74, 109, -82, -109, 93, -65, -76, 89, -16, 27, -41, -45, -99, 109, 45, -95,
            44, -68, 21, -24, -93, -78, 41, 38, -104, 80, 43, 96, 123, -90, 127, -125, -44, -100,
            0, -48, 35, 38, 66, 15, 60, -37, -8, -48, -84, 61, -95, 96, 25, -53, 56, 112, -10, -6,
            -73, -82, 81, -50, -33, -62, -115, -90, 6, 83, 59, 4, -13, -114, 84, 4, 32, 64, -64,
            -104, -42, 84, -57, -37, 14, 100, -58, 20, 82, -85, 70, 125, 115, 106, -57, 21, -43,
            -11, 54, -61, -3, -66, 104, 69, 54, 92, 100, -21, -120, -10, 29, 70, 92, 53, 105, 28,
            40, -128, 37, 56, -37, 47, -53, -47, 32, 40, -125, 12, 117, -27, 80, 12, -78, -102,
            101, -107, 93, -111, 47, -41, -94, -124, -86, 72, 63, 20, -114
        ]);
        let mut decrypted_message = [0; 128];
        super::decrypt_message(&message_in, &mut decrypted_message);

        assert_eq!(
            jb_to_rb!([
                0, 1, -99, -23, 114, -121, 82, -36, 7, -125, 74, 78, -43, 47, -83, 116, 45, -85,
                -25, -76, -92, -5, -13, 39, -55, -31, 32, 79, 121, -67, 107, 89, -15, -126, 97,
                -61, 26, -76, 49, 115, -83, 61, 110, -70, -91, 29, -90, 21, -118, -25, -51, -74,
                26, 87, -74, -78, -30, 43, -48, -73, -69, 74, -63, 56, 74, -46, -50, 38, 6, -11, 4,
                -87, 28, 55, 1, -86, -53, -13, 112, -70, -71, -21, 52, -93, 66, -64, 27, 51, -52,
                -117, -73, -76, 90, 33, 83, -33, 90, -38, -35, 43, 29, -95, -67, 106, -119, -115,
                23, -103, 67, 116, 22, 38, 5, 79, -37, -119, 114, -8, 10, 116, 12, -20, 25, 82,
                -38, -31, 97, -78
            ]),
            decrypted_message
        );
    }

    #[test]
    fn generate_key_schedule1_test() {
        let mut key_schedule = [[0; 4]; 11];
        let mut key_schedule_mut: Vec<&mut [i32]> =
            key_schedule.iter_mut().map(|v| &mut v[..]).collect();
        super::generate_key_schedule(&INITIAL_SESSION_KEY, &mut key_schedule_mut);

        assert_eq!(
            [-665135092, 2094846048, 1974007022, -129415421],
            key_schedule[0]
        );
        assert_eq!(
            [1657169001, 505066505, 1806844135, -1812264988],
            key_schedule[1]
        );
        assert_eq!(
            [-168858664, -336207919, -2142789834, 331126994],
            key_schedule[2]
        );
        assert_eq!(
            [923012806, -588132073, 1555451425, 1326110451],
            key_schedule[3]
        );
        assert_eq!(
            [-1487983013, 2076114764, 654881133, 1744988062],
            key_schedule[4]
        );
        assert_eq!(
            [317585804, 1767048896, 1314633645, 643420211],
            key_schedule[5]
        );
        assert_eq!(
            [1119227396, 736430276, 1706999657, 1139240794],
            key_schedule[6]
        );
        assert_eq!(
            [1815178368, 1205101636, 577428269, 1636676727],
            key_schedule[7]
        );
        assert_eq!(
            [-1671784215, -611386195, -102488192, -1737911305],
            key_schedule[8]
        );
        assert_eq!(
            [1548300438, -2016949189, 2116283323, -431327156],
            key_schedule[9]
        );
        assert_eq!(
            [-488839655, 1696279074, 456704409, -42812971],
            key_schedule[10]
        );
    }

    #[test]
    fn generate_key_schedule2_test() {
        let key_material =
            jb_to_rb!([39, 110, -67, 89, -58, 116, 70, 37, 101, -9, -9, -68, -58, 68, 4, 50]);
        let mut key_schedule = [[0; 4]; 11];
        let mut key_schedule_mut: Vec<&mut [i32]> =
            key_schedule.iter_mut().map(|v| &mut v[..]).collect();
        super::generate_key_schedule(&key_material, &mut key_schedule_mut);

        assert_eq!(
            [940862199, -1572417363, -1448575987, -1679986221],
            key_schedule[0]
        );
        assert_eq!(
            [571422622, -2142640333, 702651198, -1304630547],
            key_schedule[1]
        );
        assert_eq!(
            [-894254476, 1257973063, 1662697081, -786017132],
            key_schedule[2]
        );
        assert_eq!(
            [-2081822342, -921514947, -1442287036, 2066622160],
            key_schedule[3]
        );
        assert_eq!(
            [-105003103, 816778140, -1700300328, -511090936],
            key_schedule[4]
        );
        assert_eq!(
            [-118041420, -933634264, 1392420592, -1283982856],
            key_schedule[5]
        );
        assert_eq!(
            [265498950, -947196306, -1787541346, 637776230],
            key_schedule[6]
        );
        assert_eq!(
            [726154226, -322775652, 2042010882, 1605708900],
            key_schedule[7]
        );
        assert_eq!(
            [1995586036, -1708069784, -477728406, -1137484530],
            key_schedule[8]
        );
        assert_eq!(
            [84681731, -1623406485, 2092611841, -1064705009],
            key_schedule[9]
        );
        assert_eq!(
            [-787235461, 1311733008, 848677905, -233035746],
            key_schedule[10]
        );
    }

    #[test]
    fn cycle_test() {
        let mut block_in =
            jb_to_rb!([-3, -95, -97, 118, 127, 54, 34, -29, -72, 30, 20, -60, -43, -124, -13, 21]);
        let key_schedule = [
            [940862199, -1572417363, -1448575987, -1679986221],
            [571422622, -2142640333, 702651198, -1304630547],
            [-894254476, 1257973063, 1662697081, -786017132],
            [-2081822342, -921514947, -1442287036, 2066622160],
            [-105003103, 816778140, -1700300328, -511090936],
            [-118041420, -933634264, 1392420592, -1283982856],
            [265498950, -947196306, -1787541346, 637776230],
            [726154226, -322775652, 2042010882, 1605708900],
            [1995586036, -1708069784, -477728406, -1137484530],
            [84681731, -1623406485, 2092611841, -1064705009],
            [-787235461, 1311733008, 848677905, -233035746],
        ];

        let key_schedule_ref: Vec<&[i32]> = key_schedule.iter().map(|v| &v[..]).collect();

        super::cycle(&mut block_in, &key_schedule_ref);

        assert_eq!(
            jb_to_rb!([-104, -5, -75, -79, 27, 76, -72, -54, 72, -15, 9, 99, -29, 29, 36, 124]),
            block_in
        );
    }
}
