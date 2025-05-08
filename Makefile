run:
	cargo run

clean:
	rm -rf output
	rm -rf .nec_modules

.PHONY: run clean
