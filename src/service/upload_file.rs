use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use rand::{thread_rng, Rng};
use rocket::tokio::fs::{self, File};
use rocket::{
    data::{Data, ToByteUnit},
    delete, get,
    http::uri::Absolute,
    post,
    request::FromParam,
    response::content::RawText,
    uri, UriDisplayPath,
};

// release 2024 -> 43
static HOST: Absolute<'static> = uri!("https://0.0.0.0:2024");
// 控制分配文件上传ID大小
const ID_SIZE: usize = 25;

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    pub fn new(size: usize) -> Self {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        Self(Cow::Owned(id))
    }
    pub fn file_path(&self, file_name: &String) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload_file", "/");
        Path::new(root).join(file_name).join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}

#[post("/upload_file/<file_name>", data = "<paste>")]
pub async fn upload_file(file_name: String, paste: Data<'_>) -> std::io::Result<String> {
    // 获取Rocket.toml的配置信息
    //let figment = Box::new(rocket::Config::figment());
    //let config = Box::new(rocket::Config::try_from(*figment).unwrap());
    //let address = config.address.to_string();
    //let port = config.port.to_string();
    //let _host_uri = format!("http://{}:{}",address,port);

    let id = PasteId::new(ID_SIZE);
    paste
        .open(10.mebibytes())
        .into_file(id.file_path(&file_name))
        .await?;
    Ok(uri!(HOST.clone(), retrieve_file(file_name, id)).to_string())
}

#[get("/<file_name>/<id>")]
pub async fn retrieve_file(file_name: String, id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path(&file_name)).await.map(RawText).ok()
}

#[delete("/<file_name>/<id>")]
pub async fn delete_file(file_name: String, id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path(&file_name)).await.ok()
}
