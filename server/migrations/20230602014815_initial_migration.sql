-- Add migration script here
CREATE TABLE `user` (
  id varchar(36) PRIMARY KEY,
  username varchar(100),
  password varchar(100)
);

CREATE TABLE `recipe` (
  id int PRIMARY KEY AUTO_INCREMENT,
  title varchar(240),
  content text,
  poster_id varchar(36),
  FOREIGN KEY(poster_id) REFERENCES `user`(id)
);

CREATE TABLE `comment` (
  id int PRIMARY KEY AUTO_INCREMENT,
  content text,
  recipe_id int,
  FOREIGN KEY(recipe_id) REFERENCES `recipe`(id)
);
