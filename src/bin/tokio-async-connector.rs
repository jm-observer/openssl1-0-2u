use native_tls::TlsConnector;
use openssl::ssl;
use std::io::{stdout, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sock = TcpStream::connect("google.com:443").await.unwrap();
    let cx = TlsConnector::builder().build()?;
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let mut tls = cx.connect("google.com", sock).await?;
    tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .await
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).await.unwrap();
    stdout().write_all(&plaintext).unwrap();

    Ok(())
}
