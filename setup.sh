#!/bin/bash

read -p "Enter node name: " node_name
read -p "Enter db location: " db_location

wget https://github.com/selendra/selendra/releases/download/0.1.1/selendra -q --show-progress --progress=bar:force:noscroll
sudo mv selendra /usr/bin
sudo chmod +x /usr/bin/selendra

sudo tee /etc/systemd/system/selendra.service > /dev/null <<EOT
[Unit]
Description=Selendra Node
After=network.target
Documentation=https://github.com/selendra/selendra

[Service]
ExecStart=/usr/bin/selendra --base-path ${db_location} --chain testnet --port 30333 --rpc-port 9934 --ws-port 9944 --prometheus-port 9616 --rpc-methods Unsafe --rpc-cors all --pruning archive --validator --name "${node_name}" --bootnodes /ip4/157.245.56.213/tcp/30333/p2p/12D3KooWDLR899Spcx4xJ3U8cZttv9zjzJoey3HKaTZiNqwojZJB
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
EOT

sudo systemctl enable --now selendra.service
