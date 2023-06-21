use tokio::{
    fs::File,
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, BufReader},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = File::open("./tmp/read.txt").await?;
    let mut reader = BufReader::new(file);
    let mut b = [0; 4];
    reader.fill_buf().await?;
    let buf = reader.buffer();
    print!("{:02X?}", buf);
    reader.read_exact(&mut b).await?;
    // let mut line = String::new();
    // loop {
    //     let r = reader.read_line(&mut line).await?;
    //     println!("{r}");
    //     if r == 2 || line == "\r\n" {
    //         break;
    //     }
    // }
    // print!("{}", line);
    // let mut body = String::new();
    // reader.read_to_string(&mut body).await?;
    print!("{:02X?}", b);
    Ok(())
}
