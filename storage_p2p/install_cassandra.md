## Install cassandra library in [target machine]

Download and install cassandra-cpp dependencies(libuv), have *-dbg, *-dev, driver.

dependencies driver, 1.35 ver. 
```bash
apt install libuv1 libuv1t64

# or manually:
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/dependencies/libuv/v1.35.0/libuv1_1.35.0-1_amd64.deb
sudo dpkg -i libuv1_1.35.0-1_amd64.deb
```
dependencies *-dbg
```bash
apt install libuv1-dbg

# or manually:
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/dependencies/libuv/v1.35.0/libuv1-dbg_1.35.0-1_amd64.deb
sudo dpkg -i libuv1-dbg_1.35.0-1_amd64.deb
```
dependencies *-dev
```bash
apt install libuv1-dbg-dev

# or manually:
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/dependencies/libuv/v1.35.0/libuv1-dev_1.35.0-1_amd64.deb
sudo dpkg -i libuv1-dev_1.35.0-1_amd64.deb
```

Download and install cassandra-cpp driver, have *-dbg, *-dev, driver.

cassandra driver 2.16 ver.
```bash
#wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.12.0/cassandra-cpp-driver_2.12.0-1_amd64.deb
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver_2.16.0-1_amd64.deb
sudo dpkg -i cassandra-cpp-driver_2.16.0-1_amd64.deb
```
cassandra *-dev 2.16 ver.
```bash
#wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.12.0/cassandra-cpp-driver-dev_2.12.0-1_amd64.deb
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver-dev_2.16.0-1_amd64.deb
sudo dpkg -i cassandra-cpp-driver-dev_2.16.0-1_amd64.deb
```
cassandra *-dbg 2.16 ver.
```bash
#wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.12.0/cassandra-cpp-driver-dbg_2.12.0-1_amd64.deb
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver-dbg_2.16.0-1_amd64.deb
sudo dpkg -i cassandra-cpp-driver-dbg_2.16.0-1_amd64.deb
```
__Make sure that the driver (specifically libcassandra_static.a and libcassandra.so) are in your “/usr/local/lib64/” or “/usr/lib/x86_64-linux-gnu/” directory__
```bash
sudo find / -name libcassandra_static.a
sudo find / -name libcassandra.so
```

## Enable multiarch support
```bash
wget http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/multiarch-support_2.27-3ubuntu1_amd64.deb
sudo apt-get install ./multiarch-support_2.27-3ubuntu1_amd64.deb
```

## Install Cassandra driver
```bash
wget https://downloads.datastax.com/cpp-driver/ubuntu/18.04/cassandra/v2.16.0/cassandra-cpp-driver_2.16.0-1_amd64.deb
sudo dpkg -i cassandra-cpp-driver_2.16.0-1_amd64.deb
```

## Docker 
```bash
docker pull cassandra:latest
docker run --net=host --name=cassandra cassandra:latest
```

## Use cqlsh

cqlsh is a Python-based command-line client for running CQL commands on a cassandra cluster.
It can be installed locally or used via Docker.

### cqlsh locally

Download it from [this page](https://downloads.datastax.com/#cqlsh).

Unpack the distribution:
```bash
tar -xzvf cqlsh-X.X.tar.gz
```
```bash
cd cqlsh-X.X
./cqlsh
./cqlsh 127.0.0.1 9042
```

### cqlsh from Docker

Use the `docker exec` to run `cqlsh` command inside the existing container
```bash
docker exec -it cassandra cqlsh
```

### Check it

Output on the terminal:
```
Connected to Test Cluster at 127.0.0.1:9042.
[cqlsh 6.8.0 | Cassandra 4.1.3 | CQL spec 3.4.6 | Native protocol v5]
Use HELP for help.
cqlsh>
```

Check everything is up and running by executing a simple query:
```bash
SELECT keyspace_name FROM system_schema.keyspaces;
```
```
 keyspace_name
--------------------
        system_auth
      system_schema
 system_distributed
             system
      system_traces
```

---

### [OPTIONAL] To create a certificate:
https://docs.datastax.com/en/cassandra-oss/3.0/cassandra/configuration/secureSSLCertWithCA.html

```bash
$ openssl req -config gen_rootCa_cert.conf -new -x509 -nodes -subj /CN=rootCa/OU=TestCluster/O=Ctrl+z/C=US/ -keyout rootCa.key -out rootCa.crt -days 3650
```

#### To verify the rootCa certificate:
```bash
$ openssl x509 -in rootCa.crt -text -noout
```