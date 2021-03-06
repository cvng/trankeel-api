DROP VIEW IF EXISTS reports;
DROP FUNCTION IF EXISTS generate_summary_reports(current_month TIMESTAMPTZ);
DROP FUNCTION IF EXISTS get_expected_rents(current_month TIMESTAMPTZ);
DROP TYPE IF EXISTS summary_reports;
DROP TYPE IF EXISTS expected_rents;
