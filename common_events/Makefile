ENDPOINT ?= mainnet.eth.streamingfast.io:443
# ENDPOINT ?= goerli.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	
.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:build
	substreams pack ./substreams.yaml --output-file ../target/common_events-v0.0.1.spkg

.PHONY: stream_map_ownership_transfers
stream_map_ownership_transfers: build
	substreams run -e $(ENDPOINT) substreams.yaml map_ownership_transfers -s 12287507 -t +1
