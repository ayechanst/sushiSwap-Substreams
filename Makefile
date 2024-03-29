ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_weth_transfers -s 18532170 -t +5

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml


# a block for map_pools_created with a known pool creation
# 	substreams run -e $(ENDPOINT) substreams.yaml map_weth_pools -s 18532170 -t +250

# more recent:
# 	substreams run -e $(ENDPOINT) substreams.yaml map_weth_pools -s 18971237 -t +250
