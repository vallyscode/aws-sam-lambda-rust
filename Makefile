.PHONY: build
.PHONY: deploy
.PHONY: dev

build:
	rustup target add x86_64-unknown-linux-musl
	cargo build --target x86_64-unknown-linux-musl --release
	sam build

build-HeyFunction:
	cp ./target/x86_64-unknown-linux-musl/release/hey $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

build-ByeFunction:
	cp ./target/x86_64-unknown-linux-musl/release/bye $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

deploy:
	sam deploy --profile ${PROFILE} --config-env ${ENVIRONMENT} --no-fail-on-empty-changeset

dev:
	make build  deploy PROFILE=default ENVIRONMENT=dev REGION=eu-west-1