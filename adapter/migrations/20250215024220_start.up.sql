CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS '
  BEGIN
    new.updated_at = NOW();
    RETURN new;
  END;
' LANGUAGE 'plpgsql';

