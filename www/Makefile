.PHONY: build server
build:
	rollup -c
	if [ ! -f output/index.html ]; then ln -s ../index.html output/index.html; fi
	if [ ! -d output/style ]; then ln -s ../style output/style; fi
	cp node_modules/solve2048/solve2048_bg.wasm output/solve2048_bg.wasm
	cd ..

server: build
	python3 -m http.server --directory output
