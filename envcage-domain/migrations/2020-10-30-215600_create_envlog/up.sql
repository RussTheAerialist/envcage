CREATE TABLE envlogs (
  id UUID PRIMARY KEY,
  device_id CHAR(12),
  created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  temperature NUMERIC(6, 2) NOT NULL,
  humidity NUMERIC(5, 2) NOT NULL,
  CONSTRAINT fk_device
    FOREIGN KEY(device_id) REFERENCES devices(id)
    ON DELETE SET null
)
