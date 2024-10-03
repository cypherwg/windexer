.PHONY: build test lint run docker-build docker-run

build:
	cargo build --release

test:
	cargo test

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

run:
	cargo run --release

docker-build:
	docker build -t cypher-windexer .

docker-run:
	docker run -p 8080:8080 cypher-windexer

deploy:
	scripts/deployment/deploy.sh

setup-monitoring:
	scripts/monitoring/setup_grafana.sh