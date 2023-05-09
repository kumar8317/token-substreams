FROM rust:1.60 as builder
WORKDIR /app
COPY . .
RUN cd /app/common_events && cargo build --target wasm32-unknown-unknown --release
RUN cd /app/erc20 && cargo build --target wasm32-unknown-unknown --release
RUN cd /app/erc721 && cargo build --target wasm32-unknown-unknown --release
RUN cd /app/erc1155 && cargo build --target wasm32-unknown-unknown --release
RUN cd /app/tokens && cargo build --target wasm32-unknown-unknown --release

FROM ubuntu:20.04 as packager
WORKDIR /app
RUN apt-get update && \
    apt-get install -y curl jq
RUN LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk '/download.url.*linux/ {print $2}' | sed 's/"//g') && \
    curl -L $LINK | tar zxf - 

COPY --from=builder /app/target/wasm32-unknown-unknown/release/common_events.wasm /app/target/wasm32-unknown-unknown/release/common_events.wasm
COPY --from=builder /app/common/proto /app/common/proto
COPY --from=builder /app/common_events/substreams.yaml /app/common_events/substreams.yaml
RUN ./substreams pack -o /app/target/common_events-v0.0.1.spkg /app/common_events/substreams.yaml

COPY --from=builder app/target /app/target
COPY --from=builder /app/target/wasm32-unknown-unknown/release/erc20.wasm /app/target/wasm32-unknown-unknown/release/erc20.wasm
COPY --from=builder /app/erc20/proto /app/erc20/proto
COPY --from=builder /app/erc20/substreams.yaml /app/erc20/substreams.yaml
RUN ./substreams pack -o /app/target/erc20-v0.0.1.spkg /app/erc20/substreams.yaml

COPY --from=builder /app/target/wasm32-unknown-unknown/release/erc721.wasm /app/target/wasm32-unknown-unknown/release/erc721.wasm
COPY --from=builder /app/erc721/proto /app/erc721/proto
COPY --from=builder /app/erc721/substreams.yaml /app/erc721/substreams.yaml
RUN ./substreams pack -o /app/target/erc721-v0.0.1.spkg /app/erc721/substreams.yaml

COPY --from=builder /app/target/wasm32-unknown-unknown/release/erc1155.wasm /app/target/wasm32-unknown-unknown/release/erc1155.wasm
COPY --from=builder /app/erc1155/proto /app/erc1155/proto
COPY --from=builder /app/erc1155/substreams.yaml /app/erc1155/substreams.yaml
RUN ./substreams pack -o /app/target/erc1155-v0.0.1.spkg /app/erc1155/substreams.yaml

COPY --from=builder /app/target/wasm32-unknown-unknown/release/token_substreams.wasm /app/target/wasm32-unknown-unknown/release/token_substreams.wasm
COPY --from=builder /app/tokens/substreams.yaml /app/tokens/substreams.yaml
RUN ./substreams pack -o /app/token_substreams.spkg /app/tokens/substreams.yaml

FROM scratch as spkg-export
WORKDIR /app
COPY --from=packager /app/token_substreams.spkg .

FROM node:lts-alpine AS node-builder
WORKDIR /token-substreams

COPY --from=spkg-export /app/token_substreams.spkg ./token_substreams.spkg
COPY ./subgraph.yaml /token-substreams/subgraph.yaml
COPY ./schema/ /token-substreams/schema
RUN mkdir /token-substreams/build

RUN apk add --no-cache git
RUN npm install -g @graphprotocol/graph-cli

ARG DOCKER_HOST_IP
ENV DOCKER_HOST_IP=$DOCKER_HOST_IP

RUN graph codegen

CMD graph build --ipfs http://${DOCKER_HOST_IP}:5001 subgraph.yaml ; graph create tokenSubgraph --node http://${DOCKER_HOST_IP}:8020 ;graph deploy --node http://${DOCKER_HOST_IP}:8020 --ipfs http://${DOCKER_HOST_IP}:5001 --version-label v0.0.1 tokenSubgraph /token-substreams/subgraph.yaml