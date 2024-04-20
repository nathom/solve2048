.PHONY: all build server

all: server

server: build
	open http://localhost:8000
	python3 -m http.server --directory output

build:
	npm i
	wasm-pack build --target web
	rollup -c
	if [ ! -f output/index.html ]; then ln -s ../index.html output/index.html; fi
	if [ ! -d output/style ]; then ln -s ../style output/style; fi
	cp node_modules/solve2048/solve2048_bg.wasm output/solve2048_bg.wasm
	cd ..
