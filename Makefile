.PHONY: test check clean build dist all

.PHONY: clean
clean:
	@cargo clean

.PHONY: clean.out.tarpaulin
clean.out.tarpaulin:
	-@$(RM) -f cobertura.xml
	-@$(RM) -f tarpaulin-report.json
	-@$(RM) -f tarpaulin-report.html
	-@$(RM) -f lcov.info

.PHONY: depCheck
depCheck:
	@cargo check --all --bins --examples --tests

.PHONY: dep
dep: depCheck

.PHONY: dep.lint.fmt
dep.lint.fmt:
	cargo fmt --all -- --check

.PHONY: dep.lint.fmt.emit.all.files
dep.lint.fmt.emit.all.files:
	cargo fmt --all -- --emit files
	$(info can change to: cargo fmt -- --emit files|stdout)

.PHONY: dep.lint.check.package.mistakes
dep.lint.check.package.mistakes:
	cargo clippy -- -D warnings

.PHONY: style
style: dep.lint.fmt dep.lint.check.package.mistakes

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

.PHONY: test.coverage.html
test.coverage.html:
	$(info use: https://github.com/xd009642/tarpaulin)
	$(info if run error install as: cargo install cargo-tarpaulin)
	cargo tarpaulin --all-features --out Html

.PHONY: test.coverage.stdout
test.coverage.stdout:
	$(info use: https://github.com/xd009642/tarpaulin)
	$(info if run error install as: cargo install cargo-tarpaulin)
	cargo tarpaulin --all-features --out Stdout

.PHONY: ci.coverage
ci.coverage: ci test.coverage.stdout

.PHONY: all
all: dep ci ci.coverage clean

.PHONY: helpProjectRoot
helpProjectRoot:
	@echo "Help: Project root Makefile"
	@echo "~> make init                 - check base env of this project"
	@echo "~> make dep                  - check and install"
	@echo "~> make clean                - remove build binary file, log files, and testdata"
	@echo ""
	@echo "~> make testAll              - run test fast"
	@echo "~> make ci                   - run CI tasks"
	@echo "~> make ci.coverage          - run CI coverage"
	@echo ""
	@echo "~> make all                  - run commonly used"
	@echo ""

.PHONY: help
help: helpProjectRoot