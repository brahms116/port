
build:
	wasm-pack build --target web 
	mv ./pkg ./public/pkg
