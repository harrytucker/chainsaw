# ================================================
# Chainsaw
# ================================================

## tools: installs development tools through cargo
.PHONY: tools
tools:
	cargo install cargo-nextest grpc-build

## grpc: generates grpc code from protobufs
grpc:
	grpc_build build \
		--in-dir chainsaw-proto/proto/helloworld.proto \
		--out-dir chainsaw-proto/src/grpc \
		-f -s -c

## test: runs tests for whole workspace
test:
	cargo nextest run
