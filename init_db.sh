#!/bin/bash

sudo -u postgres psql -c "CREATE USER javadyakuza LOGIN CREATEDB SUPERUSER;"
sudo -u postgres psql -c "ALTER USER javadyakuza WITH PASSWORD 'fuckon';"

echo "created *javad* as super user"

psql postgres -U javadyakuza -c "CREATE DATABASE codeduel;"

echo "created *codeduel* database at local host"



