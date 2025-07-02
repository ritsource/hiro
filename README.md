# Hiro

Hiro is a system that lets you store large files by automatically splitting them into smaller pieces and spreading those pieces across several computers on your network. When you upload a file, Hiro keeps track of where every piece is stored, so you can easily get your file back later, even if some computers go offline. This makes it easy to build your own reliable, private cloud storage at home or in a small office, using regular hardware like Raspberry Pis or old PCs.

> **Note:**  
A lot of inspiration for Hiro's architecture and design is taken from the [Google File System (GFS)](https://research.google/pubs/pub51/). GFS is a scalable distributed file system developed by Google to handle large data-intensive applications. Hiro adapts many of the core ideas from GFS. While Hiro is designed for home labs and small-scale deployments, its underlying principles are rooted in the proven concepts of GFS, making it robust and reliable even in environments with limited resources. By leveraging techniques such as file chunking, distributed metadata management, and redundancy, Hiro ensures data integrity and availability, allowing users to build their own resilient storage solutions for various usecases.

## Getting started

Clone the repository ..
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

### Deployment
![assembly_4-removebg-preview](https://github.com/ritsource/hiro/assets/35898601/c5331c9b-fd4d-4693-a709-07cc96432d42)

Hiro can be deployed as a Network Attached File System (NAS), providing seamless storage and retrieval capabilities over your network. It comes with a powerful CLI client for easy interaction and management.

For optimal scalability and reliability, Hiro is designed to run on a Kubernetes cluster. Our reference deployment uses a 6-node Raspberry Pi cluster, making it both cost-effective and energy-efficient for home labs or edge computing.

Interested in building your own Raspberry Pi Kubernetes cluster? Check out this comprehensive guide:  
https://alexsniffin.medium.com/a-guide-to-building-a-kubernetes-cluster-with-raspberry-pis-23fa4938d420

