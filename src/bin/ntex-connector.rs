use log::LevelFilter::Trace;
use openssl::ssl;
use std::convert::TryFrom;
use std::io;

use ntex::http::Uri;
use ntex::{codec, connect, util::Bytes, util::Either};

#[ntex::main]
async fn main() -> io::Result<()> {
    _main().await
}
async fn _main() -> io::Result<()> {
    custom_utils::logger::logger_stdout(Trace);

    let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
    builder.set_verify(ssl::SslVerifyMode::NONE);
    // openssl connector
    let connector = connect::openssl::Connector::new(builder.build());
    let uri = Uri::try_from("google.com:443").unwrap();
    let tls = connector.connect(uri).await.unwrap();
    println!("Connected to ssl server");
    let result = tls
        .send(
            Bytes::from(
                concat!(
                    "GET / HTTP/1.1\r\n",
                    "Host: google.com\r\n",
                    "Connection: close\r\n",
                    "Accept-Encoding: identity\r\n",
                    "\r\n"
                )
                .as_bytes(),
            ),
            &codec::BytesCodec,
        )
        .await
        .map_err(Either::into_inner)?;
    println!("Send result: {:?}", result);

    let resp = tls
        .recv(&codec::BytesCodec)
        .await
        .map_err(|e| e.into_inner())?
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "disconnected"))?;
    println!("Received: {:?}", resp);

    println!("disconnecting");

    // let mut plaintext = Vec::new();
    // tls.read_to_end(&mut plaintext).unwrap();
    // stdout().write_all(&plaintext).unwrap();

    Ok(())
}
