#!/bin/sh

docker run -d \
	--name eztutors-pgdb \
	-e POSTGRES_USER=ezt \
	-e POSTGRES_PASSWORD=ezpass \
	-e POSTGRES_DB=eztutors \
	-v ${PROJECT_ROOT}/pgdb:/var/lib/postgresql/data \
    -p 5435:5432 \
    postgres:13

