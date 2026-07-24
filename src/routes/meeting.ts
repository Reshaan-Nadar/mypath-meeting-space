// Mirrors mypath-backend/src/meeting/handlers.rs
import { getSupabase, isUniqueViolation, type Env } from "../supabase.js";
import { isTimeSlot } from "../shared/time-slots.js";
import { empty, json, parseJson, BadInput, handleError } from "../shared/http.js";

export interface MeetingBookingInfo {
  date: string;             // YYYY-MM-DD
  organizer_name: string;
  topic: string;
  contact: string;
  time_slot: string;        // one of TIME_SLOTS
  room_name: string;
  attendees: string;        // One..Eight
}

// POST /meeting/book/add  -> 201 | 409 | 500
export async function meetingBookAdd(req: Request, env: Env): Promise<Response> {
  let payload: MeetingBookingInfo;
  try {
    payload = await parseJson<MeetingBookingInfo>(req);
  } catch (e) {
    return handleError(e);
  }

  if (
    !payload.date ||
    !payload.organizer_name ||
    !payload.topic ||
    !payload.contact ||
    !payload.room_name ||
    !payload.attendees ||
    !isTimeSlot(payload.time_slot)
  ) {
    return empty(400);
  }

  const supabase = getSupabase(env);
  const { error } = await supabase.from("bookings").insert({
    booking_date: payload.date,
    time_slot: payload.time_slot,
    booking_type: "Meeting",
    contact: payload.contact,
    organizer_name: payload.organizer_name,
    topic: payload.topic,
    room_name: payload.room_name,
    attendees: payload.attendees,
  });

  if (error) {
    if (isUniqueViolation(error)) return empty(409); // slot already taken
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return empty(201);
}

// DELETE /meeting/book/remove/:id  -> 200 | 404 | 500
export async function meetingBookRemove(idParam: string, env: Env): Promise<Response> {
  const id = Number(idParam);
  if (!Number.isInteger(id)) return empty(400);

  const supabase = getSupabase(env);
  const { error, count } = await supabase
    .from("bookings")
    .delete({ count: "exact" })
    .eq("id", id)
    .eq("booking_type", "Meeting");

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return (count ?? 0) > 0 ? empty(200) : empty(404);
}

// GET /meeting/book/list  -> 200 (JSON)
export async function meetingBookList(env: Env): Promise<Response> {
  const supabase = getSupabase(env);
  const { data, error } = await supabase
    .from("bookings")
    .select("*")
    .eq("booking_type", "Meeting");

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return json(data);
}

// Guard so parseJson + BadInput stay referenced (kept for future POST bodies).
export const _types = { parseJson, BadInput };
