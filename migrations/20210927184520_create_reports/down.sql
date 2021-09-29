DROP VIEW IF EXISTS reports;
DROP FUNCTION IF EXISTS generate_summary_reports(current_month TIMESTAMPTZ);
DROP FUNCTION IF EXISTS variation(x NUMERIC, y NUMERIC);
DROP TYPE IF EXISTS summary_reports CASCADE;
