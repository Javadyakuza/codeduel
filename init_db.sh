#!/bin/bash

sudo -u postgres psql -c "CREATE USER javadyakuza LOGIN CREATEDB SUPERUSER;"
sudo -u postgres psql -c "ALTER USER javadyakuza WITH PASSWORD 'fuckon';"

echo "created *javad* as super user"

psql postgres -U javadyakuza -c "CREATE DATABASE codeduel;"

echo "created *codeduel* database at local host"

echo "creating tables ..."

diesel migration run

echo "adding triggers ..."

psql -U javadyakuza -d codeduel -c "CREATE TRIGGER lowercase_values_trigger
BEFORE INSERT OR UPDATE ON ALL TABLES
FOR EACH ROW
EXECUTE PROCEDURE lowercase_values();
"
psql -U javadyakuza -d codeduel -c "CREATE OR REPLACE FUNCTION lowercase_values()
RETURNS TRIGGER AS $$
DECLARE
    lowercased_value TEXT;
BEGIN
    IF NEW.field_name IS NOT NULL AND (data_type(NEW.field_name) = 'VARCHAR' OR data_type(NEW.field_name) = 'TEXT') THEN
        lowercased_value := NEW.field_name;
        lowercased_value := LOWER(lowercased_value);
        NEW.field_name := lowercased_value;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

"





