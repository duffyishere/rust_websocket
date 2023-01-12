CREATE TABLE account
(
    id SERIAL PRIMARY KEY,
    email varchar not null unique,
    password text not null,
    name varchar not null
);

