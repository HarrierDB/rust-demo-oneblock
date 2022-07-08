use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    // 连接server端
    let mut stream = TcpStream::connect("178.128.112.21:8080")?;
    for _ in 0..10 {
        // 定义string类型的输入
        let mut input = String::new();
        io::stdin()
            // 读一行到input里，有问题报错
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        stream
            // 把input转成bytes写入stream里
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        // 创建reader
        let mut reader = BufReader::new(&stream);
        // 用vector创建一个buffer
        let mut buffer: Vec<u8> = Vec::new();
        reader
            // 读到换行
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        // 把读取到的打印出来
        println!("{}",
                 str::from_utf8(&buffer).expect("Could not write buffer as string"));
        // 换行
        println!("");
    }
    Ok(())
}
