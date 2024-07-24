use std::path::PathBuf;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;

#[derive(Error, Debug)]
pub enum ClamAVError {
    #[error("clamav-daemon disconnected")]
    Disconnect,
    #[error("no replay with PONG")]
    WithoutPong,
    #[error("tokio::io::Error: {0}")]
    IoError(#[from] tokio::io::Error),
    #[error("std::io::Error: {0}")]
    StdError(std::io::Error),
    #[error("virus")]
    Virus,
}

pub struct ClamAV {
    clamav_host: String,
}

impl ClamAV {
    pub async fn new(host: String) -> Self {
        Self { clamav_host: host }
    }
    pub async fn ping(&self) -> Result<(), ClamAVError> {
        let socket = TcpSocket::new_v4();
        if socket.is_err() {
            return Err(ClamAVError::Disconnect);
        }
        match socket
            .unwrap()
            .connect(self.clamav_host.parse().unwrap())
            .await
        {
            Ok(mut v) => {
                v.write_all(b"PING").await.unwrap();
                let mut response = Vec::with_capacity(4096);
                v.read_buf(&mut response).await.unwrap();
                if response.as_slice() == "PONG".as_bytes() {
                    return Ok(());
                }
                Err(ClamAVError::Disconnect)
            }
            Err(_) => Err(ClamAVError::Disconnect),
        }
    }
    pub async fn scan_file(&self, file_path: Box<PathBuf>) -> Result<Vec<u8>, ClamAVError> {
        let socket = TcpSocket::new_v4();
        if socket.is_err() {
            return Err(ClamAVError::Disconnect);
        }
        return match socket
            .unwrap()
            .connect(self.clamav_host.parse().unwrap())
            .await
        {
            Ok(mut v) => {
                let commend = Box::new(format!("SCAN {}", file_path.display()));
                let commend = Box::new(commend.as_bytes());
                v.write_all(*commend).await.unwrap();
                let mut response = Vec::with_capacity(4096);
                v.read_buf(&mut response).await.unwrap();
                Ok(response)
            }
            Err(_) => Err(ClamAVError::Disconnect),
        };
    }
}

impl Default for ClamAV {
    fn default() -> Self {
        Self {
            clamav_host: String::from("localhost:3310"),
        }
    }
}
