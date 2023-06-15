use bnum::types::{U2048, U256, U512};

macro_rules! bnum_bytes_len {
    ($var:expr) => {
        ($var.bits() + 7) / 8
    };
}

fn main() {
    let n_hex = "AC6BDB41324A9A9BF166DE5E1389582FAF72B6651987EE07FC3192943DB56050A37329CBB4\
A099ED8193E0757767A13DD52312AB4B03310DCD7F48A9DA04FD50E8083969EDB767B0CF60\
95179A163AB3661A05FBD5FAAAE82918A9962F0B93B855F97993EC975EEAA80D740ADBF4FF\
747359D041D5C33EA71D281E446B14773BCA97B43A23FB801676BD207A436C6481F1D2B907\
8717461A5B9D32E688F87748544523B524B0D57D5EA77A2775D2ECFA032CFBDBF52FB37861\
60279004E57AE6AF874E7303CE53299CCC041C7BC308D82A5698F3A8D0C38271AE35F8E9DB\
FBB694B5C803D89F7AE435DE236D525F54759B65E372FCD68EF20FA7111F9E4AFF73";
    let n1 = U2048::from_str_radix(n_hex, 16).unwrap();
    let n2 = U2048::from_str_radix("2", 16).unwrap();
    let len_n1 = bnum_bytes_len!(n1);
    let _len_n2 = bnum_bytes_len!(n2);
    let _n_bytes = 512;

    println!("{}", len_n1);
    println!("{}", n2);

    let result = [
        122, 3, 88, 216, 178, 70, 201, 143, 111, 14, 170, 247, 76, 227, 109, 112, 27, 81, 22, 142,
    ];

    let bn = U256::from_be_slice(&result).unwrap();
    let bn_bytes = bn.to_be_bytes();
    println!(
        "{:?}",
        &bn_bytes[bn_bytes.len() - bnum_bytes_len!(bn) as usize..]
    );
    let a = U512::from(111500u32);
    println!("{}", bnum_bytes_len!(a));
    println!("{:?}", a.to_be_bytes());
}
