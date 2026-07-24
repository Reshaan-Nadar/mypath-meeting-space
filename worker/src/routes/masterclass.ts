// Mirrors mypath-backend/src/masterclass/handlers.rs
import { getSupabase, type Env } from "../supabase.js";
import { empty, json, parseJson, handleError } from "../shared/http.js";

export interface MasterClassCreatePayload {
  title: string;
  description: string;
  timing: string;
  date: string;
  presenter_name: string;
  admin_hash: string;
}
export interface MasterClassEnrollPayload {
  master_class_id: number;
  first_name: string;
  last_name: string;
  contact: string;
}
export interface MasterClassEnquirePayload {
  master_class_id: number;
  contact: string;
  message: string;
}

// POST /masterclass/create  -> 201 | 401 | 500
export async function masterclassCreate(req: Request, env: Env): Promise<Response> {
  let payload: MasterClassCreatePayload;
  try {
    payload = await parseJson<MasterClassCreatePayload>(req);
  } catch (e) {
    return handleError(e);
  }

  if (payload.admin_hash !== env.ADMIN_HASH) return empty(401);

  const supabase = getSupabase(env);
  const { error } = await supabase.from("master_classes").insert({
    title: payload.title,
    description: payload.description,
    timing: payload.timing,
    date: payload.date,
    presenter_name: payload.presenter_name,
  });

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return empty(201);
}

// GET /masterclass/list  -> 200 (JSON)
export async function masterclassList(env: Env): Promise<Response> {
  const supabase = getSupabase(env);
  const { data, error } = await supabase.from("master_classes").select("*");
  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return json(data);
}

// POST /masterclass/enroll  -> 201 | 500
export async function masterclassEnroll(req: Request, env: Env): Promise<Response> {
  let payload: MasterClassEnrollPayload;
  try {
    payload = await parseJson<MasterClassEnrollPayload>(req);
  } catch (e) {
    return handleError(e);
  }

  const supabase = getSupabase(env);
  const { error } = await supabase.from("master_class_enrollments").insert({
    master_class_id: payload.master_class_id,
    first_name: payload.first_name,
    last_name: payload.last_name,
    contact: payload.contact,
  });

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return empty(201);
}

// POST /masterclass/enquire  -> 201 | 500
export async function masterclassEnquire(req: Request, env: Env): Promise<Response> {
  let payload: MasterClassEnquirePayload;
  try {
    payload = await parseJson<MasterClassEnquirePayload>(req);
  } catch (e) {
    return handleError(e);
  }

  const supabase = getSupabase(env);
  const { error } = await supabase.from("master_class_enquiries").insert({
    master_class_id: payload.master_class_id,
    contact: payload.contact,
    message: payload.message,
  });

  if (error) {
    console.error("❌ DB Error", error);
    return empty(500);
  }
  return empty(201);
}
