.PHONY: test check clean build dist all

clean:
	@cargo clean

run:
	cargo run

doc:
	@cargo doc --no-deps

docOpen:
	@cargo doc -q --no-deps --open

testList: testBuild
	@cargo test -q -- --list

testBuild:
	@cargo test -q --no-run

testAll: testBuild
	$(info -> can use as: cargo test --test <foo>)
	@cargo test --tests -- --show-output

testIgnore: testBuild
	$(info -> run with test method as: #[ignore])
	@cargo test -- --ignored --show-output

testLib:
	@cargo test --lib -- --show-output

testBenches:
	$(info -> can use as: cargo test --benche <foo>)
	@cargo test --benches -- --show-output

testBins:
	$(info -> can use as: cargo test --bin <foo>)
	@cargo test --bins -- --show-output

testExamples:
	$(info -> can use as: cargo test --example <foo>)
	@cargo test --examples -- --show-output

testDoc:
	@cargo test --doc -- --show-output

bench:
	@cargo bench -q

benchBenches:
	@cargo bench -q --benches

benchLib:
	@cargo bench -q --lib