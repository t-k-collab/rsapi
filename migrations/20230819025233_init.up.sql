-- Add up migration script here
CREATE TABLE members
(
    member_id     SERIAL       PRIMARY KEY,
    first_name    VARCHAR(32)  NOT NULL,
    middle_name   VARCHAR(32)  NOT NULL,
    family_name   VARCHAR(32)  NOT NULL,
    date_of_birth DATE, 
    email         VARCHAR(255) UNIQUE NOT NULL,
    password      VARCHAR(32)  NOT NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION set_timestamp_trigger()
  RETURNS TRIGGER
  LANGUAGE PLPGSQL
AS $$
BEGIN
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$;

CREATE TRIGGER timestamp_trg
  BEFORE UPDATE
  ON members
  FOR EACH ROW
  EXECUTE FUNCTION set_timestamp_trigger();

