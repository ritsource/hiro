# Quick upload
```
docker-compose up --build
```
Or,
```
cargo run -- --master --port 8080 --workers 127.0.0.1:5050,127.0.0.1:5051
```
```
cargo run -- --worker --port 5051 --master 127.0.0.1:8080
cargo run -- --worker --port 5050 --master 127.0.0.1:8080
```
```
cargo run -- --client ./data/ipsum.text -m 127.0.0.1:8080
```
<!--

# state distribution

1. Master - [ Files, Pieces, and Piece/Peer(worker) mapping ]
2. Worker - [ Pirce (indexed by fileID or pirceID) ]
3. Client - [ ... none! ]

# tasks

A rough outline for what happens while writing files

1. [ Client ] sending a new file metadata to master
2. [ Master ] calculating the pirces to construct from for the file
3. [ Master ] assigning each pirces to one/multiple workers, and saving it to a table
4. [ Master ] sending information about which piece is going to be stored in which worker
5. [ Client ] writing pirces of the original file to workers
6. [ Worker ] writing those pieces to SSD/HHDs and sending confirmation messages to master and client

For reading files

1. [ Client ] requesting chunk and worker location from master for a given file
2. [ Master ] sending the requested data to client
3. [ Client ] reading the pieces from one/multiple workers

# data (interfaces) that needs to be transfered over RPC

1. [ File ]
2. [ Piece/Peer Maps ] peer (worker ip addr and port stuff)
3. [ Pieces ] from master to worker, so that worker can expect files beforehand (not necessary though)
4. [ Pieces + Data ] pieces with the actual piece data
5. ...

# messages

> [ Client ] sending a new file metadata to master

From `Client` to `Master`   
Request -> `File`   
Response -> `HashMap<Chunk, Vec<<Peer>>`   

> [ Master ] calculating the pirces to construct from for the file
> [ Master ] assigning each pirces to one/multiple workers, and saving it to a table
> [ Master ] sending information about which piece is going to be stored in which worker
> [ Client ] writing pirces of the original file to workers
> [ Worker ] writing those pieces to SSD/HHDs and sending confirmation messages to master and client

... many :p


# idea

5. [ Client ] writing pirces of the original file to workers - the client sends data about other workers that the piece
   needs to be copied to.


# handling configurations

## what are we gonan have

### 1 client
CRUD (create, read, update, delete) files

### 1 master


### 1+ workers


# Tasks (and TaskQueue)



# Notes

Check out [https://en.wikipedia.org/wiki/Server_Message_Block](https://en.wikipedia.org/wiki/Server_Message_Block)






# Note - File upload

1. Client sends file metadata to master
2. Master assigns 1 major and multiple minor workers for a given file and creates a map
   something like, `Vec<(Piece, Vec<Peer>)>`, where in the `Vec<Peer>` only the first peer
   is "major"
2. Master responds with the data







-->
