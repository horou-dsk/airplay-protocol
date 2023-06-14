use airplay2_protocol::{foo, print_buf};

fn main() {
    let result = unsafe { foo(1, 6) };
    println!("result = {result}");
    unsafe {
        let buf = [1, 3, 4, 5];
        print_buf(buf.as_ptr(), buf.len());
        println!("{:?}", buf);
    }
}
