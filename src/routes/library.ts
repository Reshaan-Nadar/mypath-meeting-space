// Mirrors mypath-backend/src/library/handlers.rs
import { getSupabase, isUniqueViolation, type Env } from "../supabase.js";
import { isTimeSlot } from "../shared/time-slots.js";
import { empty, json, parseJson, handleError } from "../shared/http.js";

export interface LibraryBookingInfo {
  date: string;             // YYYY-MM-DD
  first_name: string;
  last_name: string;
  contact: string;
  time_slot: string;        // one of TIME_SLOTS
  attendees: string;        // One..Four
}

// POST /library/book/add  -> 201 | 409 | 500
export async function libraryBookAdd(req: Request, env: Env): Promise<Response> {
  let payload: LibraryBookingInfo;
  try {
    payload = await parseJson<LibraryBookingInfo>(req);
  } catch (e) {
    return handleError(e);
  }

  if (
    !payload.date ||
    !payload.first_name ||
    !payload.last_name ||
    !payload.contact ||
    !payload.attendees ||
    !isTimeSlot(payload.time_slot)
  ) {
    return empty(400);
  }

  const supabase = getSupabase(env);
  const { error } = await supabase.from("bookings").insert({
    booking_date: payload.date,
    time_slot: payload.time_slot,
    booking_type: "Library",
    contact: payload.contact,
    first_name: payload.first_name,
    last_name: payload.last_name,
    attendees: payload.attendees,
  });

  if (error) {
    if (isUniqueViolation(error)) return empty(409);
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return empty(201);
}

// DELETE /library/book/remove/:id  -> 200 | 404 | 500
export async function libraryBookRemove(idParam: string, env: Env): Promise<Response> {
  const id = Number(idParam);
  if (!Number.isInteger(id)) return empty(400);

  const supabase = getSupabase(env);
  const { error, count } = await supabase
    .from("bookings")
    .delete({ count: "exact" })
    .eq("id", id)
    .eq("booking_type", "Library");

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return (count ?? 0) > 0 ? empty(200) : empty(404);
}

// GET /library/book/list  -> 200 (JSON)
export async function libraryBookList(env: Env): Promise<Response> {
  const supabase = getSupabase(env);
  const { data, error } = await supabase
    .from("bookings")
    .select("*")
    .eq("booking_type", "Library");

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return json(data);
}
