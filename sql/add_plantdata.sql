INSERT INTO testing.plantdata (plant_id, created_at, updated_at, planned_data, unplanned_data)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, plant_id, created_at, updated_at, planned_data, unplanned_data;

-- INSERT INTO testing.plantdata (plant_id, created_at, updated_at, planned_data, unplanned_data)
-- VALUES (
--     (SELECT floor(random() * 9) + 1),
--     NOW()::TIMESTAMP,
--     NOW()::TIMESTAMP,
--     (SELECT floor(random() * 100) + 1),
--     (SELECT floor(random() * 100) + 1)
-- );