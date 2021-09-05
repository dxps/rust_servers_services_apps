DROP TABLE IF EXISTS ezyweb_users;

CREATE TABLE ezyweb_users
(
    username     VARCHAR(20) PRIMARY KEY,
    password     CHAR(100) NOT NULL,
    tutor_id     INT
);
