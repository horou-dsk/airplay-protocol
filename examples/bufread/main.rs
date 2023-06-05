use tokio::{
    fs::File,
    io::{self, AsyncBufReadExt, AsyncReadExt, BufReader},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = File::open("./read.txt").await?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let r = reader.read_line(&mut line).await?;
        println!("{r}");
        if r == 2 || line == "\r\n" {
            break;
        }
    }
    print!("{}", line);
    let mut body = String::new();
    reader.read_to_string(&mut body).await?;
    print!("{}", body);
    Ok(())
}
