#!/bin/sh

docker run -d \
	--name dev-pgdb \
	-e POSTGRES_PASSWORD=pass123 \
	-v ${PROJECT_ROOT}/pgdb_data:/var/lib/postgresql/data \
    -p 5432:5432 \
    postgres

