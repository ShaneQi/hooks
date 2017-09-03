#!/bin/bash
docker run \
-d \
--name hooks \
-v `pwd`:/hooks \
-w /hooks \
rust:1.20 \
/bin/bash -c \
"\
cargo run --release\
"
