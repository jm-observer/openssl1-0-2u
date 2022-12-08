use openssl::ssl;
use std::io::{stdout, Read, Write};
use std::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::NONE);
    let connector = builder.build();
    let sock = TcpStream::connect("google.com:443").unwrap();
    let mut tls = connector.connect("google.com", sock)?;
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
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();

    Ok(())
}
