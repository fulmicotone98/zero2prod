-- migrations/{timestamp}_create_subscriptions_table.sql

-- Create subscriptions Table

-- Enforcing email uniqueness at DB level
-- Enforcing all fields are not null

CREATE TABLE subscriptions ( 
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	subscribed_at timestamptz NOT NULL
);

