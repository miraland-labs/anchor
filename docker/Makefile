WORKDIR=$(PWD)
#
# Anchor version.
#
ANCHOR_CLI=v0.27.0
#
# Solana toolchain.
#
MIRALAND_CLI=v1.18.0
#
# Build version should match the Anchor cli version.
#
IMG_ORG ?= miralandlabs  # MI, vanilla: projectserum
IMG_VER ?= $(ANCHOR_CLI)

.PHONY: build build-push build-shell publish

default:

build: build/Dockerfile
	@docker build \
	--build-arg ANCHOR_CLI=$(ANCHOR_CLI) \
	--build-arg MIRALAND_CLI=$(MIRALAND_CLI) \
	$@ -t $(IMG_ORG)/$@:$(IMG_VER)

build-push:
	@docker push $(IMG_ORG)/build:$(IMG_VER)

build-shell:
	@docker run -ti --rm --net=host \
		-v $(WORKDIR)/..:/workdir \
		$(IMG_ORG)/build:$(IMG_VER) bash

publish: build build-push
