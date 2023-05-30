CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  email varchar(255) NOT NULL,
  password varchar(255) NOT NULL,
  first_name varchar(255) NOT NULL,
  last_name varchar(255) NOT NULL,
  hcp real NOT NULL,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW(),
  CONSTRAINT unique_user_email UNIQUE (email)
);

CREATE TABLE courses (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name varchar(255) NOT NULL,
  red_cr real NOT NULL,
  yellow_cr real NOT NULL,
  blue_cr real,
  white_cr real,
  slope_red int NOT NULL,
  slope_yellow int NOT NULL,
  slope_blue int,
  slope_white int,
  CONSTRAINT unique_course_name UNIQUE (name)
);

CREATE TABLE holes (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  course_id uuid NOT NULL REFERENCES courses(id),
  number int NOT NULL,
  par int NOT NULL,
  distance_red int NOT NULL,
  distance_yellow int NOT NULL,
  distance_blue int,
  distance_white int,
  CONSTRAINT unique_hole_number UNIQUE (course_id, number)
);

CREATE TABLE rounds (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  course_id uuid NOT NULL REFERENCES courses(id),
  user_id uuid NOT NULL REFERENCES users(id),
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE scores (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  round_id uuid NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
  hole_id uuid NOT NULL REFERENCES holes(id),
  strokes int NOT NULL,
  CONSTRAINT unique_score_hole UNIQUE (round_id, hole_id)
);
