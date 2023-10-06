TAG ?= 0.1.0
IMAGE_NAME=myimages/largejson
BASEIMAGE=debian:bookworm-slim

ifeq ($(shell command -v podman 2> /dev/null),)
	DOCKER=docker
else
	DOCKER=podman
endif

.PHONY: build crds

all:
	@echo making everything

.ONESHELL:
docker-build:
	{ \
	$(DOCKER) build . -t $(IMAGE_NAME) -f Dockerfile --build-arg BASEIMAGE=$(BASEIMAGE); \
	$(DOCKER) image ls $(IMAGE_NAME); \
	}

docker-build-distroless: BASEIMAGE=gcr.io/distroless/cc-debian12
docker-build-distroless: IMAGE_NAME=polecatworks/largejson-distroless
docker-build-distroless: docker-build

cargo-doc:
	@cargo doc --no-deps --document-private-items
	@open target/doc/largejson/index.html
