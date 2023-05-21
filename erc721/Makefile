ENDPOINT ?= mainnet.eth.streamingfast.io:443
# ENDPOINT ?= goerli.eth.streamingfast.io:443
startBlock ?= 847527
endBlock ?=  937910

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	
.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:build
	substreams pack ./substreams.yaml --output-file ../target/erc721-v0.0.1.spkg

.PHONY: stream_map_collections
stream_map_collections: build
	substreams run -e $(ENDPOINT) substreams.yaml map_collections --start-block ${startBlock} --stop-block ${endBlock}

.PHONY: stream_map_transfers
stream_map_transfers: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s 12287507 -t +1

.PHONY: stream_map_extract_mints
stream_map_extract_mints: build
	substreams run -e $(ENDPOINT) substreams.yaml map_extract_mints -s 12287507 -t +1

.PHONY: stream_map_extract_tokens
stream_map_extract_tokens: build
	substreams run -e $(ENDPOINT) substreams.yaml map_extract_tokens -s 13300950 -t +1

.PHONY: stream_map_approvals
stream_map_approvals: build
	substreams run -e $(ENDPOINT) substreams.yaml map_approvals -s 12311317 -t +1

.PHONY: stream_map_collection_entities
stream_map_collection_entities: build
	substreams run -e $(ENDPOINT) substreams.yaml map_collection_entities -s 12287507 -t +1

.PHONY: stream_map_transfer_entities
stream_map_transfer_entities: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfer_entities -s 13300950 -t +1

.PHONY: stream_map_token_entities
stream_map_token_entities: build
	substreams run -e $(ENDPOINT) substreams.yaml map_token_entities -s 13301043 -t +1

.PHONY: stream_graph_out
stream_graph_out: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 13300950 -t +10

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s 13300950 -t +10
	
.PHONE: deploy_local
deploy_local: 
	graph codegen
	graph build --ipfs http://localhost:5001 subgraph.yaml
	graph create nft --node http://127.0.0.1:8020
	graph deploy --node http://127.0.0.1:8020 --ipfs http://127.0.0.1:5001 --version-label v0.0.1 nft subgraph.yaml

.PHONY: sink_db
sink_db:package
	substreams-sink-postgres run  "psql://graph-node:let-me-in@localhost:5432/graph-node?sslmode=disable"  $(ENDPOINT)  ../target/erc721-v0.0.1.spkg  db_out