# Token Substreams

Token-subgraph powered by substreams to index ERC20, ERC721 and ERC1155 tokens.

## Quick-Start

Install [Substreams](https://substreams.streamingfast.io/getting-started/installing-the-cli) and get an [authentication token](https://substreams.streamingfast.io/reference-and-specs/authentication).

- Build all modules:- `make build-all`
- Stream tokens graph entities that can be consumed by subgraph: -
    ```
    cd tokens
    make stream_graph_out
    ```
## Building and Deploying Subgraph

To start Graph-Node and IPFS, navigate to the graph-node directory and run the following command:

```
cd graph-node
bash start-all.sh
```

Build subgraph docker image:

```
docker build --build-arg DOCKER_HOST_IP=<DOCKER-HOST-IP> -t token-subgraph -f Dockerfile .
```
Replace <DOCKER-HOST-IP> with the IP address of your Docker host.

Once the Docker image is built, deploy the subgraph using the following command:
```
docker run --name subgraph-container token-subgraph
docker rm subgraph-container
```
### Enviroment Variables
Create a file named .env in the graph-node directory with the following content:

- `postgres_user`: the username for the PostgreSQL database.
- `postgres_pass`: the password for the PostgreSQL database.
- `postgres_db`: the name of the PostgreSQL database.
- `postgres_host`: the host address of the PostgreSQL database.
- `CHAIN_NAME`: the name of the blockchain network.
- `CHAIN_RPC`: the RPC endpoint of the blockchain network.
- `ipfs`: the IPFS endpoint.
- `GRAPH_NODE_CONFIG`: the path to the configuration file for graph-node.
- `GRAPH_LOG`: the logging level for graph-node.
- `RUST_BACKTRACE`: the backtrace level for Rust errors.
- `ethereum`: the Ethereum endpoint in the format network:rpc_endpoint.
- `SUBSTREAMS_ENDPOINT`: Firehose gRPC endpoint
- `SUBSTREAMS_API_TOKEN`: the API token for Substreams.

## Building and Deploying substreams-sink-postgres

Build docker image:

```
docker build -t token-sink -f Dockerfile.sink .
```

Once the Docker image is built, deploy the substreams using the following command:
```
docker run --env-file .env -p 9102:9102 --name token-sink-container token-sink
```
### Enviroment Variables
Create a file named .env in the root directory with the following content:

- `POSTGRES_DSN`: Postgres data source string. ex- psql://graph-node:let-me-in@172.17.0.1:5432/graph-node?sslmode=disable
- `FIREHOSE_ENDPOINT`: Firehose gRPC endpoint
- `SUBSTREAMS_API_TOKEN`: the API token for Substreams endpoint 

## Substreams Module DAG
### ERC20
![erc20-graph](images/erc20.png)

### ERC721
![erc721-graph](images/erc721.png)

### ERC1155
![erc1155-graph](images/erc1155.png)
