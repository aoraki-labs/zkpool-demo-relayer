-- Remove the custom enum type definition
-- CREATE TYPE proof_status AS ENUM ('created', 'proving', 'proven', 'submitted');

CREATE TABLE big_proofs (
    id BIGSERIAL PRIMARY KEY,
    project_id VARCHAR NOT NULL UNIQUE,
    task_id VARCHAR NOT NULL UNIQUE,
    status VARCHAR NOT NULL DEFAULT 'created', -- Changed to VARCHAR
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP NOT NULL
);

CREATE TABLE small_proofs (
    id BIGSERIAL PRIMARY KEY,
    project_id VARCHAR NOT NULL,
    task_id VARCHAR NOT NULL,
    task_split_id VARCHAR NOT NULL,
    task_percentage FLOAT NOT NULL CHECK (task_percentage >= 0 AND task_percentage <= 1),
    status VARCHAR NOT NULL DEFAULT 'created', -- Changed to VARCHAR
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP NOT NULL,
    FOREIGN KEY (task_id) REFERENCES big_proofs (task_id),
    FOREIGN KEY (project_id) REFERENCES big_proofs (project_id) 
);

