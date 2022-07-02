
build:
	wasm-pack build --target web 
	cp -r ./pkg ./public/
