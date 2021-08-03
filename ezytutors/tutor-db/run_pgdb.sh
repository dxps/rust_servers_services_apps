#!/bin/sh

docker run -d \
	--name eztutors-pgdb \
	-e POSTGRES_USER=ezt \
	-e POSTGRES_PASSWORD=ezpass \
	-e POSTGRES_DB=ezdb \
	-v ${PROJECT_ROOT}/pgdb:/var/lib/postgresql/data \
    -p 5432:5432 \
    postgres:13

