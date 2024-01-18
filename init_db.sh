#!/bin/bash
sudo -u postgres psql -c "ALTER USER javad WITH CREATEDB"

# Start the PostgreSQL server
sudo service postgresql start

# Create the "codeduel" database
createdb codeduel
