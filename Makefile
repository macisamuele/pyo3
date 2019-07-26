.PHONY: test test_py3 publish clippy

test:
	cargo test
	${MAKE} clippy
	tox
	for example in examples/*; do tox -e py -c $$example/tox.ini; done

test_py3:
	tox -e py3
	for example in examples/*; do tox -e py3 -c $$example/tox.ini; done

clippy:
	cargo clippy --all-targets

publish: test
	cargo publish --manifest-path pyo3-derive-backend/Cargo.toml
	cargo publish --manifest-path pyo3cls/Cargo.toml
	cargo publish
