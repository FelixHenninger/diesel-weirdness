CREATE TABLE nested (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR(255),
  --
  -- Nested chained structure
  parent_id INTEGER,
  -- Attached metadata
  meta_a_id INTEGER,
  meta_b_id INTEGER,
  FOREIGN KEY(parent_id) REFERENCES nested(id),
  FOREIGN KEY(meta_a_id) REFERENCES meta_a(id),
  FOREIGN KEY(meta_b_id) REFERENCES meta_b(id)
);

CREATE TABLE meta_a (
  id INTEGER PRIMARY KEY NOT NULL,
  data VARCHAR(255) NOT NULL
);

CREATE TABLE meta_b (
  id INTEGER PRIMARY KEY NOT NULL,
  data VARCHAR(255) NOT NULL
);

INSERT INTO meta_a (id, data) VALUES
  (1, 'meta_a 1');

INSERT INTO meta_b (id, data) VALUES
  (1, 'meta_b 1');

INSERT INTO nested (id, name, parent_id, meta_a_id, meta_b_id) VALUES
  (1, 'nested_item 1', NULL, 1, NULL),
  (2, 'nested_item 2',    1, NULL, 1);
