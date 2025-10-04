FROM postgres:latest

ENV POSTGRES_PASSWORD docker
ENV POSTGRES_PORT 5432
ENV POSTGRES_DB deutschland_sammeln_datenbank
COPY ./DB_Deployer.sql /docker-entrypoint-initdb.d/
USER postgres