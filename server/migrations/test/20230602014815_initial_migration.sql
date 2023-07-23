-- User Table
CREATE TABLE `user` (
  id varchar(36) PRIMARY KEY,
  username varchar(100) UNIQUE KEY,
  password varchar(150) NOT NULL
);

-- Recipe Table
CREATE TABLE `recipe` (
  id int PRIMARY KEY AUTO_INCREMENT,
  title varchar(240) NOT NULL,
  content text NOT NULL,
  user_id varchar(36) NOT NULL,
  created_at timestamp DEFAULT CURRENT_TIMESTAMP,
  -- Recipe >- Many:One -> User (Poster)
  FOREIGN KEY(user_id) REFERENCES `user`(id)
);

-- Recipe >- Many:Many -< User (Saved)
CREATE TABLE `saves` (
  id int PRIMARY KEY AUTO_INCREMENT,
  user_id varchar(36) NOT NULL,
  recipe_id int NOT NULL,
  saved_at timestamp DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES `user`(id),
  FOREIGN KEY(recipe_id) REFERENCES `recipe`(id)
);

-- Comment Table
CREATE TABLE `comment` (
  id int PRIMARY KEY AUTO_INCREMENT,
  content text NOT NULL,
  user_id varchar(36) NOT NULL,
  recipe_id int NOT NULL,
  posted_at timestamp DEFAULT CURRENT_TIMESTAMP,
  -- Comment >- Many:One -> User (Poster)
  FOREIGN KEY(user_id) REFERENCES user(id),
  -- Comment >- Many:One -> Recipe
  FOREIGN KEY(recipe_id) REFERENCES `recipe`(id)
);
