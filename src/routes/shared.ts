// Mirrors mypath-backend/src/shared/handlers.rs -> get_time_slots
import { TIME_SLOTS } from "../shared/time-slots.js";
import { json } from "../shared/http.js";

// GET /shared/slots  -> 200 (JSON array of the 10 hourly slot strings)
export function getSlots(): Response {
  return json(TIME_SLOTS);
}
