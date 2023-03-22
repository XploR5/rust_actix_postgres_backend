INSERT INTO testing.plantdata (plant_id, created_at, updated_at, planned_data, unplanned_data)
VALUES ($1, $2, $3, $4, $5)
RETURNING id, plant_id, created_at, updated_at, planned_data, unplanned_data;
