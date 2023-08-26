-- Add up migration script here
CREATE TABLE families
(
    family_id   SERIAL                 PRIMARY KEY,
    name        CHARACTER VARYING(32)  NOT NULL,
    pass_code   VARCHAR(32)            NOT NULL,
    created_at  TIMESTAMP              NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP              NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER families_timestamp_trg
  BEFORE UPDATE
  ON families
  FOR EACH ROW
  EXECUTE FUNCTION set_timestamp_trigger();

CREATE TABLE family_members
(
    family_member_id  VARCHAR(32)  PRIMARY KEY,
    family_id         INTEGER      NOT NULL  REFERENCES families(family_id),
    member_id         INTEGER      NOT NULL  REFERENCES members(member_id),
    created_at        TIMESTAMP    NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP    NOT NULL  DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER family_members_timestamp_trg
  BEFORE UPDATE
  ON family_members
  FOR EACH ROW
  EXECUTE FUNCTION set_timestamp_trigger();
  