FROM alpine
COPY target/release/carrypigeon-server /carrypigeon/
WORKDIR /carrypigeon/
EXPOSE 80
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache redis
CMD ["./carrypigeon-server"]
