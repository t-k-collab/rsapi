-- Add down migration script here
DROP TABLE members;
DROP FUNCTION IF EXISTS set_timestamp_trigger();
