#![allow(non_snake_case)]
pub fn garble(
    buffer0: &mut [u8],
    buffer1: &mut [u8],
    buffer2: &mut [u8],
    buffer3: &mut [u8],
    buffer4: &mut [u8],
) {
    let (tmp, tmp2, tmp3);
    let (mut A, mut B, mut C, mut D, mut E, M, J, mut G, mut F, H, K, R, S, T, U, V, W, X, Y, Z);

    buffer2[12] = 0x14
        + ((((buffer1[64]) & 92) | (((buffer1[99]) / 3) & 35))
            & (buffer4[(rol8x(buffer4[(buffer1[206] % 21) as usize] as isize, 4) % 21) as usize]));

    buffer1[4] = (buffer1[99] / 5) * (buffer1[99] / 5) * 2;

    // Simpler still!
    buffer2[34] = 0xb8;

    buffer1[153] ^= buffer2[(buffer1[203] % 35) as usize]
        * buffer2[(buffer1[203] % 35) as usize]
        * buffer1[190];

    // This one looks simple, but wow was it not :(
    buffer0[3] -= (((buffer4[(buffer1[205] % 21) as usize] >> 1) & 80) as i32 | 0xe6440) as u8;

    // This is always 0x93
    buffer0[16] = 0x93;

    // This is always 0x62
    buffer0[13] = 0x62;

    buffer1[33] -= buffer4[(buffer1[36] % 21) as usize] & 0xf6;

    // This is always 7
    tmp2 = buffer2[(buffer1[67] % 35) as usize];
    buffer2[12] = 0x07;

    // This is pretty easy!
    tmp = buffer0[(buffer1[181] % 20) as usize];
    buffer1[2] = (buffer1[2] as i32 - 3136) as u8;

    buffer0[19] = buffer4[(buffer1[58] % 21) as usize];

    buffer3[0] = 92 - buffer2[(buffer1[32] % 35) as usize];

    buffer3[4] = buffer2[(buffer1[15] % 35) as usize] + 0x9e;

    buffer1[34] += buffer4[((buffer2[(buffer1[15] % 35) as usize] + 0x9e) % 21) as usize] / 5;

    buffer0[19] = ((buffer0[19] as i64) + 0xfffffee6
        - ((buffer0[(buffer3[4] % 20) as usize] >> 1) & 102) as i64) as u8;

    // This LOOKS like it should be a rol8x, but it just doesnt work out because if the shift amount is 0, then the output is 0 too :(
    // FIXME: Switch to weird_ror8
    buffer1[15] ^= 3
        * (((buffer1[72] >> (buffer4[(buffer1[190] % 21) as usize] & 7))
            ^ ((buffer1[72]) << ((7 - ((buffer4[(buffer1[190] % 21) as usize]) - 1)) & 7)))
            - (3 * buffer4[(buffer1[126] % 21) as usize]));

    buffer0[15] ^= (buffer2[(buffer1[181] % 35) as usize])
        * (buffer2[(buffer1[181] % 35) as usize])
        * (buffer2[(buffer1[181] % 35) as usize]);

    buffer2[4] ^= (buffer1[202]) / 3;

    // This could probably be quite a bit simpler.
    A = (92 - (buffer0[(buffer3[0] % 20) as usize])) as isize;
    E = ((A as u8 & 0xc6) | (!(buffer1[105]) & 0xc6) | (A as u8 & (!(buffer1[105])))) as isize;
    buffer2[1] += (E * E * E) as u8;

    buffer0[19] ^= (((224 | ((buffer4[(buffer1[92] % 21) as usize]) & 27)) as isize
        * (buffer2[(buffer1[41] % 35) as usize]) as isize)
        / 3) as u8;

    buffer1[140] += weird_ror8(92, (buffer1[5] as isize) & 7) as u8;

    // Is this as simple as it could be?
    buffer2[12] += ((((!(buffer1[4])) ^ (buffer2[(buffer1[12] % 35) as usize])) | (buffer1[182]))
        & 192)
        | (((!(buffer1[4])) ^ (buffer2[(buffer1[12] % 35) as usize])) & (buffer1[182]));

    buffer1[36] += 125;

    buffer1[124] = rol8x(
        ((((74 & (buffer1[138])) | ((74 | (buffer1[138])) & (buffer0[15])))
            & (buffer0[(buffer1[43] % 20) as usize]))
            | (((74 & (buffer1[138]))
                | ((74 | (buffer1[138])) & (buffer0[15]))
                | (buffer0[(buffer1[43] % 20) as usize]))
                & 95)) as isize,
        4,
    ) as u8;

    buffer3[8] = ((((buffer0[(buffer3[4] % 20) as usize]) & 95)
        & (((buffer4[(buffer1[68] % 21) as usize]) & 46) << 1))
        | 16)
        ^ 92;

    A = (buffer1[177] as isize) + (buffer4[(buffer1[79] % 21) as usize] as isize);

    D = (((A >> 1) | ((3 * (buffer1[148] as isize)) / 5)) & buffer2[1] as isize)
        | ((A >> 1) & ((3 * (buffer1[148] as isize)) / 5));

    buffer3[12] = (222 - D) as u8;

    A = 8 - ((buffer2[22]) & 7) as isize; // FIXME: buffer2[22] = 74, so A is always 6 and B^C is just ror8(buffer1[33], 6)
    B = ((buffer1[33]) >> (A & 7)) as isize;
    C = ((buffer1[33]) << ((buffer2[22]) & 7)) as isize;
    buffer2[16] +=
        (((buffer2[(buffer3[0] % 35) as usize]) & 159) | (buffer0[(buffer3[4] % 20) as usize]) | 8)
            - ((B ^ C) | 128) as u8;

    // This one was very easy so I just skipped ahead and did it
    buffer0[14] ^= buffer2[(buffer3[12] % 35) as usize];

    // Monster goes here
    A = weird_rol8(
        buffer4[(buffer0[(buffer1[201] % 20) as usize] % 21) as usize] as isize,
        ((buffer2[(buffer1[112] % 35) as usize]) << 1) as isize & 7,
    );
    D = ((buffer0[(buffer1[208] % 20) as usize] & 131)
        | ((buffer0[(buffer1[164] % 20) as usize]) & 124)) as isize;
    buffer1[19] += ((A & (D / 5)) | ((A | (D / 5)) & 37)) as u8;

    buffer2[8] = weird_ror8(
        140,
        ((((buffer4[(buffer1[45] % 21) as usize]) + 92)
            * ((buffer4[(buffer1[45] % 21) as usize]) + 92))
            & 7) as isize,
    ) as u8;

    buffer1[190] = 56;

    buffer2[8] ^= buffer3[0];

    buffer1[53] = !(((buffer0[(buffer1[83] % 20) as usize]) | 204) / 5);

    buffer0[13] += buffer0[(buffer1[41] % 20) as usize];

    buffer0[10] = (((buffer2[(buffer3[0] % 35) as usize]) & (buffer1[2]))
        | (((buffer2[(buffer3[0] % 35) as usize]) | (buffer1[2])) & (buffer3[12])))
        / 15;

    A = ((((56 | ((buffer4[(buffer1[2] % 21) as usize]) & 68))
        | (buffer2[(buffer3[8] % 35) as usize]))
        & 42)
        | ((((buffer4[(buffer1[2] % 21) as usize]) & 68) | 56)
            & (buffer2[(buffer3[8] % 35) as usize]))) as isize;
    buffer3[16] = (A * A) as u8 + 110;

    buffer3[20] = 202 - (buffer3[16]);

    buffer3[24] = buffer1[151];

    buffer2[13] ^= buffer4[(buffer3[0] % 21) as usize];

    B = (((buffer2[(buffer1[179] % 35) as usize]) as isize - 38) & 177)
        | (buffer3[12] as isize & 177);
    C = ((buffer2[(buffer1[179] % 35) as usize]) as isize - 38) & buffer3[12] as isize;
    buffer3[28] = (30 + ((B | C) * (B | C))) as u8;

    buffer3[32] = buffer3[28] + 62;

    // eek
    A = (((buffer3[20]) + ((buffer3[0]) & 74)) | !(buffer4[(buffer3[0] % 21) as usize])) as isize
        & 121;
    B = (((buffer3[20]) + ((buffer3[0]) & 74)) & !(buffer4[(buffer3[0] % 21) as usize])) as isize;
    tmp3 = A | B;
    C = (((((A | B) as i64 ^ 0xffffffa6) | buffer3[0] as i64) & 4)
        | (((A | B) as i64 ^ 0xffffffa6) & buffer3[0] as i64)) as isize;
    buffer1[47] ^= ((buffer2[(buffer1[89] % 35) as usize]) as isize + C) as u8;

    buffer3[36] =
        ((rol8((tmp as isize & 179) + 68, 2) as u8 & (buffer0[3])) | (tmp2 & !(buffer0[3]))) - 15;

    buffer1[123] ^= 221;

    A = ((buffer4[(buffer3[0] % 21) as usize]) as isize / 3)
        - (buffer2[(buffer3[4] % 35) as usize] as isize);
    C = (((buffer3[0] as isize & 163) + 92) & 246) | (buffer3[0] as isize & 92);
    E = ((C | buffer3[24] as isize) & 54) | (C & buffer3[24] as isize);
    buffer3[40] = (A - E) as u8;

    buffer3[44] = (tmp3 ^ 81 ^ ((((buffer3[0] as isize) >> 1) & 101) + 26)) as u8;

    buffer3[48] = (buffer2[(buffer3[4] % 35) as usize]) & 27;

    buffer3[52] = 27;

    buffer3[56] = 199;

    // caffeine
    buffer3[64] = (buffer3[4])
        + ((((((((buffer3[40]) | (buffer3[24])) & 177) | ((buffer3[40]) & (buffer3[24])))
            & ((((buffer4[(buffer3[0] % 20) as usize]) & 177) | 176)
                | ((buffer4[(buffer3[0] % 21) as usize]) & !3)))
            | (((((buffer3[40]) & (buffer3[24])) | (((buffer3[40]) | (buffer3[24])) & 177))
                & 199)
                | (((((buffer4[(buffer3[0] % 21) as usize]) & 1) + 176)
                    | ((buffer4[(buffer3[0] % 21) as usize]) & !3))
                    & (buffer3[56]))))
            & (!(buffer3[52])))
            | (buffer3[48]));

    buffer2[33] ^= buffer1[26];

    buffer1[106] ^= buffer3[20] ^ 133;

    buffer2[30] = (((buffer3[64]) / 3) - (275 | ((buffer3[0]) & 247) as i32) as u8)
        ^ (buffer0[(buffer1[122] % 20) as usize]);

    buffer1[22] = ((buffer2[(buffer1[90] % 35) as usize]) & 95) | 68;

    A = ((buffer4[(buffer3[36] % 21) as usize]) as isize & 184)
        | ((buffer2[(buffer3[44] % 35) as usize]) as isize & !184);

    buffer2[18] += ((A * A * A) >> 1) as u8;

    buffer2[5] -= buffer4[(buffer1[92] % 21) as usize];

    A = (((((buffer1[41]) & !24) | ((buffer2[(buffer1[183] % 35) as usize]) & 24))
        & ((buffer3[16]) + 53))
        | (buffer3[20] & (buffer2[(buffer3[20] % 35) as usize]))) as isize;
    B = (((buffer1[17]) & (!(buffer3[44])))
        | ((buffer0[(buffer1[59] % 20) as usize]) & (buffer3[44]))) as isize;
    buffer2[18] ^= (A * B) as u8;

    A = weird_ror8(
        buffer1[11] as isize,
        (buffer2[(buffer1[28] % 35) as usize]) as isize & 7,
    ) & 7;
    B = (((((buffer0[(buffer1[93] % 20) as usize]) & !(buffer0[14])) | ((buffer0[14]) & 150))
        & !28)
        | ((buffer1[7]) & 28)) as isize;
    buffer2[22] = ((((B | weird_rol8(buffer2[(buffer3[0] % 35) as usize] as isize, A))
        & (buffer2[33] as isize))
        | (B & weird_rol8(buffer2[(buffer3[0] % 35) as usize] as isize, A)))
        + 74) as u8;

    A = buffer4[(((buffer0[(buffer1[39] % 20) as usize]) ^ 217) % 21) as usize] as isize;
    buffer0[15] -= (((((buffer3[20]) | (buffer3[0])) & 214) | ((buffer3[20]) & (buffer3[0])))
        & A as u8)
        | (((((buffer3[20]) | (buffer3[0])) & 214) | ((buffer3[20]) & (buffer3[0])) | A as u8)
            & (buffer3[32]));

    // We need to save T here, and boy is it complicated to calculate!
    B = (((buffer2[(buffer1[57] % 35) as usize] & buffer0[(buffer3[64] % 20) as usize])
        | ((buffer0[(buffer3[64] % 20) as usize] | buffer2[(buffer1[57] % 35) as usize]) & 95)
        | (buffer3[64] & 45)
        | 82)
        & 32) as isize;
    C = (((buffer2[(buffer1[57] % 35) as usize] & buffer0[(buffer3[64] % 20) as usize])
        | ((buffer2[(buffer1[57] % 35) as usize] | buffer0[(buffer3[64] % 20) as usize]) & 95))
        & ((buffer3[64] & 45) | 82)) as isize;
    D = ((((buffer3[0]) / 3) - ((buffer3[64]) | (buffer1[22])))
        ^ ((buffer3[28]) + 62)
        ^ (B | C) as u8) as isize;
    T = buffer0[(D % 20) as usize];

    buffer3[68] = ((buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize]))
        | (buffer2[(buffer3[64] % 35) as usize]);

    U = buffer0[(buffer1[50] % 20) as usize]; // this is also v100
    W = buffer2[(buffer1[138] % 35) as usize];
    X = buffer4[(buffer1[39] % 21) as usize];
    Y = buffer0[(buffer1[4] % 20) as usize]; // this is also v120
    Z = buffer4[(buffer1[202] % 21) as usize]; // also v124
    V = buffer0[(buffer1[151] % 20) as usize];
    S = buffer2[(buffer1[14] % 35) as usize];
    R = buffer0[(buffer1[145] % 20) as usize];

    A = (((buffer2[(buffer3[68] % 35) as usize]) & (buffer0[(buffer1[209] % 20) as usize]))
        | (((buffer2[(buffer3[68] % 35) as usize]) | (buffer0[(buffer1[209] % 20) as usize])) & 24))
        as isize;
    B = weird_rol8(
        buffer4[(buffer1[127] % 21) as usize] as isize,
        (buffer2[(buffer3[68] % 35) as usize]) as isize & 7,
    );
    C = ((A as u8 & (buffer0[10])) | (B as u8 & !(buffer0[10]))) as isize;
    D = 7 ^ ((buffer4[(buffer2[(buffer3[36] % 35) as usize] % 21) as usize] as isize) << 1);
    buffer3[72] = ((C & 71) | (D & !71)) as u8;

    buffer2[2] += ((((buffer0[(buffer3[20] % 20) as usize]) << 1) & 159)
        | ((buffer4[(buffer1[190] % 21) as usize]) & !159))
        & (((((buffer4[(buffer3[64] % 21) as usize]) & 110)
            | ((buffer0[(buffer1[25] % 20) as usize]) & !110))
            & !150)
            | ((buffer1[25]) & 150));

    buffer2[14] -= (((buffer2[(buffer3[20] % 35) as usize])
        & ((buffer3[72]) ^ (buffer2[(buffer1[100] % 35) as usize])))
        & !34)
        | ((buffer1[97]) & 34);

    buffer0[17] = 115;

    buffer1[23] ^= ((((((buffer4[(buffer1[17] % 21) as usize])
        | (buffer0[(buffer3[20] % 20) as usize]))
        & (buffer3[72]))
        | ((buffer4[(buffer1[17] % 21) as usize]) & (buffer0[(buffer3[20] % 20) as usize])))
        & ((buffer1[50]) / 3))
        | (((((buffer4[(buffer1[17] % 21) as usize]) | (buffer0[(buffer3[20] % 20) as usize]))
            & (buffer3[72]))
            | ((buffer4[(buffer1[17] % 21) as usize]) & buffer0[(buffer3[20] % 20) as usize])
            | ((buffer1[50]) / 3))
            & 246))
        << 1;

    buffer0[13] = ((((((buffer0[(buffer3[40] % 20) as usize]) | (buffer1[10])) & 82)
        | ((buffer0[(buffer3[40] % 20) as usize]) & (buffer1[10])))
        & 209)
        | (((buffer0[(buffer1[39] % 20) as usize]) << 1) & 46))
        >> 1;

    buffer2[33] -= buffer1[113] & 9;

    buffer2[28] -= (((2 | (buffer1[110] & 222)) >> 1) & !223) | (buffer3[20] & 223);

    J = weird_rol8((V | Z) as isize, U as isize & 7); // OK
    A = (((buffer2[16]) & T) | (W & (!(buffer2[16])))) as isize;
    B = (((buffer1[33]) & 17) | (X & !17)) as isize;

    E = ((Y as isize | ((A + B) / 5)) & 147) | (Y as isize & ((A + B) / 5)); // OK
    M = ((buffer3[40]) & (buffer4[(((buffer3[8]) + (J + E) as u8) % 21) as usize]))
        | (((buffer3[40]) | (buffer4[(((buffer3[8]) + (J + E) as u8) % 21) as usize]))
            & (buffer2[23]));

    buffer0[15] = ((((buffer4[(buffer3[20] % 21) as usize]) - 48) & (!(buffer1[184])))
        | (((buffer4[(buffer3[20] % 21) as usize]) - 48) & 189)
        | (189 & !(buffer1[184])))
        & (M * M * M);

    buffer2[22] += buffer1[183];

    buffer3[76] = (3 * buffer4[(buffer1[1] % 21) as usize]) ^ buffer3[0];

    A = buffer2[(((buffer3[8]) + (J + E) as u8) % 35) as usize] as isize;
    F = ((((buffer4[(buffer1[178] % 21) as usize] as isize) & A)
        | (((buffer4[(buffer1[178] % 21) as usize] as isize) | A) & 209))
        * (buffer0[(buffer1[13] % 20) as usize]) as isize)
        * ((buffer4[(buffer1[26] % 21) as usize] as isize) >> 1);
    G = (F + 0x733ffff9) * 198 - (((F + 0x733ffff9) * 396 + 212) & 212) + 85;

    buffer3[80] = ((buffer3[36] as isize) + (G ^ 148) + ((G ^ 107) << 1) - 127) as u8;

    buffer3[84] = ((buffer2[(buffer3[64] % 35) as usize]) & 245)
        | ((buffer2[(buffer3[20] % 35) as usize]) & 10);

    A = (buffer0[(buffer3[68] % 20) as usize]) as isize | 81;
    buffer2[18] -= (((A * A * A) & !buffer0[15] as isize)
        | (((buffer3[80] as isize) / 15) & (buffer0[15] as isize))) as u8;

    buffer3[88] = ((buffer3[8] as isize) + J + E - (buffer0[(buffer1[160] % 20) as usize] as isize)
        + ((buffer4[(buffer0[((buffer3[8] + (J + E) as u8) % 20) as usize] % 21) as usize]
            as isize)
            / 3)) as u8;

    B = (((R ^ (buffer3[72])) & !198) | ((S * S) & 198)) as isize;
    F = ((buffer4[(buffer1[69] % 21) as usize] as isize) & (buffer1[172]) as isize)
        | (((buffer4[(buffer1[69] % 21) as usize] as isize) | (buffer1[172] as isize))
            & (((buffer3[12] as isize) - B) + 77));
    buffer0[16] = (147
        - (((buffer3[72] as isize) & ((F & 251) | 1))
            | (((F & 250) | (buffer3[72] as isize)) & 198))) as u8;

    C = (((buffer4[(buffer1[168] % 21) as usize]) & buffer0[(buffer1[29] % 20) as usize] & 7)
        | ((buffer4[(buffer1[168] % 21) as usize] | buffer0[(buffer1[29] % 20) as usize]) & 6))
        as isize;
    F = (((buffer4[(buffer1[155] % 21) as usize]) & (buffer1[105]))
        | (((buffer4[(buffer1[155] % 21) as usize]) | (buffer1[105])) & 141)) as isize;
    buffer0[3] -= buffer4[(weird_rol32(F, C) % 21) as usize];

    buffer1[5] = weird_ror8(
        buffer0[12] as isize,
        ((buffer0[(buffer1[61] % 20) as usize]) / 5) as isize & 7,
    ) as u8
        ^ (((!buffer2[(buffer3[84] % 35) as usize]) as i64 & 0xffffffff) / 5) as u8;

    buffer1[198] += buffer1[3];

    A = 162 | (buffer2[(buffer3[64] % 35) as usize]) as isize;
    buffer1[164] += ((A * A) / 5) as u8;

    G = weird_ror8(139, buffer3[80] as isize & 7);
    C = ((((buffer4[(buffer3[64] % 21) as usize])
        * (buffer4[(buffer3[64] % 21) as usize])
        * (buffer4[(buffer3[64] % 21) as usize]))
        & 95)
        | ((buffer0[(buffer3[40] % 20) as usize]) & !95)) as isize;
    buffer3[92] = (G as u8 & 12)
        | ((buffer0[(buffer3[20] % 20) as usize]) & 12)
        | (G as u8 & (buffer0[(buffer3[20] % 20) as usize]))
        | C as u8;

    buffer2[12] += (((buffer1[103]) & 32) | ((buffer3[92]) & ((buffer1[103]) | 60)) | 16) / 3;

    buffer3[96] = buffer1[143];

    buffer3[100] = 27;

    buffer3[104] =
        ((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8]))) & (buffer3[64])) ^ 119;

    buffer3[108] = 238
        & (((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8]))) & (buffer3[64]))
            << 1);

    buffer3[112] = (!(buffer3[64]) & ((buffer3[84]) / 3)) ^ 49;

    buffer3[116] = 98 & ((!(buffer3[64]) & ((buffer3[84]) / 3)) << 1);

    // finale
    A = (((buffer1[35]) & (buffer2[8])) | ((buffer3[40]) & !(buffer2[8]))) as isize;
    B = (A & buffer3[64] as isize) | (((buffer3[84]) as isize / 3) & !(buffer3[64] as isize));
    buffer1[143] = (buffer3[96])
        - ((B as u8 & (86 + (((buffer1[172]) & 64) >> 1)))
            | ((((((buffer1[172]) & 65) >> 1) ^ 86)
                | ((!(buffer3[64]) & ((buffer3[84]) / 3))
                    | ((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8])))
                        & (buffer3[64]))))
                & (buffer3[100])));

    buffer2[29] = 162;

    A = ((((buffer4[(buffer3[88] % 21) as usize]) & 160)
        | ((buffer0[(buffer1[125] % 20) as usize]) & 95))
        >> 1) as isize;
    B = ((buffer2[(buffer1[149] % 35) as usize]) ^ ((buffer1[43]) * (buffer1[43]))) as isize;

    buffer0[15] += ((B & A) | ((A | B) & 115)) as u8;

    buffer3[120] = (buffer3[64]) - (buffer0[(buffer3[40] % 20) as usize]);

    buffer1[95] = buffer4[(buffer3[20] % 21) as usize];

    A = weird_ror8(
        buffer2[(buffer3[80] % 35) as usize] as isize,
        (((buffer2[(buffer1[17] % 35) as usize])
            * (buffer2[(buffer1[17] % 35) as usize])
            * (buffer2[(buffer1[17] % 35) as usize]))
            & 7) as isize,
    );

    buffer0[7] -= (A * A) as u8;

    buffer2[8] = (buffer2[8]) - (buffer1[184])
        + ((buffer4[(buffer1[202] % 21) as usize])
            * (buffer4[(buffer1[202] % 21) as usize])
            * (buffer4[(buffer1[202] % 21) as usize]));

    buffer0[16] = ((buffer2[(buffer1[102] % 35) as usize]) << 1) & 132;

    buffer3[124] = ((buffer4[(buffer3[40] % 21) as usize]) >> 1) ^ (buffer3[68]);

    buffer0[7] -= (buffer0[(buffer1[191] % 20) as usize])
        - ((((buffer4[(buffer1[80] % 21) as usize]) << 1) & !177)
            | ((buffer4[(buffer4[(buffer3[88] % 21) as usize] % 21) as usize]) & 177));

    buffer0[6] = buffer0[(buffer1[119] % 20) as usize];

    A = ((buffer4[(buffer1[190] % 21) as usize] & !209) | (buffer1[118] & 209)) as isize;
    B = (buffer0[(buffer3[120] % 20) as usize] * buffer0[(buffer3[120] % 20) as usize]) as isize;
    buffer0[12] = (buffer0[(buffer3[84] % 20) as usize]
        ^ (buffer2[(buffer1[71] % 35) as usize] + buffer2[(buffer1[15] % 35) as usize]))
        & ((A & B) | ((A | B) & 27)) as u8;

    B = (((buffer1[32]) & (buffer2[(buffer3[88] % 35) as usize]))
        | (((buffer1[32]) | (buffer2[(buffer3[88] % 35) as usize])) & 23)) as isize;
    D = (((buffer4[(buffer1[57] % 21) as usize] as isize) * 231) & 169) | (B & 86);
    F = ((((buffer0[(buffer1[82] % 20) as usize] as isize) & !29)
        | ((buffer4[(buffer3[124] % 21) as usize] as isize) & 29))
        & 190)
        | ((buffer4[(D / 5 % 21) as usize] as isize) & !190);
    H = (buffer0[(buffer3[40] % 20) as usize])
        * (buffer0[(buffer3[40] % 20) as usize])
        * (buffer0[(buffer3[40] % 20) as usize]);
    K = (H & (buffer1[82])) | (H & 92) | ((buffer1[82]) & 92);
    buffer3[128] = ((F as u8 & K) | ((F as u8 | K) & 192)) ^ (D as u8 / 5);

    buffer2[25] ^= (((buffer0[(buffer3[120] % 20) as usize]) << 1) * (buffer1[5]))
        - (weird_rol8(
            buffer3[76] as isize,
            (buffer4[(buffer3[124] % 21) as usize]) as isize & 7,
        ) as u8
            & ((buffer3[20]) + 110));
}

