**Hiro** is a network-attached distributed file system that stores large files by automatically splitting them into smaller chunks and distributing them across several computers on your network. When you upload a file, Hiro keeps track of where every piece is stored, so you can easily retrieve it later, even if some computers go offline. This makes it easy to build your own reliable, private cloud storage at home or in the office, using low-power devices like Raspberry Pis.

<br>
<p align="center">
  <img src="https://github.com/user-attachments/assets/77adac3a-3e8b-48a9-822c-447225709e57" alt="Hiro-system" />
</p>

Much of Hiro’s architecture and design is inspired by the [Google File System (GFS)](https://research.google/pubs/pub51/), a scalable distributed file system developed by Google to support large, data-intensive applications. While Hiro is built for home labs and small-scale deployments, it adapts many of the core ideas from GFS. Its design leverages proven techniques such as file chunking, distributed metadata management, and redundancy to ensure data integrity and availability, even in environments with limited resources. With Hiro, users can build resilient, self-hosted storage solutions tailored to their specific use cases.

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
Hiro can be deployed as a Network Attached File System (NAS), providing seamless storage and retrieval capabilities over your network. It comes with a powerful CLI client for easy interaction and management.


<br>
<p align="left">
  <img width="300px" src="https://github.com/ritsource/hiro/assets/35898601/c5331c9b-fd4d-4693-a709-07cc96432d42" alt="raspi-cluster" />
</p>

For optimal scalability and reliability, Hiro is designed to run on a Kubernetes cluster. Our reference deployment uses a **6-node Raspberry Pi cluster**, making it both cost-effective and energy-efficient for home labs or edge computing.

Interested in building your own Raspberry Pi Kubernetes cluster? Check out this comprehensive guide: [https://alexsniffin.medium.com/a-guide-to-building-a-kubernetes-cluster-with-raspberry-pis-23fa4938d420](https://alexsniffin.medium.com/a-guide-to-building-a-kubernetes-cluster-with-raspberry-pis-23fa4938d420).

