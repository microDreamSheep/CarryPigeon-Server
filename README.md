# CarryPigeon
## About
一个简单的聊天服务器
## Build
```shell
rustup target add x86_64-unknown-linux-musl
sudo docker build -t carrypigeon .
```
stop
```shell
sudo docker rm $(sudo docker ps -aq)
```