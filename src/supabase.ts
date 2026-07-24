import { createClient, type SupabaseClient } from "@supabase/supabase-js";

export interface Env {
  // Cloudflare Static Assets binding — serves files from public/.
  ASSETS: Fetcher;
  // Supabase credentials (set as secrets).
  SUPABASE_URL: string;
  SUPABASE_SERVICE_ROLE_KEY: string;
  ADMIN_HASH: string;
}

// One client per request. The supabase-js client is cheap to construct and
// not safe to share across requests with different credentials, so we scope
// it to each fetch handler via a module-level singleton created on demand.
let _client: SupabaseClient | null = null;

export function getSupabase(env: Env): SupabaseClient {
  if (_client) return _client;
  _client = createClient(env.SUPABASE_URL, env.SUPABASE_SERVICE_ROLE_KEY, {
    auth: { persistSession: false, autoRefreshToken: false },
  });
  return _client;
}

/**
 * PostgREST returns HTTP 409 with code 23505 on a unique-constraint violation.
 * The Rust backend surfaces this as 409 Conflict (Err(StatusCode::CONFLICT)).
 */
export function isUniqueViolation(err: unknown): boolean {
  if (!err || typeof err !== "object") return false;
  const code = (err as { code?: string }).code;
  return code === "23505";
}
