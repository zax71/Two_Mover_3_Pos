CREATE TABLE Lines (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  start_x FLOAT,
  start_y FLOAT,
  start_z FLOAT,
  end_x FLOAT,
  end_y FLOAT,
  end_z FLOAT,
  name varchar(255)
);

CREATE TABLE BezierCurves (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  start_x FLOAT,
  start_y FLOAT,
  start_z FLOAT,
  midpoint_x FLOAT,
  midpoint_y FLOAT,
  midpoint_z FLOAT,
  end_x FLOAT,
  end_y FLOAT,
  end_z FLOAT,
  name varchar(255)
);

CREATE TABLE CubicBezierCurves (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  start_x FLOAT,
  start_y FLOAT,
  start_z FLOAT,
  end_x FLOAT,
  end_y FLOAT,
  end_z FLOAT,
  handle_1_x FLOAT,
  handle_1_y FLOAT,
  handle_1_z FLOAT,
  handle_2_x FLOAT,
  handle_2_y FLOAT,
  handle_2_z FLOAT,
  name varchar(255)
);