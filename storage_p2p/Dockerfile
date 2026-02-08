FROM ubuntu:20.04

WORKDIR /app

RUN apt-get update && apt-get install -y \
    wget \
    dpkg \
    libssl-dev \
    tar \
    && apt-get install -f -y \
    && rm -rf /var/lib/apt/lists/*

RUN apt update && apt upgrade -y
RUN apt install libuv1-dev libssl-dev libkrb5-dev zlib1g-dev -y

RUN wget http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/multiarch-support_2.27-3ubuntu1_amd64.deb \
    && dpkg -i multiarch-support_2.27-3ubuntu1_amd64.deb \
    || apt-get install -f -y \
    && rm multiarch-support_2.27-3ubuntu1_amd64.deb

# Drivers
RUN wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver_2.16.0-1_amd64.deb
RUN dpkg  -i cassandra-cpp-driver_2.16.0-1_amd64.deb

# Dev Drivers
RUN wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver-dev_2.16.0-1_amd64.deb
RUN dpkg  -i cassandra-cpp-driver-dev_2.16.0-1_amd64.deb

# Debug Drivers
RUN wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver-dbg_2.16.0-1_amd64.deb
RUN dpkg  -i cassandra-cpp-driver-dbg_2.16.0-1_amd64.deb

RUN find / -name libcassandra_static.a
RUN find / -name libcassandra.so

COPY main /app/main
COPY cert /app/cert

ENV CASSANDRA_HOST 127.0.0.1
ENV ZENOH_HOST 0.0.0.0
ENV CERT_PATH /app/cert/rootCa.crt
ENV RUST_LOG=debug

CMD ["./main"]

EXPOSE 7447