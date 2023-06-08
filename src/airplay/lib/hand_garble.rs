pub fn garble(
    buffer0: &mut [u8],
    buffer1: &mut [u8],
    buffer2: &mut [u8],
    buffer3: &mut [u8],
    buffer4: &mut [u8],
) {
    let (mut tmp, mut tmp2, mut tmp3);
    let (
        mut A,
        mut B,
        mut C,
        mut D,
        mut E,
        mut M,
        mut J,
        mut G,
        mut F,
        mut H,
        mut K,
        mut R,
        mut S,
        mut T,
        mut U,
        mut V,
        mut W,
        mut X,
        mut Y,
        mut Z,
    );

    buffer2[12] = 0x14
        + ((((buffer1[64]) & 92) | (((buffer1[99]) / 3) & 35))
            & (buffer4[(rol8x(buffer4[(buffer1[206] % 21) as usize], 4) % 21) as usize]));

    buffer1[4] = (buffer1[99] / 5) * (buffer1[99] / 5) * 2;

    // Simpler still!
    buffer2[34] = 0xb8;

    buffer1[153] ^= buffer2[(buffer1[203] % 35) as usize]
        * buffer2[(buffer1[203] % 35) as usize]
        * buffer1[190];

    // This one looks simple, but wow was it not :(
    buffer0[3] = ((buffer0[3] as i32 - ((buffer4[(buffer1[205] % 21) as usize] >> 1) & 80) as i32)
        | 0xe6440) as u8;

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

    buffer0[19] = ((buffer0[19] as isize) + 0xfffffee6
        - ((buffer0[(buffer3[4] % 20) as usize] >> 1) & 102) as isize) as u8;

    // This LOOKS like it should be a rol8x, but it just doesnt work out because if the shift amount is 0, then the output is 0 too :(
    // FIXME: Switch to weird_ror8
    buffer1[15] = (3
        * (((buffer1[72] >> (buffer4[(buffer1[190] % 21) as usize] & 7))
            ^ ((buffer1[72]) << (7 - ((buffer4[(buffer1[190] % 21) as usize]) - 1) & 7)))
            - (3 * buffer4[(buffer1[126] % 21) as usize])))
        ^ buffer1[15];

    buffer0[15] ^= (buffer2[(buffer1[181] % 35) as usize])
        * (buffer2[(buffer1[181] % 35) as usize])
        * (buffer2[(buffer1[181] % 35) as usize]);

    buffer2[4] ^= (buffer1[202]) / 3;

    // This could probably be quite a bit simpler.
    A = 92 - (buffer0[(buffer3[0] % 20) as usize]);
    E = (A & 0xc6) | (!(buffer1[105]) & 0xc6) | (A & (!(buffer1[105])));
    buffer2[1] += (E * E * E);

    buffer0[19] ^= ((224 | ((buffer4[(buffer1[92] % 21) as usize]) & 27))
        * (buffer2[(buffer1[41] % 35) as usize]))
        / 3;

    buffer1[140] += weird_ror8(92, (buffer1[5]) & 7);

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
                & 95)),
        4,
    );

    buffer3[8] = (((((buffer0[(buffer3[4] % 20) as usize]) & 95)
        & (((buffer4[(buffer1[68] % 21) as usize]) & 46) << 1))
        | 16)
        ^ 92);

    A = (buffer1[177]) + (buffer4[(buffer1[79] % 21) as usize]);
    D = (((A >> 1) | ((3 * (buffer1[148])) / 5)) & (buffer2[1]))
        | ((A >> 1) & ((3 * (buffer1[148])) / 5));
    buffer3[12] = (-34 - D as i32) as u8;

    A = 8 - ((buffer2[22]) & 7); // FIXME: buffer2[22] = 74, so A is always 6 and B^C is just ror8(buffer1[33], 6)
    B = ((buffer1[33]) >> (A & 7));
    C = (buffer1[33]) << ((buffer2[22]) & 7);
    buffer2[16] +=
        (((buffer2[(buffer3[0] % 35) as usize]) & 159) | (buffer0[(buffer3[4] % 20) as usize]) | 8)
            - ((B ^ C) | 128);

    // This one was very easy so I just skipped ahead and did it
    buffer0[14] ^= (buffer2[(buffer3[12] % 35) as usize]);

    // Monster goes here
    A = weird_rol8(
        (buffer4[(buffer0[(buffer1[201] % 20) as usize] % 21) as usize]),
        (((buffer2[(buffer1[112] % 35) as usize]) << 1) & 7),
    );
    D = (buffer0[(buffer1[208] % 20) as usize] & 131)
        | ((buffer0[(buffer1[164] % 20) as usize]) & 124);
    buffer1[19] += (A & (D / 5)) | ((A | (D / 5)) & 37);

    buffer2[8] = (weird_ror8(
        140,
        (((buffer4[(buffer1[45] % 21) as usize]) + 92)
            * ((buffer4[(buffer1[45] % 21) as usize]) + 92))
            & 7,
    ));

    buffer1[190] = 56;

    buffer2[8] ^= (buffer3[0]);

    buffer1[53] = !(((buffer0[(buffer1[83] % 20) as usize]) | 204) / 5);

    buffer0[13] += (buffer0[(buffer1[41] % 20) as usize]);

    buffer0[10] = ((((buffer2[(buffer3[0] % 35) as usize]) & (buffer1[2]))
        | (((buffer2[(buffer3[0] % 35) as usize]) | (buffer1[2])) & (buffer3[12])))
        / 15);

    A = (((56 | ((buffer4[(buffer1[2] % 21) as usize]) & 68))
        | (buffer2[(buffer3[8] % 35) as usize]))
        & 42)
        | ((((buffer4[(buffer1[2] % 21) as usize]) & 68) | 56)
            & (buffer2[(buffer3[8] % 35) as usize]));
    buffer3[16] = ((A * A) + 110);

    buffer3[20] = (202 - (buffer3[16]));

    buffer3[24] = buffer1[151];

    buffer2[13] ^= buffer4[(buffer3[0] % 21) as usize];

    B = (((buffer2[(buffer1[179] % 35) as usize]) - 38) & 177) | ((buffer3[12]) & 177);
    C = ((buffer2[(buffer1[179] % 35) as usize]) - 38) & (buffer3[12]);
    buffer3[28] = (30 + ((B | C) * (B | C)));

    buffer3[32] = (buffer3[28] + 62);

    // eek
    A = (((buffer3[20]) + ((buffer3[0]) & 74)) | !(buffer4[(buffer3[0] % 21) as usize])) & 121;
    B = (((buffer3[20]) + ((buffer3[0]) & 74)) & !(buffer4[(buffer3[0] % 21) as usize]));
    tmp3 = A | B;
    C = ((((A | B) ^ 0xffffffa6) | (buffer3[0])) & 4) | (((A | B) ^ 0xffffffa6) & (buffer3[0]));
    buffer1[47] = (((buffer2[(buffer1[89] % 35) as usize]) + C) ^ (buffer1[47]));

    buffer3[36] = (((rol8(((tmp & 179) + 68), 2) & (buffer0[3])) | (tmp2 & !(buffer0[3]))) - 15);

    buffer1[123] ^= 221;

    A = ((buffer4[(buffer3[0] % 21) as usize]) / 3) - (buffer2[(buffer3[4] % 35) as usize]);
    C = (((buffer3[0] & 163) + 92) & 246) | (buffer3[0] & 92);
    E = ((C | buffer3[24]) & 54) | (C & buffer3[24]);
    buffer3[40] = (A - E);

    buffer3[44] = (tmp3 ^ 81 ^ ((((buffer3[0]) >> 1) & 101) + 26));

    buffer3[48] = ((buffer2[(buffer3[4] % 35) as usize]) & 27);

    buffer3[52] = 27;

    buffer3[56] = 199;

    // caffeine
    buffer3[64] = ((buffer3[4])
        + ((((((((buffer3[40]) | (buffer3[24])) & 177) | ((buffer3[40]) & (buffer3[24])))
            & ((((buffer4[(buffer3[0] % 20) as usize]) & 177) | 176)
                | ((buffer4[(buffer3[0] % 21) as usize]) & !3)))
            | (((((buffer3[40]) & (buffer3[24])) | (((buffer3[40]) | (buffer3[24])) & 177))
                & 199)
                | (((((buffer4[(buffer3[0] % 21) as usize]) & 1) + 176)
                    | ((buffer4[(buffer3[0] % 21) as usize]) & !3))
                    & (buffer3[56]))))
            & (!(buffer3[52])))
            | (buffer3[48])));

    buffer2[33] ^= buffer1[26];

    buffer1[106] ^= buffer3[20] ^ 133;

    buffer2[30] = ((((buffer3[64]) / 3) - (275 | ((buffer3[0]) & 247)))
        ^ (buffer0[(buffer1[122] % 20) as usize]));

    buffer1[22] = (((buffer2[(buffer1[90] % 35) as usize]) & 95) | 68);

    A = ((buffer4[(buffer3[36] % 21) as usize]) & 184)
        | ((buffer2[(buffer3[44] % 35) as usize]) & !184);
    buffer2[18] += ((A * A * A) >> 1);

    buffer2[5] -= (buffer4[(buffer1[92] % 21) as usize]);

    A = ((((buffer1[41]) & !24) | ((buffer2[(buffer1[183] % 35) as usize]) & 24))
        & ((buffer3[16]) + 53))
        | (buffer3[20] & (buffer2[(buffer3[20] % 35) as usize]));
    B = ((buffer1[17]) & (!(buffer3[44])))
        | ((buffer0[(buffer1[59] % 20) as usize]) & (buffer3[44]));
    buffer2[18] ^= (A * B);

    A = weird_ror8((buffer1[11]), (buffer2[(buffer1[28] % 35) as usize]) & 7) & 7;
    B = ((((buffer0[(buffer1[93] % 20) as usize]) & !(buffer0[14])) | ((buffer0[14]) & 150)) & !28)
        | ((buffer1[7]) & 28);
    buffer2[22] = ((((B | weird_rol8((buffer2[(buffer3[0] % 35) as usize]), A)) & (buffer2[33]))
        | (B & weird_rol8((buffer2[(buffer3[0] % 35) as usize]), A)))
        + 74);

    A = buffer4[(((buffer0[(buffer1[39] % 20) as usize]) ^ 217) % 21) as usize];
    buffer0[15] -= (((((buffer3[20]) | (buffer3[0])) & 214) | ((buffer3[20]) & (buffer3[0]))) & A)
        | (((((buffer3[20]) | (buffer3[0])) & 214) | ((buffer3[20]) & (buffer3[0])) | A)
            & (buffer3[32]));

    // We need to save T here, and boy is it complicated to calculate!
    B = (((buffer2[(buffer1[57] % 35) as usize] & buffer0[(buffer3[64] % 20) as usize])
        | ((buffer0[(buffer3[64] % 20) as usize] | buffer2[(buffer1[57] % 35) as usize]) & 95)
        | (buffer3[64] & 45)
        | 82)
        & 32);
    C = ((buffer2[(buffer1[57] % 35) as usize] & buffer0[(buffer3[64] % 20) as usize])
        | ((buffer2[(buffer1[57] % 35) as usize] | buffer0[(buffer3[64] % 20) as usize]) & 95))
        & ((buffer3[64] & 45) | 82);
    D = ((((buffer3[0]) / 3) - ((buffer3[64]) | (buffer1[22]))) ^ ((buffer3[28]) + 62) ^ (B | C));
    T = (buffer0[(D % 20) as usize]);

    buffer3[68] = (((buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize])
        * (buffer0[(buffer1[99] % 20) as usize]))
        | (buffer2[(buffer3[64] % 35) as usize]));

    U = buffer0[(buffer1[50] % 20) as usize]; // this is also v100
    W = buffer2[(buffer1[138] % 35) as usize];
    X = buffer4[(buffer1[39] % 21) as usize];
    Y = buffer0[(buffer1[4] % 20) as usize]; // this is also v120
    Z = buffer4[(buffer1[202] % 21) as usize]; // also v124
    V = buffer0[(buffer1[151] % 20) as usize];
    S = buffer2[(buffer1[14] % 35) as usize];
    R = buffer0[(buffer1[145] % 20) as usize];

    A = ((buffer2[(buffer3[68] % 35) as usize]) & (buffer0[(buffer1[209] % 20) as usize]))
        | (((buffer2[(buffer3[68] % 35) as usize]) | (buffer0[(buffer1[209] % 20) as usize])) & 24);
    B = weird_rol8(
        (buffer4[(buffer1[127] % 21) as usize]),
        (buffer2[(buffer3[68] % 35) as usize]) & 7,
    );
    C = (A & (buffer0[10])) | (B & !(buffer0[10]));
    D = 7 ^ ((buffer4[(buffer2[(buffer3[36] % 35) as usize] % 21) as usize]) << 1);
    buffer3[72] = ((C & 71) | (D & !71));

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

    buffer1[23] ^= (((((((buffer4[(buffer1[17] % 21) as usize])
        | (buffer0[(buffer3[20] % 20) as usize]))
        & (buffer3[72]))
        | ((buffer4[(buffer1[17] % 21) as usize]) & (buffer0[(buffer3[20] % 20) as usize])))
        & ((buffer1[50]) / 3))
        | (((((buffer4[(buffer1[17] % 21) as usize]) | (buffer0[(buffer3[20] % 20) as usize]))
            & (buffer3[72]))
            | ((buffer4[(buffer1[17] % 21) as usize]) & buffer0[(buffer3[20] % 20) as usize])
            | ((buffer1[50]) / 3))
            & 246))
        << 1);

    buffer0[13] = (((((((buffer0[(buffer3[40] % 20) as usize]) | (buffer1[10])) & 82)
        | ((buffer0[(buffer3[40] % 20) as usize]) & (buffer1[10])))
        & 209)
        | (((buffer0[(buffer1[39] % 20) as usize]) << 1) & 46))
        >> 1);

    buffer2[33] -= buffer1[113] & 9;

    buffer2[28] -= ((((2 | (buffer1[110] & 222)) >> 1) & !223) | (buffer3[20] & 223));

    J = weird_rol8((V | Z), (U & 7)); // OK
    A = ((buffer2[16]) & T) | (W & (!(buffer2[16])));
    B = ((buffer1[33]) & 17) | (X & !17);
    E = ((Y | ((A + B) / 5)) & 147) | (Y & ((A + B) / 5)); // OK
    M = ((buffer3[40]) & (buffer4[(((buffer3[8]) + J + E) % 21) as usize]))
        | (((buffer3[40]) | (buffer4[(((buffer3[8]) + J + E) % 21) as usize])) & (buffer2[23]));

    buffer0[15] = (((((buffer4[(buffer3[20] % 21) as usize]) - 48) & (!(buffer1[184])))
        | (((buffer4[(buffer3[20] % 21) as usize]) - 48) & 189)
        | (189 & !(buffer1[184])))
        & (M * M * M));

    buffer2[22] += buffer1[183];

    buffer3[76] = ((3 * buffer4[(buffer1[1] % 21) as usize]) ^ buffer3[0]);

    A = buffer2[(((buffer3[8]) + (J + E)) % 35) as usize];
    F = ((((buffer4[(buffer1[178] % 21) as usize]) & A)
        | (((buffer4[(buffer1[178] % 21) as usize]) | A) & 209))
        * (buffer0[(buffer1[13] % 20) as usize]))
        * ((buffer4[(buffer1[26] % 21) as usize]) >> 1);
    G = (F + 0x733ffff9) * 198 - (((F + 0x733ffff9) * 396 + 212) & 212) + 85;
    buffer3[80] = ((buffer3[36]) + (G ^ 148) + ((G ^ 107) << 1) - 127);

    buffer3[84] = (((buffer2[(buffer3[64] % 35) as usize]) & 245)
        | ((buffer2[(buffer3[20] % 35) as usize]) & 10));

    A = (buffer0[(buffer3[68] % 20) as usize]) | 81;
    buffer2[18] -= ((A * A * A) & !buffer0[15]) | (((buffer3[80]) / 15) & (buffer0[15]));

    buffer3[88] = ((buffer3[8]) + J + E - (buffer0[(buffer1[160] % 20) as usize])
        + ((buffer4[(buffer0[(((buffer3[8] + J + E) & 255) % 20) as usize] % 21) as usize]) / 3));

    B = ((R ^ (buffer3[72])) & !198) | ((S * S) & 198);
    F = ((buffer4[(buffer1[69] % 21) as usize]) & (buffer1[172]))
        | (((buffer4[(buffer1[69] % 21) as usize]) | (buffer1[172])) & (((buffer3[12]) - B) + 77));
    buffer0[16] = (147 - (((buffer3[72]) & ((F & 251) | 1)) | (((F & 250) | (buffer3[72])) & 198)));

    C = ((buffer4[(buffer1[168] % 21) as usize]) & buffer0[(buffer1[29] % 20) as usize] & 7)
        | ((buffer4[(buffer1[168] % 21) as usize] | buffer0[(buffer1[29] % 20) as usize]) & 6);
    F = ((buffer4[(buffer1[155] % 21) as usize]) & (buffer1[105]))
        | (((buffer4[(buffer1[155] % 21) as usize]) | (buffer1[105])) & 141);
    buffer0[3] -= buffer4[(weird_rol32(F, C) % 21) as usize];

    buffer1[5] = (weird_ror8(
        (buffer0[12]),
        (((buffer0[(buffer1[61] % 20) as usize]) / 5) & 7),
    ) ^ (((!buffer2[(buffer3[84] % 35) as usize]) & 0xffffffff) / 5));

    buffer1[198] += buffer1[3];

    A = (162 | (buffer2[(buffer3[64] % 35) as usize]));
    buffer1[164] += ((A * A) / 5);

    G = weird_ror8(139, ((buffer3[80]) & 7));
    C = (((buffer4[(buffer3[64] % 21) as usize])
        * (buffer4[(buffer3[64] % 21) as usize])
        * (buffer4[(buffer3[64] % 21) as usize]))
        & 95)
        | ((buffer0[(buffer3[40] % 20) as usize]) & !95);
    buffer3[92] = ((G & 12)
        | ((buffer0[(buffer3[20] % 20) as usize]) & 12)
        | (G & (buffer0[(buffer3[20] % 20) as usize]))
        | C);

    buffer2[12] += (((buffer1[103]) & 32) | ((buffer3[92]) & ((buffer1[103]) | 60)) | 16) / 3;

    buffer3[96] = buffer1[143];

    buffer3[100] = 27;

    buffer3[104] = (((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8])))
        & (buffer3[64]))
        ^ 119);

    buffer3[108] = (238
        & (((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8]))) & (buffer3[64]))
            << 1));

    buffer3[112] = ((!(buffer3[64]) & ((buffer3[84]) / 3)) ^ 49);

    buffer3[116] = (98 & ((!(buffer3[64]) & ((buffer3[84]) / 3)) << 1));

    // finale
    A = ((buffer1[35]) & (buffer2[8])) | ((buffer3[40]) & !(buffer2[8]));
    B = (A & buffer3[64]) | (((buffer3[84]) / 3) & !(buffer3[64]));
    buffer1[143] = ((buffer3[96])
        - ((B & (86 + (((buffer1[172]) & 64) >> 1)))
            | ((((((buffer1[172]) & 65) >> 1) ^ 86)
                | ((!(buffer3[64]) & ((buffer3[84]) / 3))
                    | ((((buffer3[40]) & !(buffer2[8])) | ((buffer1[35]) & (buffer2[8])))
                        & (buffer3[64]))))
                & (buffer3[100]))));

    buffer2[29] = 162;

    A = ((((buffer4[(buffer3[88] % 21) as usize]) & 160)
        | ((buffer0[(buffer1[125] % 20) as usize]) & 95))
        >> 1);
    B = (buffer2[(buffer1[149] % 35) as usize]) ^ ((buffer1[43]) * (buffer1[43]));

    buffer0[15] += (B & A) | ((A | B) & 115);

    buffer3[120] = ((buffer3[64]) - (buffer0[(buffer3[40] % 20) as usize]));

    buffer1[95] = buffer4[(buffer3[20] % 21) as usize];

    A = weird_ror8(
        (buffer2[(buffer3[80] % 35) as usize]),
        ((buffer2[(buffer1[17] % 35) as usize])
            * (buffer2[(buffer1[17] % 35) as usize])
            * (buffer2[(buffer1[17] % 35) as usize]))
            & 7,
    );
    buffer0[7] -= (A * A);

    buffer2[8] = ((buffer2[8]) - (buffer1[184])
        + ((buffer4[(buffer1[202] % 21) as usize])
            * (buffer4[(buffer1[202] % 21) as usize])
            * (buffer4[(buffer1[202] % 21) as usize])));

    buffer0[16] = (((buffer2[(buffer1[102] % 35) as usize]) << 1) & 132);

    buffer3[124] = (((buffer4[(buffer3[40] % 21) as usize]) >> 1) ^ (buffer3[68]));

    buffer0[7] -= ((buffer0[(buffer1[191] % 20) as usize])
        - ((((buffer4[(buffer1[80] % 21) as usize]) << 1) & !177)
            | ((buffer4[(buffer4[(buffer3[88] % 21) as usize] % 21) as usize]) & 177)));

    buffer0[6] = buffer0[(buffer1[119] % 20) as usize];

    A = (buffer4[(buffer1[190] % 21) as usize] & !209) | (buffer1[118] & 209);
    B = buffer0[(buffer3[120] % 20) as usize] * buffer0[(buffer3[120] % 20) as usize];
    buffer0[12] = ((buffer0[(buffer3[84] % 20) as usize]
        ^ (buffer2[(buffer1[71] % 35) as usize] + buffer2[(buffer1[15] % 35) as usize]))
        & ((A & B) | ((A | B) & 27)));

    B = ((buffer1[32]) & (buffer2[(buffer3[88] % 35) as usize]))
        | (((buffer1[32]) | (buffer2[(buffer3[88] % 35) as usize])) & 23);
    D = ((((buffer4[(buffer1[57] % 21) as usize]) * 231) & 169) | (B & 86));
    F = ((((buffer0[(buffer1[82] % 20) as usize]) & !29)
        | ((buffer4[(buffer3[124] % 21) as usize]) & 29))
        & 190)
        | ((buffer4[(D / 5 % 21) as usize]) & !190);
    H = (buffer0[(buffer3[40] % 20) as usize])
        * (buffer0[(buffer3[40] % 20) as usize])
        * (buffer0[(buffer3[40] % 20) as usize]);
    K = (H & (buffer1[82])) | (H & 92) | ((buffer1[82]) & 92);
    buffer3[128] = (((F & K) | ((F | K) & 192)) ^ (D / 5));

    buffer2[25] ^= (((buffer0[(buffer3[120] % 20) as usize]) << 1) * (buffer1[5]))
        - (weird_rol8((buffer3[76]), ((buffer4[(buffer3[124] % 21) as usize]) & 7))
            & ((buffer3[20]) + 110));
}

fn rol8(input: u8, count: u8) -> u8 {
    (input << count) | input >> (8 - count)
}

fn rol8x(input: u8, count: u8) -> u8 {
    (input << count) | (input) >> (8 - count)
}

fn weird_ror8(input: u8, count: u8) -> u8 {
    if count == 0 {
        0
    } else {
        (input >> count) | (input << (8 - count))
    }
}

fn weird_rol8(input: u8, count: u8) -> u8 {
    if count == 0 {
        0
    } else {
        (input << count) | (input >> (8 - count))
    }
}

fn weird_rol32(input: u8, count: u8) -> u8 {
    if count == 0 {
        0
    } else {
        (input << count) ^ (input >> (8 - count))
    }
}
