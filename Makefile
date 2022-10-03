#!/usr/bin/make -f
BUILDDIR ?= $(CURDIR)/build

initialize:
	rm -r $(BUILDDIR)/cyber-desmos
	bash ./scripts/init-desmos-chain.sh $(BUILDDIR)/cyber-desmos
	bash ./scripts/init-cyber-chain.sh $(BUILDDIR)/cyber-desmos

localnet-start: localnet-stop
	docker-compose up -d

localnet-stop:
	docker-compose down