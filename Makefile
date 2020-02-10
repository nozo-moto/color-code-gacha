CARGO_PATH := $(HOME)/.cargo

insatll:
	 cargo install --force --root $(CARGO_PATH) --path .
