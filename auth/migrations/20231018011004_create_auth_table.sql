CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY NOT NULL ,
    username VARCHAR(20) UNIQUE NOT NULL,
    password VARCHAR(30) NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

