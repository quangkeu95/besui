install-sui-binaries:
	cargo install --locked --git https://github.com/MystenLabs/sui.git --branch "devnet" sui sui-gateway

generate-entities:
	sea-orm-cli generate entity -u postgresql://postgres:postgres@localhost:5434/besui -o crates/besui-core/entity/src