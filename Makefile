
# The MIT License (MIT)
#
# Copyright (c) 2024 Greg Slocum, WBTek
# a division of WhiteBear Family, Inc.
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

WASM_PACK_DEBUG := wasm-pack build --target web --dev
WASM_PACK_RELEASE := wasm-pack build --target web --release

# OUTPUT_DIR can point to a symbolic link to a served location
# Output structure in my case is:
# ./web/index.html
# ./web/pkg/OtherStuff
OUTPUT_DIR := ./web
PKG_DIR := $(OUTPUT_DIR)/pkg

all: release install

debug:
	$(WASM_PACK_DEBUG)

release:
	$(WASM_PACK_RELEASE)

install:
	@test -d $(OUTPUT_DIR) || { echo "Error: Create a directory or symbolic link for output at '$(OUTPUT_DIR)'"; exit 1; }
	@test -d $(PKG_DIR) || mkdir $(PKG_DIR) || { echo "Error: couldn't make '$(PKG_DIR)'"; exit 1; }
	cp static/index.html $(OUTPUT_DIR)/
	cp static/styles.css $(OUTPUT_DIR)/
	cp pkg/crypto_screener_bg.wasm $(OUTPUT_DIR)/pkg/
	cp pkg/crypto_screener.js $(OUTPUT_DIR)/pkg/
	cp ./LICENSE $(OUTPUT_DIR)/pkg/

# Clean the target directory
clean:
	rm -f pkg/*
	cargo clean

# Generate fresh documentation
doc: clean
	cargo doc --no-deps --document-private-items
	# Copy documentation to the desired location, e.g., `docs`
	rm -rf docs/
	mkdir -p docs
	cp -r target/doc/* docs/
	# GitHub navigation aid
	ln -s ../index.html docs/index.html

# Shortcut to clean and then generate documentation
update-doc: clean doc

.PHONY: all build install clean doc

