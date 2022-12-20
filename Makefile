.PHONY: plan
plan:
	docker-compose exec db /bin/bash -ic "psqldef --dry-run < ./schema.sql"

.PHONY: migrate
migrate:
	docker-compose exec db /bin/bash -ic "psqldef < ./schema.sql"
