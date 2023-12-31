CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    display_name VARCHAR(32) NOT NULL,
    email VARCHAR(320) UNIQUE NOT NULL,
    password VARCHAR(256) NOT NULL,
    handle VARCHAR(20) UNIQUE NOT NULL,
    created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    profile_uri VARCHAR(64) UNIQUE NOT NULL,
    photo_uri VARCHAR(128) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS Individuals (
    id SERIAL PRIMARY KEY,
    last_name VARCHAR(64) NOT NULL,
    dob DATE NOT NULL,
    fk_user INTEGER REFERENCES Users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Organization (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    founded DATE NOT NULL,
    fk_user INTEGER REFERENCES Users(id) ON DELETE CASCADE
);

