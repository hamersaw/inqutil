#inqutil

##Overview
Basic tool for viewing and converting data from the inquest project
(https://github.com/hamersaw/inquest.git).

##Compiling
Note that you must have rust-protoc 
(https://github.com/stepancheg/rust-protobuf) installed.

1. make pbinit
2. make pbcompile
3. cargo build

##TODO
- handle probes and results other than http
- fix issue with closing sqlite3/postgresql writer
