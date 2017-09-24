#!/bin/bash
docker run \
-d \
--name hooks \
-v `pwd`:/hooks \
-w /hooks \
rust:1.20 \
/bin/bash -c \
"\
apt update; \
apt install sshpass; \
cargo run --release; \
"
