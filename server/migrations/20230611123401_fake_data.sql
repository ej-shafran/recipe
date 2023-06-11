-- User Data
INSERT INTO `user` VALUES 
  ('dd009d50-f2b3-4191-acdd-52ac86c7c3a5','evyatar','$2b$10$b70fGa5Yug3nNcuvT8HJVeF.F5H5yGgIo1rjzhSOekPO7ITUmFwMS');

-- Recipe Data
INSERT INTO recipe (title, content, poster_id) VALUES
  ('Recipe 1', 'Content for Recipe 1', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5'),
  ('Recipe 2', 'Content for Recipe 2', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5'),
  ('Recipe 3', 'Content for Recipe 3', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5'),
  ('Recipe 4', 'Content for Recipe 4', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5'),
  ('Recipe 5', 'Content for Recipe 5', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5');

-- Saves Data
INSERT INTO saves (user_id, recipe_id) VALUES
  ('dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 1),
  ('dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 3),
  ('dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 5);

-- Comment Data
INSERT INTO comment (content, user_id, recipe_id) VALUES
  ('Comment 1', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 1),
  ('Comment 2', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 2),
  ('Comment 3', 'dd009d50-f2b3-4191-acdd-52ac86c7c3a5', 4);

