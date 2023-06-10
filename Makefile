SHELL=/bin/bash

venv: requirements.txt ## Set up virtual environment
	python3 -m venv venv
	venv/bin/pip install -r requirements.txt

install: venv
	unset CONDA_PREFIX && \
	source venv/bin/activate && maturin develop -m Cargo.toml

install-release: venv
	unset CONDA_PREFIX && \
	source venv/bin/activate && maturin develop --release -m Cargo.toml

clean:
	-@rm -r venv
	-@cargo clean

build:
	docker run --rm -v $(PWD):/io ghcr.io/pyo3/maturin build --release -i python3.8
	docker run --rm -v $(PWD):/io ghcr.io/pyo3/maturin build --release -i python3.9
	docker run --rm -v $(PWD):/io ghcr.io/pyo3/maturin build --release -i python3.10

mv-wheel:
	cp -u target/wheels/*.whl ../../app/lib
