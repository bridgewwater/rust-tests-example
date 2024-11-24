.PHONY: test check clean build dist all

.PHONY: clean
clean:
	@cargo clean

.PHONY: depCheck
depCheck:
	@cargo check --all --bins --examples --tests

.PHONY: dep
dep: depCheck

depLintFmt:
	cargo fmt --all -- --check

depLintFmtEmitAllFiles:
	cargo fmt --all -- --emit files
	$(info can change to: cargo fmt -- --emit files|stdout)

depLintChecksPackageMistakes:
	cargo clippy -- -D warnings

style: depLintFmt depLintChecksPackageMistakes

.PHONY: run
run:
	cargo run

.PHONY: doc
doc:
	@cargo doc --no-deps

.PHONY: docOpen
docOpen:
	@cargo doc -q --no-deps --open

.PHONY: testList
testList: testBuild
	@cargo test -q -- --list

.PHONY: testBuild
testBuild:
	@cargo test -q --no-run

.PHONY: testAll
testAll: testBuild
	$(info -> can use as: cargo test --test <foo>)
	@cargo test --tests -- --show-output

.PHONY: testIgnore
testIgnore: testBuild
	$(info -> run with test method as: #[ignore])
	@cargo test -- --ignored --show-output

.PHONY: testLib
testLib:
	@cargo test --lib -- --show-output

.PHONY: testBenches
testBenches:
	$(info -> can use as: cargo test --benche <foo>)
	@cargo test --benches -- --show-output

.PHONY: testBins
testBins:
	$(info -> can use as: cargo test --bin <foo>)
	@cargo test --bins -- --show-output

.PHONY: testExamples
testExamples:
	$(info -> can use as: cargo test --example <foo>)
	@cargo test --examples -- --show-output

.PHONY: testDoc
testDoc:
	@cargo test --doc -- --show-output

.PHONY: bench
bench:
	@cargo bench -q

.PHONY: benchBenches
benchBenches:
	@cargo bench -q --benches

.PHONY: benchLib
benchLib:
	@cargo bench -q --lib

.PHONY: ci
ci: style testAll

.PHONY: helpProjectRoot
helpProjectRoot:
	@echo "Help: Project root Makefile"
	@echo "~> make init                 - check base env of this project"
	@echo "~> make dep                  - check and install"
	@echo "~> make clean                - remove build binary file, log files, and testdata"
	@echo ""
	@echo "~> make testAll              - run test fast"
	@echo "~> make ci                   - run CI tasks"
	@echo ""

.PHONY: help
help: helpProjectRoot