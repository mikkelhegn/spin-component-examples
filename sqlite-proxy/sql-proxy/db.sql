CREATE TABLE IF NOT EXISTS customers
(
    id INTEGER PRIMARY KEY,
    name TEXT,
    email TEXT
);

INSERT INTO customers(id, name, email)
VALUES(1, "Mikkel Mork Hegnhoj", "mikkel@fermyon.com");