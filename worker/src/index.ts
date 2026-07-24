// mypath — Cloudflare Worker entry point.
// Serves the API (the 10 routes the Rust backend exposed) and falls through
// to Workers Static Assets for the frontend (public/).
//
// Routing is deliberately hand-rolled (no framework) to mirror the axum
// router in mypath-backend/src/main.rs one-to-one.
import type { Env } from "./supabase.js";
import { empty, handleError } from "./shared/http.js";
import { meetingBookAdd, meetingBookRemove, meetingBookList } from "./routes/meeting.js";
import { libraryBookAdd, libraryBookRemove, libraryBookList } from "./routes/library.js";
import { masterclassCreate, masterclassList, masterclassEnroll, masterclassEnquire } from "./routes/masterclass.js";
import { getSlots } from "./routes/shared.js";

// Permissive CORS — matches the Rust CorsLayer (allow_origin(Any) ...).
const CORS = [
  ["Access-Control-Allow-Origin", "*"],
  ["Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS"],
  ["Access-Control-Allow-Headers", "*"],
];

function withCors(res: Response): Response {
  const clone = new Response(res.body, res);
  for (const [k, v] of CORS) clone.headers.set(k, v);
  return clone;
}

/** Try to match an API route; returns null when nothing matched (-> static). */
async function routeApi(req: Request, env: Env, url: URL): Promise<Response | null> {
  const { pathname } = url;
  const method = req.method;

  // Preflight for every API path.
  if (method === "OPTIONS" && isApiPath(pathname)) {
    return empty(204);
  }

  // ---- /shared/slots -----------------------------------------------------
  if (pathname === "/shared/slots" && method === "GET") {
    return getSlots();
  }

  // ---- /meeting ----------------------------------------------------------
  if (pathname === "/meeting/book/add" && method === "POST") {
    return await meetingBookAdd(req, env);
  }
  if (pathname === "/meeting/book/list" && method === "GET") {
    return await meetingBookList(env);
  }
  if (method === "DELETE" && pathname.startsWith("/meeting/book/remove/")) {
    return await meetingBookRemove(pathname.slice("/meeting/book/remove/".length), env);
  }

  // ---- /library ----------------------------------------------------------
  if (pathname === "/library/book/add" && method === "POST") {
    return await libraryBookAdd(req, env);
  }
  if (pathname === "/library/book/list" && method === "GET") {
    return await libraryBookList(env);
  }
  if (method === "DELETE" && pathname.startsWith("/library/book/remove/")) {
    return await libraryBookRemove(pathname.slice("/library/book/remove/".length), env);
  }

  // ---- /masterclass ------------------------------------------------------
  if (pathname === "/masterclass/create" && method === "POST") {
    return await masterclassCreate(req, env);
  }
  if (pathname === "/masterclass/list" && method === "GET") {
    return await masterclassList(env);
  }
  if (pathname === "/masterclass/enroll" && method === "POST") {
    return await masterclassEnroll(req, env);
  }
  if (pathname === "/masterclass/enquire" && method === "POST") {
    return await masterclassEnquire(req, env);
  }

  return null;
}

function isApiPath(pathname: string): boolean {
  return (
    pathname.startsWith("/meeting/") ||
    pathname.startsWith("/library/") ||
    pathname.startsWith("/shared/") ||
    pathname.startsWith("/masterclass/")
  );
}

export default {
  async fetch(req: Request, env: Env): Promise<Response> {
    const url = new URL(req.url);

    // Only treat our four API prefixes as API; everything else is the site.
    if (isApiPath(url.pathname)) {
      try {
        const res = await routeApi(req, env, url);
        if (res) return withCors(res);
        // API path but wrong method/shape -> 404 (don't serve HTML here).
        return withCors(empty(404));
      } catch (err) {
        return withCors(handleError(err));
      }
    }

    // Not an API path -> serve static asset from public/ (index.html, images...).
    return env.ASSETS.fetch(req);
  },
};
