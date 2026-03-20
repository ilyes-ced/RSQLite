CREATE DATABASE test_db;



/* TABLES */
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT,
    age INTEGER,
    balance FLOAT,
    is_active BOOLEAN
);
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER,
    title TEXT,
    content TEXT
);
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE,
    price FLOAT NOT NULL,
    stock INTEGER DEFAULT 0
);
CREATE TABLE logs (
    id INTEGER AUTO_INCREMENT,
    message TEXT,
    created_at TEXT
);



/* INSERT */
INSERT INTO users VALUES (1, 'Alice', 25, 100.5, true);
INSERT INTO users VALUES (2, 'Bob', 30, 200.0, false);
INSERT INTO users VALUES (3, 'Charlie', 35, 0.0, true);
INSERT INTO posts VALUES (1, 1, 'Hello', 'First post');
INSERT INTO posts VALUES (2, 1, 'Rust', 'Learning Rust SQL engine');
INSERT INTO posts VALUES (3, 2, 'Hi', 'Bob post');
INSERT INTO products VALUES (1, 'Laptop', 999.99, 10);
INSERT INTO products VALUES (2, 'Mouse', 25.5, 100);
INSERT INTO logs VALUES (1, 'System started', '2026-01-01');
INSERT INTO logs VALUES (2, 'User login', '2026-01-02');


/* SELECT */
SELECT * FROM users;
SELECT name, age FROM users;
SELECT id FROM posts;
SELECT * FROM products;
    /* SELECT  WHERE */
    SELECT * FROM users WHERE age > 25;
    SELECT * FROM users WHERE age >= 30;
    SELECT * FROM users WHERE balance = 0;
    SELECT * FROM users WHERE is_active = true;
    SELECT * FROM users WHERE is_active = false;
    SELECT * FROM products WHERE price > 50;
    SELECT * FROM products WHERE stock > 0;
    /* SELECT  NAD/NOT/OR */
    SELECT * FROM users WHERE age > 20 AND is_active = true;
    SELECT * FROM users WHERE age > 20 OR balance > 150;
    SELECT * FROM users WHERE NOT is_active = true;
    SELECT * FROM products WHERE price > 10 AND stock > 0;
    SELECT * FROM users WHERE NOT age = 25;



/* UPDATE */
UPDATE users SET age = 26 WHERE id = 1;
UPDATE users SET balance = 999.99 WHERE name = 'Bob';
UPDATE users SET is_active = false WHERE age > 30;
UPDATE products SET stock = 50 WHERE id = 1;
UPDATE products SET price = 19.99 WHERE name = 'Mouse';



/* DELETE */
DELETE FROM users WHERE id = 3;
DELETE FROM users WHERE age < 25;
DELETE FROM posts WHERE id = 2;
DELETE FROM products WHERE stock = 0;



/* DROP */
DROP TABLE logs;
DROP TABLE posts;



/* INSERT */
SELECT AVG(age) FROM users;
SELECT SUM(balance) FROM users;
SELECT MAX(age) FROM users;
SELECT MIN(age) FROM users;
SELECT AVG(price) FROM products;
SELECT SUM(stock) FROM products;



/* OPs */
SELECT 1 + 2;
SELECT 10 - 3;
SELECT 4 * 5;
SELECT 20 / 4;
SELECT 10 % 3;
SELECT (1 + 2) * 3;



/* COMPLEX WHERE */
SELECT * FROM users WHERE age > 20 AND balance > 50;
SELECT * FROM users WHERE age > 20 AND (balance > 50 OR is_active = true);
SELECT * FROM users WHERE NOT (age < 30);
SELECT * FROM products WHERE price > 10 AND stock > 0 AND name != 'Mouse';

/* CONSTRAINT TESTING */
INSERT INTO products VALUES (5, 'Laptop', 1000.0, 5);
INSERT INTO products VALUES (6, 'Laptop', 1200.0, 2);  -- UNIQUE violation
INSERT INTO products VALUES (7, 'Tablet', null, 5);    -- NOT NULL violation



/* Mixed stress queries */
UPDATE users SET balance = balance + 50 WHERE age > 20;
SELECT name FROM users WHERE balance > 100 AND age < 40;
DELETE FROM users WHERE balance = 0 AND is_active = false;
SELECT AVG(balance) FROM users WHERE is_active = true;



/* Parser edge cases */
SELECT name FROM users WHERE age>20;
SELECT name FROM users WHERE age >=20 AND balance<=100;
SELECT * FROM users WHERE name='Alice';
SELECT * FROM users WHERE name = 'Bob';
SELECT * FROM users WHERE name != 'Charlie';
SELECT * FROM users;;;;   -- repeated semicolons



/* Invalid / error cases */
SELECT FROM users;                    -- missing columns
INSERT INTO users VALUES (1);         -- wrong arity
UPDATE users SET;                     -- missing assignment
DELETE users WHERE id = 1;           -- missing FROM
CREATE TABLE;                        -- incomplete
SELECT * FROM;                        -- incomplete