CREATE TABLE Lights (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  coordinate_x FLOAT,
  coordinate_y FLOAT,
  coordinate_z FLOAT,
  minimum_beam INTEGER,
  maximum_beam INTEGER,
  name varchar(255),
  address INTEGER
)