#!/bin/bash

sudo -u postgres psql -c "CREATE USER javadyakuza LOGIN CREATEDB SUPERUSER;" > /dev/null
sudo -u postgres psql -c "ALTER USER javadyakuza WITH PASSWORD 'fuckon';" > /dev/null

echo "Created superuser ✔"

psql postgres -U javadyakuza -c "CREATE DATABASE codeduel;" > /dev/null

echo "Created database ✔"

diesel migration run > /dev/null

echo "Created relations ✔"






