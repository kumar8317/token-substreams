ENDPOINT ?= mainnet.eth.streamingfast.io:443
# ENDPOINT ?= firehose.eth.zeeve.net:443
startBlock ?= 462663
endBlock ?=  17277277

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	
.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack -o ../token_substreams.spkg ./substreams.yaml

.PHONY: stream_graph_out
stream_graph_out: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 462680 -t +10

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out --start-block $(startBlock) --stop-block $(endBlock)

.PHONY: sink_db
sink_db:package
	substreams-sink-postgres run  "psql://graph-node:let-me-in@localhost:5432/graph-node?sslmode=disable"  $(ENDPOINT)  ../token_substreams.spkg  db_out 