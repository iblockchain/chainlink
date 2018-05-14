.DEFAULT_GOAL := build
.PHONY: dep build install docker dockerpush

REPO=smartcontract/chainlink
LDFLAGS=-ldflags "-X github.com/smartcontractkit/chainlink/store.Sha=`git rev-parse HEAD`"
#LDFLAGS=-ldflags "-X github.com/smartcontractkit/chainlink/store.Sha=`git rev-parse HEAD` -L $(dir $(LIBADAPTERS))"

ENVIRONMENT ?= release

LIBADAPTERS := ./sgx/target/$(ENVIRONMENT)/libadapters.so
LIBS := $(LIBADAPTERS)

dep: ## Ensure chainlink's go dependencies are installed.
	@dep ensure

build: $(LIBS) ## Build chainlink.
	ENVIRONMENT=$(ENVIRONMENT) go build $(LDFLAGS) -o chainlink

install: dep ## Install chainlink
	@go install $(LDFLAGS)

docker: ## Build the docker image.
	@docker build . -t $(REPO)

dockerpush: ## Push the docker image to dockerhub
	@docker push $(REPO)

$(LIBADAPTERS): sgx/**
	ENVIRONMENT=$(ENVIRONMENT) $(MAKE) -C ./sgx/

help:
	@echo ""
	@echo "         .__           .__       .__  .__        __"
	@echo "    ____ |  |__ _____  |__| ____ |  | |__| ____ |  | __"
	@echo "  _/ ___\|  |  \\\\\\__  \ |  |/    \|  | |  |/    \|  |/ /"
	@echo "  \  \___|   Y  \/ __ \|  |   |  \  |_|  |   |  \    <"
	@echo "   \___  >___|  (____  /__|___|  /____/__|___|  /__|_ \\"
	@echo "       \/     \/     \/        \/             \/     \/"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
