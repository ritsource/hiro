# Hiro

Hiro is a scalable, fault-tolerant, network-attached distributed file system. It's for personal use, so the focus is reliability rather than speed, but speed is always welcome (we'll see).

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


