run:
	npm run build; cargo leptos watch

build:
	cargo leptos build

deploy: build
	sudo systemctl restart temidara-rocks
