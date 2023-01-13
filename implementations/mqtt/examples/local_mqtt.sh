#!/bin/sh

cargo run --release --bin socioty_mqtt_node -- "node0001_config.toml" &
cargo run --release --bin socioty_mqtt_node -- "node0002_config.toml" &
cargo run --release --bin socioty_mqtt_node -- "node0003_config.toml" &
cargo run --release --bin socioty_mqtt_node -- "node0004_config.toml" &
cargo run --release --bin socioty_mqtt_node -- "node0005_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0006_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0007_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0008_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0009_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0010_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0011_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0012_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0013_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0014_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0015_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0016_config.toml" &
# cargo run --release --bin socioty_mqtt_node -- "node0017_config.toml" &

