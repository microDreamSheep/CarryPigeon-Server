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

## 关于暂停更新和维护

目前我是一个准高三学生，在对计算机热爱的驱动下尝试了这个项目的编写。事实上这个项目写的也不太好，String的clone满天飞诸如此类，很大一定程度上是赶工造成的。因此先提前感谢大家的批评和指正，也欢迎大家提交针对此屎山的补丁。我也本打算在暑假时抓紧时间完善功能和优化代码。但是我最近有些疲惫，或许我也应该停下来，看看我喜欢的悬疑侦探小说或武侠小说，亦或者是出去散散心，锻炼身体。未来一年中，我也将停止更新和维护直至高考结束。尽管如此，依旧欢迎大家给项目提PR或者指出不足，我会在放月假时审核PR和记录反馈。在高考结束后，我也将重新开始维护本项目和开源计划中本应这个暑假内完成的安卓客户端和PC端。

最后仍感谢看到这里的朋友们和所有支持过我的人，欢迎大家给项目提PR或者指出不足

​                                                                                                                                            写于2024.7.15