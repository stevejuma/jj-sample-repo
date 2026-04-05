RESET_BASE_REF := reset-base
LOCAL_ONLY_BRANCH := local-only/scratch-pad
RESET_HELPER_BRANCHES := work/reset-support work/reset-refresh work/reset-docs work/reset-finish work/reset-history-clean
REMOTE_BRANCHES := \
	feature/bootstrap-cleanup \
	feature/cloud-runner \
	feature/cloud-observability \
	feature/query-filtering \
	feature/export-json \
	feature/readme-polish

.PHONY: reset reset-git reset-jj show-state

reset: reset-git reset-jj
	@echo "Reset complete."

reset-git:
	@git fetch origin --prune --tags --force
	@git rev-parse --verify $(RESET_BASE_REF) >/dev/null
	@git checkout trunk
	@git reset --hard $(RESET_BASE_REF)
	@git clean -fd -e .jj/
	@for branch in $(REMOTE_BRANCHES); do \
		git branch -f $$branch origin/$$branch >/dev/null; \
	done
	@git branch -D $(LOCAL_ONLY_BRANCH) >/dev/null 2>&1 || true
	@for branch in $(RESET_HELPER_BRANCHES); do \
		git branch -D $$branch >/dev/null 2>&1 || true; \
	done
	@git branch $(LOCAL_ONLY_BRANCH) trunk >/dev/null
	@printf "Git state reset to %s with local-only branch %s.\n" \
		"$$(git rev-parse --short $(RESET_BASE_REF))" \
		"$(LOCAL_ONLY_BRANCH)"

reset-jj:
	@if command -v jj >/dev/null 2>&1; then \
		if [ ! -d .jj ]; then \
			jj git init --git-repo . >/dev/null; \
		fi; \
		jj git import >/dev/null; \
		echo "JJ state refreshed."; \
	else \
		echo "JJ not installed; skipped JJ reset."; \
	fi

show-state:
	@echo "Git:"
	@git status --short --branch
	@printf "\nBranches:\n"
	@git branch -vv
	@if command -v jj >/dev/null 2>&1 && [ -d .jj ]; then \
		printf "\nJJ:\n"; \
		jj status; \
	fi
