// Small HTTP helpers — keeps route handlers terse and consistent.

export const json = (body: unknown, status = 200): Response =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });

export const empty = (status: number): Response => new Response(null, { status });

/** Pull a typed object out of the request body, rejecting bad JSON. */
export async function parseJson<T = unknown>(req: Request): Promise<T> {
  try {
    return (await req.json()) as T;
  } catch {
    throw new BadInput("Invalid JSON body");
  }
}

export class BadInput extends Error {}

/**
 * Maps app-level errors to Responses. Anything unknown becomes 500,
 * matching the Rust handlers' `Err(StatusCode::INTERNAL_SERVER_ERROR)` fallback.
 */
export function handleError(err: unknown): Response {
  if (err instanceof BadInput) return empty(400);
  console.error("❌ unhandled error", err);
  return empty(500);
}
