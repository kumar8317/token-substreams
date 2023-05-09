.PHONY: build-all
build-all:
	$(MAKE) -C common_events build
	$(MAKE) -C erc20 build
	$(MAKE) -C erc721 build
	$(MAKE) -C erc1155 build
	$(MAKE) -C tokens build

.PHONY: package-all
package-all:
	$(MAKE) -C common_events package
	$(MAKE) -C erc20 package
	$(MAKE) -C erc721 package
	$(MAKE) -C erc1155 package
	$(MAKE) -C tokens package

.PHONE: deploy_local
deploy_local: 
	graph codegen
	graph build --ipfs http://localhost:5001 subgraph.yaml
	graph create tokenSubgraph --node http://127.0.0.1:8020
	graph deploy --node http://127.0.0.1:8020 --ipfs http://127.0.0.1:5001 --version-label v0.0.1 tokenSubgraph subgraph.yaml

.PHONY: docker_build_subgraph
docker_build_subgraph:
	docker build --build-arg DOCKER_HOST_IP=172.17.0.1 -t token-subgraph -f Dockerfile .

.PHONY: docker_deploy_subgraph
docker_deploy_subgraph:
	docker run --name subgraph-container token-subgraph
	docker rm subgraph-container

.PHONY:	
docker_build_postgres_sink:
	docker build -t token-sink -f Dockerfile.sink .

.PHONY:
docker_run_postgres_sink:
	docker run --env-file .env -p 9102:9102 --name token-sink-container token-sink