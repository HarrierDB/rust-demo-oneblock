// 引入io库处理error
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
// 对输入的每一个流创建一个线程
use std::thread;
use std::time;

// 这个client就是这个流
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    // 创建一个buf的数组，内容为0长度512
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // 从流里面读内容
        let bytes_read = stream.read(&mut buf)?;
        // 为0就啥都不干
        if bytes_read == 0 {
            return Ok(());
        }
        // echo内容
        stream.write(&buf[..bytes_read])?;
        // 服务间隔
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 监听ip与端口号
    let listener = TcpListener::bind("178.128.112.21:8080")?;
    // 创建一个容器放句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        // 转换stream流
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // 把handle加到容器里
        thread_vec.push(handle);
    }
    // 等待线程结束
    for handle in thread_vec {

        handle.join().unwrap();
    }

    Ok(())
}

