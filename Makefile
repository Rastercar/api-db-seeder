# [PROD-TODO] remove me
.PHONY: lazy
lazy:
	git add . && git commit -m "." && git push origin master

.PHONY: run_dev
run_dev:
	cargo r