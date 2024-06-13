# Hiro
![assembly_4-removebg-preview](https://github.com/ritsource/hiro/assets/35898601/c5331c9b-fd4d-4693-a709-07cc96432d42)

A network attached distriburted file system that can run on a **Raspberry PI** (at home). It's a fast, horizontally-scalable, and fault-tolerant storage solution.

## Getting started

Clone the repository using the follwoing command
```shell
git clone https://github.com/ritsource/hiro.git
```
```shell
cd hiro
```

### Starting up the servers

To start all the servers and workers in local [docker](https://www.docker.com/), run ..
```shell
docker-compose up --build
```
**Or,** we can also run the master and worker server using [Cargo (Rust)](https://doc.rust-lang.org/stable/cargo/), with the following commands ..
```shell
cargo run -- --master --port 8080 --workers 127.0.0.1:5050,127.0.0.1:5051
```
```shell
cargo run -- --worker --port 5050 --master 127.0.0.1:8080
cargo run -- --worker --port 5051 --master 127.0.0.1:8080
cargo run -- --worker --port 5052 --master 127.0.0.1:8080
```
NOTE: worker needs a master address (127.0.0.1:8080), which you can pass using "--master" flag with address

**Upload a file using the client-cli**
```shell
cargo run -- --client ./data/demo.mp4 -m 127.0.0.1:8080
```
