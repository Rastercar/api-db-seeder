# [PROD-TODO] remove me
.PHONY: lazy
lazy:
	git add . && git commit -m "." && git push origin master

.PHONY: run_dev
run_dev:
	cargo r

.PHONY: run_debug
run_debug:
	RUST_LOG=debug cargo r