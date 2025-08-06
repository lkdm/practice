
.PHONY: help

help: ## Display this help screen
	@echo
	@echo "Usage: make [target] ..."
	@echo
	@sed -n 's/^\([A-Za-z0-9_.-]*\):.*## \(.*\)$$/\t\1: \2/p' Makefile | sort | column -t -s ':'
	@echo

.PHONY: run

run: ## Develop the application
	@echo "Running the app"
	trunk serve --port 30400 --open
