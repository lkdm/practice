-- Users
INSERT INTO user (id, created_date, modified_date, email, tz)
VALUES
    ('5be7adab-3ba7-4bd5-977d-e1fd1a4a116e', '2024-07-13T09:00:00', '2024-07-13T09:00:00', 'alice@example.com', 'UTC');

INSERT INTO user (id, created_date, modified_date, email, backup_email, tz)
VALUES
    ('0b5e42b2-6989-41b1-8e0d-1e23456a7af3', '2024-07-15T11:05:00', '2024-07-15T11:05:00', 'bob@example.com', 'bob.alt@example.com', 'Australia/Sydney');

-- Profiles
INSERT INTO profile (id, created_date, modified_date, display_name, user_id)
VALUES
    ('1811ba39-768a-41ff-b842-4a78c770769b', '2024-07-13T09:05:00', '2024-07-13T09:05:00', 'Alice Wonder', '5be7adab-3ba7-4bd5-977d-e1fd1a4a116e');

INSERT INTO profile (id, created_date, modified_date, display_name, user_id)
VALUES
    ('79142730-2aaf-43f0-a7af-4de4b657e2e7', '2024-07-15T11:06:00', '2024-07-15T11:06:00', 'Bob Builder', '0b5e42b2-6989-41b1-8e0d-1e23456a7af3');
