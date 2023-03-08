#!/bin/bash
cargo b -r
sudo setcap cap_net_admin=eip /home/ksleo/rust/ttcp/target/release/ttcp
/home/ksleo/rust/ttcp/target/release/ttcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
