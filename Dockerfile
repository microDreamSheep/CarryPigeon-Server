FROM alpine
RUN mkdir /home/carrypigeon
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache redis
WORKDIR /home/carrypigeon
COPY target/x86_64-unknown-linux-musl/release/carrypigeon-server /home/carrypigeon
EXPOSE 80
ENTRYPOINT ./carrypigeon-server
