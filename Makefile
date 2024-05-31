
build:
	wasm-pack build --target web

install: build
	cp static/index.html web/
	cp pkg/crypto_screener.js web/pkg/
	cp pkg/crypto_screener_bg.wasm web/pkg/
