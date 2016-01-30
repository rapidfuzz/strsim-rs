#!/usr/bin/env bash

echo Installing Rust...
curl -sf https://static.rust-lang.org/rustup.sh | bash -s -- --revision=1.6.0 -y
echo "cd /vagrant" > /home/vagrant/.bashrc

