# Makefile for sz-sdk-rust

# Detect the operating system and architecture.

include makefiles/osdetect.mk

# -----------------------------------------------------------------------------
# Variables
# -----------------------------------------------------------------------------

PROGRAM_NAME := $(shell basename `git rev-parse --show-toplevel`)
MAKEFILE_PATH := $(abspath $(firstword $(MAKEFILE_LIST)))
MAKEFILE_DIRECTORY := $(shell dirname $(MAKEFILE_PATH))
BUILD_VERSION := $(shell git describe --always --tags --abbrev=0 --dirty 2>/dev/null || echo "0.0.0")

.EXPORT_ALL_VARIABLES:

# -----------------------------------------------------------------------------
# The first "make" target runs as default.
# -----------------------------------------------------------------------------

.PHONY: default
default: help

# -----------------------------------------------------------------------------
# Operating System / Architecture targets
# -----------------------------------------------------------------------------

# -include makefiles/$(OSTYPE).mk
# -include makefiles/$(OSTYPE)_$(OSARCH).mk

# -----------------------------------------------------------------------------
# Dependency management
# -----------------------------------------------------------------------------

.PHONY: dependencies
dependencies:
	@cargo update

# -----------------------------------------------------------------------------
# Setup - start a Senzing gRPC server for testing
# -----------------------------------------------------------------------------

.PHONY: setup
setup:
	$(info No setup needed.)

# -----------------------------------------------------------------------------
# Lint
# -----------------------------------------------------------------------------

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: fmt-check
fmt-check:
	@cargo fmt -- --check

# -----------------------------------------------------------------------------
# Build
# -----------------------------------------------------------------------------

.PHONY: build
build:
	@cargo build

# -----------------------------------------------------------------------------
# Run
# -----------------------------------------------------------------------------

# .PHONY: run
# run: run-osarch-specific

# -----------------------------------------------------------------------------
# Test
# -----------------------------------------------------------------------------

.PHONY: test
test:
	@cargo test -- --show-output

# -----------------------------------------------------------------------------
# Coverage
# -----------------------------------------------------------------------------

# .PHONY: coverage
# coverage: coverage-osarch-specific

# -----------------------------------------------------------------------------
# Documentation
# -----------------------------------------------------------------------------

# .PHONY: documentation
# documentation: documentation-osarch-specific

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	$(info No cleaning needed.)

# -----------------------------------------------------------------------------
# Utility targets
# -----------------------------------------------------------------------------

.PHONY: help
help:
	$(info Build $(PROGRAM_NAME) version $(BUILD_VERSION))
	$(info Makefile targets:)
	@$(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$$@$$' | xargs

.PHONY: print-make-variables
print-make-variables:
	@$(foreach V,$(sort $(.VARIABLES)), \
		$(if $(filter-out environment% default automatic, \
		$(origin $V)),$(info $V=$($V) ($(value $V)))))

# -----------------------------------------------------------------------------
# Specific programs
# -----------------------------------------------------------------------------

.PHONY: cspell
cspell:
	@cspell lint --dot .
