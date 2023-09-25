VERSION=1.0
NAME=rust-makefile
EXEC=rust-exec
PREFIX=$(HOME)/.local

default: build_release

clean:
	rm -rf target/*
	cargo clean

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

dist: release
	@if [ ! -d ./pkg ]; \
	then \
		mkdir ./pkg; \
	fi

	@if [ -d ./pkg/$(NAME)-$(VERSION) ]; \
	then \
		echo "Current version number already exists! Removing old files!"; \
		rm -rf ./pkg/$(NAME)-$(VERSION); \
	fi

	@mkdir ./pkg/$(NAME)-$(VERSION)

	@cp ./dist-scripts/install.sh ./pkg/$(NAME)-$(VERSION)/

	@sed -i 's#{prefix}#$(PREFIX)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{version}#$(VERSION)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{name}#$(NAME)#g' ./pkg/$(NAME)-$(VERSION)/install.sh
	@sed -i 's#{exec}#$(EXEC)#g' ./pkg/$(NAME)-$(VERSION)/install.sh

	@mkdir ./pkg/$(NAME)-$(VERSION)/files
	@cp target/release/$(EXEC) ./pkg/$(NAME)-$(VERSION)/files/
	@strip ./pkg/$(NAME)-$(VERSION)/files/$(EXEC)

	@cp LICENSE ./pkg/$(NAME)-$(VERSION)/

	@cd ./pkg && tar -czf ./$(NAME)-$(VERSION).tar.gz ./$(NAME)-$(VERSION)
	@echo "Cleaning up"
	@rm -rf ./pkg/$(NAME)-$(VERSION)

create_container:
	if [-f ./docker-compose.yml]; then \
		docker compose up -d --build \
	fi

deploy_container:
	# TODO:
	





