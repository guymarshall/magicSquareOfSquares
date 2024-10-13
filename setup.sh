#!/bin/bash

sudo apt update
sudo apt install mysql-server
sudo systemctl start mysql
sudo mysql_secure_installation
sudo mysql -u root -p # choose password here and enter into .env file

# CREATE DATABASE database_name;
# USE database_name;
# CREATE TABLE users (
#     id INT AUTO_INCREMENT PRIMARY KEY,
#     total INT,
#     count INT
# );
# ALTER TABLE totals ADD UNIQUE (total);
# ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'your_password';
# FLUSH PRIVILEGES;