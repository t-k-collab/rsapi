-- Add down migration script here
DROP TRIGGER families_timestamp_trg ON families;
DROP TRIGGER family_members_timestamp_trg ON family_members;
DROP TABLE family_members;
DROP TABLE families;
