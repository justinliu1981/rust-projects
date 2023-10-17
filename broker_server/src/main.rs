use anyhow::Result;
use axum::{extract::Query, response::Html, routing::get, Router};
use bytes::Bytes;
use chrono::Local;
use futures::{SinkExt, StreamExt};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()> {
    //let rt1 = Runtime::new().unwrap();
    // 创建一个blocking thread，可立即执行(由操作系统调度系统决定何时执行)
    // 注意，不阻塞当前线程
    // 进入runtime，但不阻塞当前线程
   // let guard1 = rt1.enter();

    // 生成的异步任务将放入当前的runtime上下文中执行
    tokio::spawn(async {
        println!("in task: {}", now());
        //启动HTTP Server
        println!("HTTP server is listening ...");
        let app = Router::new().route("/", get(handler));

        // Address that server will bind to.
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // 释放runtime上下文，这并不会删除runtime
    //drop(guard1);

    println!("TCP server is listening ...");
    let listener = TcpListener::bind("127.0.0.1:9527").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted: {:?}", addr);
        //LengthDelimitedCodec 默认4字节长度
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            // 接收到的消息会只包含消息主体（不包含长度）
            while let Some(Ok(data)) = stream.next().await {
                println!("Got: {:?}", String::from_utf8_lossy(&data));
                // 发送的消息也只需要发送消息主体，不需要提供长度
                // Framed/LengthDelimitedCodec 会自动计算并添加
                //    let response = &data[0..5];
                stream.send(Bytes::from(data)).await.unwrap();
            }
        });
    }
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

//handler process the request and return the response
async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}