fn rol8(input: isize, count: isize) -> isize {
    (((input << count) & 0xff) | (input & 0xff) >> (8 - count)) & 0xff
}

fn rol8x(input: isize, count: isize) -> isize {
    (input << count) | (input) >> (8 - count)
}

fn weird_ror8(input: isize, count: isize) -> isize {
    if count == 0 {
        0
    } else {
        ((input >> count) & 0xff) | ((input & 0xff) << (8 - count))
    }
}

fn weird_rol8(input: isize, count: isize) -> isize {
    if count == 0 {
        0
    } else {
        ((input << count) & 0xff) | ((input & 0xff) >> (8 - count))
    }
}

fn weird_rol32(input: isize, count: isize) -> isize {
    if count == 0 {
        0
    } else {
        (input << count) ^ (input >> (8 - count))
    }
}

#[cfg(test)]
mod tests {
    use crate::airplay::lib::hand_garble::garble;

    #[test]
    fn garble1_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            156, 200, 104, 182, 128, 138, 90, 187, 39, 241, 184, 54, 10, 85, 216, 121, 37, 226,
            178, 90, 61, 237, 138, 181, 101, 144, 255, 178, 178, 247, 191, 109, 174, 108, 232, 33,
            92, 45, 232, 98, 95, 13, 152, 231, 3, 33, 20, 26, 92, 154, 133, 221, 87, 75, 249, 142,
            68, 95, 134, 159, 229, 153, 143, 183, 246, 206, 167, 5, 49, 71, 67, 38, 41, 98, 200, 8,
            231, 0, 42, 220, 102, 67, 216, 125, 197, 51, 66, 87, 39, 116, 252, 26, 114, 123, 152,
            84, 75, 71, 74, 74, 161, 12, 10, 214, 53, 253, 51, 134, 161, 236, 121, 70, 37, 42, 189,
            31, 230, 115, 190, 15, 75, 76, 24, 29, 18, 172, 8, 253, 92, 115, 255, 104, 253, 101,
            49, 181, 20, 221, 125, 115, 10, 145, 122, 46, 255, 6, 24, 76, 240, 166, 40, 79, 205,
            43, 29, 80, 190, 211, 32, 249, 85, 254, 107, 107, 96, 169, 32, 161, 52, 175, 65, 88,
            129, 42, 101, 110, 198, 29, 94, 191, 104, 169, 232, 191, 140, 7, 59, 52, 219, 117, 15,
            1, 42, 93, 128, 153, 0, 138, 22, 18, 41, 162, 194, 158, 93, 9, 127, 178, 140, 12,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);
        assert_eq!(
            [
                150, 95, 198, 22, 248, 70, 63, 140, 223, 190, 0, 248, 2, 72, 168, 63, 132, 115, 32,
                53
            ],
            buf0
        );

        assert_eq!(
            [
                156, 200, 40, 182, 136, 242, 90, 187, 39, 241, 184, 54, 10, 85, 216, 55, 37, 226,
                178, 119, 61, 237, 87, 201, 101, 144, 255, 178, 178, 247, 191, 109, 174, 202, 232,
                33, 217, 45, 232, 98, 95, 13, 152, 231, 3, 33, 20, 186, 92, 154, 133, 221, 87, 214,
                249, 142, 68, 95, 134, 159, 229, 153, 143, 183, 246, 206, 167, 5, 49, 71, 67, 38,
                41, 98, 200, 8, 231, 0, 42, 220, 102, 67, 216, 125, 197, 51, 66, 87, 39, 116, 252,
                26, 114, 123, 152, 82, 75, 71, 74, 74, 161, 12, 10, 214, 53, 253, 142, 134, 161,
                236, 121, 70, 37, 42, 189, 31, 230, 115, 190, 15, 75, 76, 24, 192, 165, 172, 8,
                253, 92, 115, 255, 104, 253, 101, 49, 181, 20, 221, 125, 115, 33, 145, 122, 28,
                255, 6, 24, 76, 240, 166, 40, 79, 205, 172, 29, 80, 190, 211, 32, 249, 85, 254,
                107, 107, 199, 169, 32, 161, 52, 175, 65, 88, 129, 42, 101, 110, 198, 29, 94, 191,
                104, 169, 232, 191, 140, 7, 59, 52, 219, 117, 56, 1, 42, 93, 128, 153, 0, 138, 204,
                18, 41, 162, 194, 158, 93, 9, 127, 178, 140, 12
            ],
            buf1
        );
        assert_eq!(
            [
                67, 44, 162, 122, 88, 26, 214, 179, 207, 86, 246, 28, 1, 220, 185, 29, 105, 54,
                195, 177, 57, 81, 194, 170, 9, 174, 254, 68, 119, 162, 117, 32, 157, 181, 184
            ],
            buf2
        );
        assert_eq!(
            [
                164, 0, 0, 0, 217, 0, 0, 0, 76, 0, 0, 0, 202, 0, 0, 0, 146, 0, 0, 0, 56, 0, 0, 0,
                79, 0, 0, 0, 167, 0, 0, 0, 229, 0, 0, 0, 180, 0, 0, 0, 34, 0, 0, 0, 51, 0, 0, 0,
                19, 0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 204, 0, 0, 0, 222, 0, 0, 0, 10,
                0, 0, 0, 164, 0, 0, 0, 8, 0, 0, 0, 212, 0, 0, 0, 68, 0, 0, 0, 160, 0, 0, 0, 46, 0,
                0, 0, 27, 0, 0, 0, 119, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0,
                135, 0, 0, 0, 204, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble2_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            112, 88, 7, 189, 224, 156, 60, 123, 121, 75, 1, 97, 27, 243, 100, 229, 175, 8, 16, 38,
            213, 28, 64, 77, 21, 210, 234, 186, 255, 217, 184, 13, 155, 169, 53, 235, 199, 180,
            147, 117, 126, 185, 167, 183, 120, 10, 189, 105, 147, 229, 164, 190, 106, 76, 89, 111,
            26, 66, 5, 152, 210, 137, 157, 202, 199, 246, 80, 162, 174, 49, 50, 159, 0, 68, 222,
            98, 153, 191, 216, 95, 62, 105, 27, 100, 194, 5, 126, 172, 114, 159, 160, 104, 74, 16,
            192, 110, 122, 247, 63, 7, 207, 34, 14, 141, 30, 77, 162, 236, 85, 56, 118, 125, 181,
            176, 121, 166, 42, 198, 105, 97, 233, 203, 232, 175, 115, 94, 54, 91, 61, 126, 129,
            164, 30, 120, 117, 165, 234, 158, 206, 70, 157, 77, 178, 173, 4, 3, 123, 150, 101, 65,
            176, 127, 159, 48, 213, 29, 168, 25, 201, 70, 76, 70, 252, 175, 194, 193, 137, 6, 188,
            99, 241, 45, 189, 12, 111, 111, 148, 3, 209, 217, 137, 53, 107, 173, 83, 247, 10, 49,
            213, 112, 248, 242, 8, 108, 36, 111, 87, 180, 64, 93, 198, 250, 165, 17, 31, 245, 12,
            17, 108, 55,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 86, 248, 70, 115, 135, 223, 190, 13, 248, 66, 27, 93, 235, 128, 115,
                32, 203
            ],
            buf0
        );
        assert_eq!(
            [
                112, 88, 199, 189, 2, 81, 60, 123, 121, 75, 1, 97, 27, 243, 100, 113, 175, 8, 16,
                75, 213, 28, 93, 33, 21, 210, 234, 186, 255, 217, 184, 13, 155, 153, 90, 235, 68,
                180, 147, 117, 126, 185, 167, 183, 120, 10, 189, 140, 147, 229, 164, 190, 106, 211,
                89, 111, 26, 66, 5, 152, 210, 137, 157, 202, 199, 246, 80, 162, 174, 49, 50, 159,
                0, 68, 222, 98, 153, 191, 216, 95, 62, 105, 27, 100, 194, 5, 126, 172, 114, 159,
                160, 104, 74, 16, 192, 192, 122, 247, 63, 7, 207, 34, 14, 141, 30, 77, 95, 236, 85,
                56, 118, 125, 181, 176, 121, 166, 42, 198, 105, 97, 233, 203, 232, 114, 181, 94,
                54, 91, 61, 126, 129, 164, 30, 120, 117, 165, 234, 158, 206, 70, 98, 77, 178, 82,
                4, 3, 123, 150, 101, 65, 176, 127, 159, 208, 213, 29, 168, 25, 201, 70, 76, 70,
                252, 175, 201, 193, 137, 6, 188, 99, 241, 45, 189, 12, 111, 111, 148, 3, 209, 217,
                137, 53, 107, 173, 83, 247, 10, 49, 213, 112, 56, 242, 8, 108, 36, 111, 87, 180,
                253, 93, 198, 250, 165, 17, 31, 245, 12, 17, 108, 55
            ],
            buf1
        );
        assert_eq!(
            [
                67, 92, 106, 122, 47, 195, 214, 179, 9, 86, 246, 28, 36, 210, 213, 29, 36, 54, 54,
                177, 57, 81, 175, 170, 9, 8, 254, 68, 55, 162, 216, 32, 157, 168, 184
            ],
            buf2
        );
        assert_eq!(
            [
                63, 0, 0, 0, 79, 0, 0, 0, 68, 0, 0, 0, 194, 0, 0, 0, 82, 0, 0, 0, 120, 0, 0, 0,
                127, 0, 0, 0, 31, 0, 0, 0, 93, 0, 0, 0, 74, 0, 0, 0, 123, 0, 0, 0, 92, 0, 0, 0, 18,
                0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 184, 0, 0, 0, 8, 0, 0,
                0, 11, 0, 0, 0, 28, 0, 0, 0, 184, 0, 0, 0, 111, 0, 0, 0, 223, 0, 0, 0, 173, 0, 0,
                0, 27, 0, 0, 0, 54, 0, 0, 0, 130, 0, 0, 0, 9, 0, 0, 0, 96, 0, 0, 0, 239, 0, 0, 0,
                181, 0, 0, 0, 246, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble3_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            207, 230, 139, 151, 25, 172, 159, 49, 105, 86, 14, 177, 150, 128, 158, 93, 88, 84, 234,
            5, 130, 231, 202, 68, 248, 31, 16, 70, 76, 73, 177, 175, 89, 20, 160, 193, 181, 251,
            54, 23, 96, 44, 152, 190, 36, 255, 154, 144, 79, 223, 243, 105, 149, 208, 48, 202, 17,
            211, 63, 167, 82, 107, 158, 47, 190, 176, 117, 247, 136, 43, 38, 25, 125, 136, 222,
            218, 163, 53, 98, 37, 180, 229, 71, 212, 202, 133, 10, 21, 214, 200, 163, 246, 60, 50,
            244, 20, 18, 30, 119, 116, 217, 155, 204, 8, 37, 93, 72, 55, 52, 61, 220, 252, 237,
            248, 4, 81, 34, 235, 192, 160, 3, 28, 210, 28, 169, 219, 175, 27, 79, 180, 175, 52,
            250, 90, 3, 138, 130, 161, 11, 200, 126, 88, 230, 235, 12, 41, 160, 23, 202, 76, 196,
            161, 33, 189, 223, 247, 143, 191, 222, 231, 9, 141, 137, 153, 47, 60, 107, 208, 181,
            160, 13, 24, 27, 50, 43, 244, 218, 209, 75, 224, 204, 117, 21, 37, 106, 178, 235, 104,
            120, 93, 38, 226, 129, 44, 27, 193, 248, 137, 64, 3, 37, 94, 62, 39, 108, 8, 35, 12,
            46, 94,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 22, 248, 70, 150, 229, 223, 190, 6, 248, 2, 3, 204, 250, 4, 115, 32,
                212
            ],
            buf0
        );
        assert_eq!(
            [
                207, 230, 75, 151, 34, 162, 159, 49, 105, 86, 14, 177, 150, 128, 158, 166, 88, 84,
                234, 9, 130, 231, 78, 238, 248, 31, 16, 70, 76, 73, 177, 175, 89, 98, 193, 193, 50,
                251, 54, 23, 96, 44, 152, 190, 36, 255, 154, 84, 79, 223, 243, 105, 149, 205, 48,
                202, 17, 211, 63, 167, 82, 107, 158, 47, 190, 176, 117, 247, 136, 43, 38, 25, 125,
                136, 222, 218, 163, 53, 98, 37, 180, 229, 71, 212, 202, 133, 10, 21, 214, 200, 163,
                246, 60, 50, 244, 188, 18, 30, 119, 116, 217, 155, 204, 8, 37, 93, 149, 55, 52, 61,
                220, 252, 237, 248, 4, 81, 34, 235, 192, 160, 3, 28, 210, 193, 181, 219, 175, 27,
                79, 180, 175, 52, 250, 90, 3, 138, 130, 161, 11, 200, 67, 88, 230, 152, 12, 41,
                160, 23, 202, 76, 196, 161, 33, 61, 223, 247, 143, 191, 222, 231, 9, 141, 137, 153,
                7, 60, 107, 208, 181, 160, 13, 24, 27, 50, 43, 244, 218, 209, 75, 224, 204, 117,
                21, 37, 106, 178, 235, 104, 120, 93, 56, 226, 129, 44, 27, 193, 248, 137, 215, 3,
                37, 94, 62, 39, 108, 8, 35, 12, 46, 94
            ],
            buf1
        );
        assert_eq!(
            [
                67, 172, 128, 122, 12, 168, 214, 179, 26, 86, 246, 28, 28, 132, 62, 29, 234, 54,
                133, 177, 57, 81, 129, 170, 9, 108, 254, 68, 55, 162, 157, 32, 157, 74, 184
            ],
            buf2
        );
        assert_eq!(
            [
                171, 0, 0, 0, 72, 0, 0, 0, 76, 0, 0, 0, 241, 0, 0, 0, 114, 0, 0, 0, 88, 0, 0, 0,
                161, 0, 0, 0, 255, 0, 0, 0, 61, 0, 0, 0, 84, 0, 0, 0, 38, 0, 0, 0, 110, 0, 0, 0, 2,
                0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 234, 0, 0, 0, 25, 0, 0, 0, 93, 0,
                0, 0, 254, 0, 0, 0, 84, 0, 0, 0, 9, 0, 0, 0, 49, 0, 0, 0, 255, 0, 0, 0, 235, 0, 0,
                0, 27, 0, 0, 0, 181, 0, 0, 0, 132, 0, 0, 0, 48, 0, 0, 0, 2, 0, 0, 0, 202, 0, 0, 0,
                104, 0, 0, 0, 207, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble4_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            17, 161, 116, 127, 254, 24, 42, 160, 165, 53, 135, 71, 149, 24, 71, 117, 246, 112, 14,
            154, 42, 248, 85, 95, 232, 26, 14, 70, 251, 113, 12, 198, 182, 150, 143, 88, 191, 13,
            244, 39, 182, 123, 243, 9, 63, 246, 47, 133, 246, 251, 252, 158, 83, 114, 235, 6, 1,
            137, 34, 197, 183, 198, 98, 237, 31, 218, 5, 112, 45, 40, 179, 225, 85, 48, 17, 107,
            210, 109, 44, 119, 81, 42, 23, 228, 109, 187, 15, 212, 144, 143, 250, 145, 70, 208, 4,
            255, 51, 227, 17, 127, 226, 110, 53, 167, 170, 2, 214, 196, 177, 32, 169, 157, 209,
            190, 144, 63, 28, 210, 155, 166, 186, 96, 231, 84, 84, 115, 130, 59, 182, 74, 6, 180,
            25, 106, 98, 144, 47, 97, 213, 218, 141, 197, 170, 143, 206, 163, 2, 182, 6, 28, 239,
            233, 251, 198, 185, 199, 244, 149, 233, 224, 98, 181, 213, 4, 248, 139, 175, 249, 176,
            198, 215, 100, 96, 213, 81, 139, 255, 70, 35, 157, 143, 17, 10, 90, 254, 24, 38, 187,
            114, 6, 171, 131, 95, 25, 229, 219, 201, 89, 186, 19, 7, 131, 12, 196, 152, 123, 187,
            232, 138, 162,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 87, 248, 70, 204, 166, 223, 190, 10, 248, 1, 19, 58, 210, 0, 115, 32,
                251
            ],
            buf0
        );
        assert_eq!(
            [
                17, 161, 52, 127, 226, 252, 42, 160, 165, 53, 135, 71, 149, 24, 71, 23, 246, 112,
                14, 154, 42, 248, 71, 243, 232, 26, 14, 70, 251, 113, 12, 198, 182, 198, 148, 88,
                60, 13, 244, 39, 182, 123, 243, 9, 63, 246, 47, 131, 246, 251, 252, 158, 83, 211,
                235, 6, 1, 137, 34, 197, 183, 198, 98, 237, 31, 218, 5, 112, 45, 40, 179, 225, 85,
                48, 17, 107, 210, 109, 44, 119, 81, 42, 23, 228, 109, 187, 15, 212, 144, 143, 250,
                145, 70, 208, 4, 2, 51, 227, 17, 127, 226, 110, 53, 167, 170, 2, 79, 196, 177, 32,
                169, 157, 209, 190, 144, 63, 28, 210, 155, 166, 186, 96, 231, 137, 229, 115, 130,
                59, 182, 74, 6, 180, 25, 106, 98, 144, 47, 97, 213, 218, 141, 197, 170, 121, 206,
                163, 2, 182, 6, 28, 239, 233, 251, 77, 185, 199, 244, 149, 233, 224, 98, 181, 213,
                4, 140, 139, 175, 249, 176, 198, 215, 100, 96, 213, 81, 139, 255, 70, 35, 157, 143,
                17, 10, 90, 254, 24, 38, 187, 114, 6, 56, 131, 95, 25, 229, 219, 201, 89, 57, 19,
                7, 131, 12, 196, 152, 123, 187, 232, 138, 162
            ],
            buf1
        );
        assert_eq!(
            [
                67, 204, 124, 122, 28, 193, 214, 179, 107, 86, 246, 28, 144, 26, 225, 29, 126, 54,
                153, 177, 57, 81, 111, 170, 9, 224, 254, 68, 147, 162, 123, 32, 157, 68, 184
            ],
            buf2
        );
        assert_eq!(
            [
                169, 0, 0, 0, 165, 0, 0, 0, 8, 0, 0, 0, 146, 0, 0, 0, 174, 0, 0, 0, 28, 0, 0, 0,
                233, 0, 0, 0, 30, 0, 0, 0, 92, 0, 0, 0, 164, 0, 0, 0, 210, 0, 0, 0, 119, 0, 0, 0,
                26, 0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 163, 0, 0, 0, 170, 0, 0, 0, 23,
                0, 0, 0, 95, 0, 0, 0, 244, 0, 0, 0, 162, 0, 0, 0, 147, 0, 0, 0, 157, 0, 0, 0, 143,
                0, 0, 0, 27, 0, 0, 0, 117, 0, 0, 0, 4, 0, 0, 0, 37, 0, 0, 0, 32, 0, 0, 0, 153, 0,
                0, 0, 220, 0, 0, 0, 84, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble5_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            38, 22, 138, 252, 34, 158, 186, 24, 144, 126, 50, 254, 250, 121, 231, 119, 89, 243,
            204, 10, 48, 90, 190, 38, 213, 204, 45, 123, 188, 48, 73, 150, 36, 12, 28, 25, 126,
            240, 29, 243, 190, 130, 197, 119, 3, 50, 109, 134, 112, 30, 89, 169, 134, 173, 83, 87,
            154, 169, 40, 235, 151, 98, 1, 141, 72, 56, 11, 95, 132, 125, 150, 80, 176, 108, 163,
            250, 26, 3, 132, 228, 29, 77, 186, 142, 82, 202, 215, 64, 30, 252, 124, 198, 92, 153,
            207, 4, 96, 26, 231, 108, 36, 37, 243, 39, 21, 146, 202, 184, 121, 21, 68, 26, 191, 71,
            235, 44, 206, 84, 221, 120, 117, 63, 249, 135, 84, 35, 252, 242, 245, 59, 61, 207, 120,
            16, 193, 144, 119, 157, 17, 234, 232, 124, 186, 65, 210, 90, 96, 148, 84, 143, 116,
            171, 222, 13, 90, 179, 159, 155, 125, 172, 204, 163, 95, 103, 240, 253, 111, 223, 99,
            220, 189, 13, 3, 193, 142, 103, 81, 145, 152, 211, 59, 41, 191, 229, 158, 80, 204, 225,
            113, 64, 62, 71, 70, 98, 130, 22, 86, 9, 192, 99, 204, 104, 174, 246, 191, 34, 63, 218,
            101, 222,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 80, 248, 70, 150, 133, 223, 190, 13, 248, 147, 11, 50, 116, 132, 115,
                32, 209
            ],
            buf0
        );
        assert_eq!(
            [
                38, 22, 74, 252, 114, 241, 186, 24, 144, 126, 50, 254, 250, 121, 231, 185, 89, 243,
                204, 15, 48, 90, 85, 14, 213, 204, 45, 123, 188, 48, 73, 150, 36, 40, 69, 25, 251,
                240, 29, 243, 190, 130, 197, 119, 3, 50, 109, 57, 112, 30, 89, 169, 134, 214, 83,
                87, 154, 169, 40, 235, 151, 98, 1, 141, 72, 56, 11, 95, 132, 125, 150, 80, 176,
                108, 163, 250, 26, 3, 132, 228, 29, 77, 186, 142, 82, 202, 215, 64, 30, 252, 124,
                198, 92, 153, 207, 82, 96, 26, 231, 108, 36, 37, 243, 39, 21, 146, 119, 184, 121,
                21, 68, 26, 191, 71, 235, 44, 206, 84, 221, 120, 117, 63, 249, 90, 181, 35, 252,
                242, 245, 59, 61, 207, 120, 16, 193, 144, 119, 157, 17, 234, 89, 124, 186, 42, 210,
                90, 96, 148, 84, 143, 116, 171, 222, 237, 90, 179, 159, 155, 125, 172, 204, 163,
                95, 103, 248, 253, 111, 223, 99, 220, 189, 13, 3, 193, 142, 103, 81, 145, 152, 211,
                59, 41, 191, 229, 158, 80, 204, 225, 113, 64, 56, 71, 70, 98, 130, 22, 86, 9, 188,
                99, 204, 104, 174, 246, 191, 34, 63, 218, 101, 222
            ],
            buf1
        );
        assert_eq!(
            [
                67, 9, 166, 122, 34, 33, 214, 179, 1, 86, 246, 28, 231, 157, 9, 29, 85, 54, 17,
                177, 57, 81, 153, 170, 9, 82, 254, 68, 119, 162, 184, 32, 157, 110, 184
            ],
            buf2
        );
        assert_eq!(
            [
                8, 0, 0, 0, 170, 0, 0, 0, 76, 0, 0, 0, 204, 0, 0, 0, 146, 0, 0, 0, 56, 0, 0, 0,
                171, 0, 0, 0, 223, 0, 0, 0, 29, 0, 0, 0, 46, 0, 0, 0, 53, 0, 0, 0, 22, 0, 0, 0, 3,
                0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 179, 0, 0, 0, 0, 0, 0,
                0, 103, 0, 0, 0, 128, 0, 0, 0, 177, 0, 0, 0, 156, 0, 0, 0, 72, 0, 0, 0, 65, 0, 0,
                0, 27, 0, 0, 0, 114, 0, 0, 0, 10, 0, 0, 0, 3, 0, 0, 0, 96, 0, 0, 0, 66, 0, 0, 0,
                179, 0, 0, 0, 136, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble6_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            34, 126, 12, 15, 143, 98, 247, 151, 124, 241, 169, 97, 160, 44, 109, 181, 228, 143,
            161, 17, 75, 110, 227, 183, 200, 178, 158, 194, 31, 202, 5, 253, 228, 42, 215, 244, 65,
            20, 116, 121, 1, 27, 231, 137, 153, 184, 242, 113, 218, 190, 138, 17, 22, 80, 179, 131,
            159, 128, 202, 201, 87, 190, 168, 177, 184, 237, 192, 65, 152, 114, 129, 14, 191, 28,
            150, 165, 103, 154, 152, 235, 68, 131, 79, 201, 220, 197, 250, 160, 249, 148, 243, 184,
            95, 250, 34, 172, 205, 69, 68, 228, 122, 207, 59, 239, 136, 197, 69, 113, 3, 122, 238,
            231, 22, 222, 170, 100, 69, 39, 113, 141, 214, 174, 189, 153, 57, 22, 165, 170, 82,
            174, 170, 216, 145, 32, 110, 154, 48, 161, 203, 173, 36, 170, 79, 247, 116, 192, 222,
            102, 227, 251, 212, 153, 44, 143, 203, 160, 44, 237, 196, 86, 176, 227, 99, 84, 189,
            128, 209, 125, 210, 158, 26, 231, 254, 82, 238, 214, 62, 150, 13, 73, 242, 25, 215, 63,
            178, 69, 222, 140, 20, 140, 193, 199, 37, 74, 33, 9, 61, 155, 49, 113, 59, 85, 47, 180,
            254, 4, 144, 153, 24, 229,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 186, 248, 70, 95, 107, 223, 190, 12, 248, 65, 31, 93, 115, 0, 115,
                32, 140
            ],
            buf0
        );
        assert_eq!(
            [
                34, 126, 204, 15, 210, 0, 247, 151, 124, 241, 169, 97, 160, 44, 109, 75, 228, 143,
                161, 54, 75, 110, 92, 123, 200, 178, 158, 194, 31, 202, 5, 253, 228, 90, 215, 244,
                190, 20, 116, 121, 1, 27, 231, 137, 153, 184, 242, 141, 218, 190, 138, 17, 22, 211,
                179, 131, 159, 128, 202, 201, 87, 190, 168, 177, 184, 237, 192, 65, 152, 114, 129,
                14, 191, 28, 150, 165, 103, 154, 152, 235, 68, 131, 79, 201, 220, 197, 250, 160,
                249, 148, 243, 184, 95, 250, 34, 82, 205, 69, 68, 228, 122, 207, 59, 239, 136, 197,
                248, 113, 3, 122, 238, 231, 22, 222, 170, 100, 69, 39, 113, 141, 214, 174, 189, 68,
                189, 22, 165, 170, 82, 174, 170, 216, 145, 32, 110, 154, 48, 161, 203, 173, 59,
                170, 79, 160, 116, 192, 222, 102, 227, 251, 212, 153, 44, 198, 203, 160, 44, 237,
                196, 86, 176, 227, 99, 84, 245, 128, 209, 125, 210, 158, 26, 231, 254, 82, 238,
                214, 62, 150, 13, 73, 242, 25, 215, 63, 178, 69, 222, 140, 20, 140, 56, 199, 37,
                74, 33, 9, 61, 155, 64, 113, 59, 85, 47, 180, 254, 4, 144, 153, 24, 229
            ],
            buf1
        );
        assert_eq!(
            [
                67, 60, 100, 122, 23, 195, 214, 179, 164, 86, 246, 28, 47, 61, 188, 29, 237, 54,
                198, 177, 57, 81, 157, 170, 9, 94, 254, 68, 119, 162, 143, 32, 157, 212, 184
            ],
            buf2
        );
        assert_eq!(
            [
                217, 0, 0, 0, 116, 0, 0, 0, 76, 0, 0, 0, 54, 0, 0, 0, 146, 0, 0, 0, 56, 0, 0, 0,
                153, 0, 0, 0, 174, 0, 0, 0, 236, 0, 0, 0, 178, 0, 0, 0, 72, 0, 0, 0, 214, 0, 0, 0,
                24, 0, 0, 0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 219, 0, 0, 0, 196,
                0, 0, 0, 30, 0, 0, 0, 178, 0, 0, 0, 209, 0, 0, 0, 0, 0, 0, 0, 236, 0, 0, 0, 247, 0,
                0, 0, 27, 0, 0, 0, 115, 0, 0, 0, 8, 0, 0, 0, 112, 0, 0, 0, 2, 0, 0, 0, 212, 0, 0,
                0, 143, 0, 0, 0, 230, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }

    #[test]
    fn garble7_test() {
        let mut buf0 = [
            150, 95, 198, 83, 248, 70, 204, 24, 223, 190, 178, 248, 56, 215, 236, 34, 3, 209, 32,
            143,
        ];
        let mut buf1 = [
            27, 49, 220, 213, 67, 153, 133, 83, 152, 73, 51, 180, 216, 255, 240, 34, 8, 160, 122,
            19, 29, 145, 49, 98, 105, 26, 42, 58, 156, 28, 18, 166, 152, 245, 240, 219, 2, 143,
            131, 61, 209, 183, 60, 55, 201, 45, 74, 70, 96, 184, 167, 219, 185, 57, 118, 79, 74,
            220, 215, 143, 235, 43, 249, 18, 172, 119, 229, 35, 218, 175, 34, 103, 117, 146, 38,
            127, 126, 201, 33, 24, 195, 77, 154, 198, 58, 168, 100, 32, 44, 66, 164, 124, 230, 138,
            237, 115, 3, 175, 82, 25, 58, 53, 57, 185, 107, 116, 180, 165, 156, 15, 220, 178, 146,
            22, 222, 208, 37, 172, 37, 138, 249, 211, 50, 142, 252, 135, 77, 212, 162, 242, 216,
            115, 21, 12, 37, 68, 202, 110, 91, 167, 197, 76, 34, 188, 110, 190, 206, 228, 186, 104,
            226, 136, 37, 144, 221, 154, 206, 237, 213, 98, 65, 28, 248, 34, 107, 76, 130, 4, 101,
            108, 54, 151, 79, 73, 159, 67, 65, 166, 19, 162, 46, 185, 152, 32, 162, 88, 140, 212,
            62, 96, 227, 240, 28, 21, 164, 117, 54, 51, 3, 158, 201, 173, 148, 52, 67, 0, 253, 63,
            55, 198,
        ];
        let mut buf2 = [
            67, 84, 98, 122, 24, 195, 214, 179, 154, 86, 246, 28, 20, 63, 12, 29, 59, 54, 131, 177,
            57, 81, 74, 170, 9, 62, 254, 68, 175, 222, 195, 32, 157, 66, 58,
        ];
        let mut buf3 = [0; 132];
        let mut buf4 = [
            237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227, 27,
            73, 199,
        ];

        garble(&mut buf0, &mut buf1, &mut buf2, &mut buf3, &mut buf4);

        assert_eq!(
            [
                150, 95, 198, 242, 248, 70, 32, 128, 223, 190, 10, 248, 2, 31, 150, 121, 132, 115,
                32, 35
            ],
            buf0
        );
        assert_eq!(
            [
                27, 49, 156, 213, 50, 14, 133, 83, 152, 73, 51, 180, 216, 255, 240, 10, 8, 160,
                122, 56, 29, 145, 77, 12, 105, 26, 42, 58, 156, 28, 18, 166, 152, 37, 25, 219, 127,
                143, 131, 61, 209, 183, 60, 55, 201, 45, 74, 50, 96, 184, 167, 219, 185, 208, 118,
                79, 74, 220, 215, 143, 235, 43, 249, 18, 172, 119, 229, 35, 218, 175, 34, 103, 117,
                146, 38, 127, 126, 201, 33, 24, 195, 77, 154, 198, 58, 168, 100, 32, 44, 66, 164,
                124, 230, 138, 237, 82, 3, 175, 82, 25, 58, 53, 57, 185, 107, 116, 9, 165, 156, 15,
                220, 178, 146, 22, 222, 208, 37, 172, 37, 138, 249, 211, 50, 83, 165, 135, 77, 212,
                162, 242, 216, 115, 21, 12, 37, 68, 202, 110, 91, 167, 243, 76, 34, 73, 110, 190,
                206, 228, 186, 104, 226, 136, 37, 60, 221, 154, 206, 237, 213, 98, 65, 28, 248, 34,
                210, 76, 130, 4, 101, 108, 54, 151, 79, 73, 159, 67, 65, 166, 19, 162, 46, 185,
                152, 32, 162, 88, 140, 212, 62, 96, 56, 240, 28, 21, 164, 117, 54, 51, 216, 158,
                201, 173, 148, 52, 67, 0, 253, 63, 55, 198
            ],
            buf1
        );
        assert_eq!(
            [
                67, 44, 164, 122, 41, 252, 214, 179, 78, 86, 246, 28, 222, 26, 234, 29, 132, 54,
                53, 177, 57, 81, 82, 170, 9, 152, 254, 68, 119, 162, 246, 32, 157, 104, 184
            ],
            buf2
        );
        assert_eq!(
            [
                85, 0, 0, 0, 86, 0, 0, 0, 8, 0, 0, 0, 178, 0, 0, 0, 146, 0, 0, 0, 56, 0, 0, 0, 136,
                0, 0, 0, 30, 0, 0, 0, 92, 0, 0, 0, 50, 0, 0, 0, 116, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
                0, 27, 0, 0, 0, 199, 0, 0, 0, 0, 0, 0, 0, 250, 0, 0, 0, 252, 0, 0, 0, 4, 0, 0, 0,
                83, 0, 0, 0, 82, 0, 0, 0, 244, 0, 0, 0, 26, 0, 0, 0, 157, 0, 0, 0, 188, 0, 0, 0,
                27, 0, 0, 0, 5, 0, 0, 0, 228, 0, 0, 0, 48, 0, 0, 0, 2, 0, 0, 0, 109, 0, 0, 0, 252,
                0, 0, 0, 204, 0, 0, 0
            ],
            buf3
        );
        assert_eq!(
            [
                237, 37, 209, 187, 188, 39, 159, 2, 162, 169, 17, 0, 12, 179, 82, 192, 189, 227,
                27, 73, 199
            ],
            buf4
        );
    }
}
