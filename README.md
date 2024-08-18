# CarryPigeon
## About
一个简单的聊天服务器
## Build
```shell
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release
sudo docker build -t carrypigeon .
```
stop
```shell
sudo docker rm $(sudo docker ps -aq)
```

## Run
### 请注意，如果上线本项目时，要注意替换该tls证书
直接运行该容器或者运行可执行文件即可
