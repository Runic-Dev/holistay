-- Add down migration script here
DELETE FROM settings WHERE setting = 'default_user';
