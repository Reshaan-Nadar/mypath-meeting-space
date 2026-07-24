## Goal
Deploy the mypath meeting-room site (frontend + backend) to a Cloudflare Worker backed by Supabase, on a custom domain. Meeting bookings go live at launch; the backend implements all 10 existing routes so library/masterclass can be enabled later with no backend rework.

## What stays the same (faithful rewrite)
- All 10 routes, paths unchanged (`/meeting/book/add`, `/library/book/list`, `/shared/slots`, etc.) so the frontend's existing `fetch` call needs no path change.
- The 5-table schema and the `unique (booking_date, time_slot, room_name)` constraint that makes double-booking return **409 Conflict**.
- The 10 hourly `TimeSlot` values (`09:00-10:00` … `18:00-19:00`) and the `MeetingAttendees`/`LibraryAttendees` word enums.
- The `ADMIN_HASH` env check on `/masterclass/create`.
- CORS allow-all (matches current `CorsLayer`).

## Step 1 — Scaffold the Worker project
Create `worker/` with `package.json` (wrangler + `@supabase/supabase-js`), `tsconfig.json`, and `wrangler.toml`. Use **Workers Static Assets** (binding `ASSETS`) to serve the frontend, so the Worker only handles `/meeting`, `/library`, `/shared`, `/masterclass` and falls through to static files for everything else.

## Step 2 — Port the backend to TypeScript
- `src/shared/time-slots.ts`: the 10 slots + `allSlots()`, matching `shared/models.rs`.
- `src/supabase.ts`: a Postgres client. (Supabase exposes a pooled Postgres connection string — use `pg`-compatible fetch over Hyperdrive, or the Supabase REST/Postgrest client for simple inserts/selects. Decision: use **PostgREST** via `@supabase/supabase-js` for the simple CRUD here — least moving parts on Workers.)
- `src/routes/{meeting,library,masterclass,shared}.ts`: one handler per route, mirroring the Rust handlers' status codes (201 created / 409 conflict / 404 not found / 401 unauthorized / 500).
- `src/index.ts`: method+path router (no framework needed — ~40 lines), CORS layer, JSON parsing, and a 404 fallback. The unique-constraint logic: on insert, if a row with the same `(booking_date, time_slot, room_name)` exists, return 409 (equivalent to `is_unique_violation`).
- `worker/schema.sql`: `CREATE TABLE` for all 5 tables with a Postgres `UNIQUE` constraint, to run once in the Supabase SQL editor.

## Step 3 — Move & wire the frontend
- Create `worker/public/`. Copy `quorum-meeting-room_v2.html` → `public/index.html` and copy the three referenced images (`photo-4seater-compressed.jpg`, `photo-8seater-compressed.jpg`, `photo-beanbag.jpg`) into `public/`.
- Update the one line `var API_BASE = 'http://127.0.0.1:8090';` → `var API_BASE = '';` (same origin). Paths already match.
- Original files in the project root are left untouched.

## Step 4 — Secrets & config
- `wrangler.toml` sets `name`, `compatibility_date`, the `ASSETS` binding pointing at `public/`, and a `[vars]` block for non-secret config.
- Two secrets via `wrangler secret put`: `SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY` (the service role key is needed because the Worker writes rows; anon key is read-only). Plus `ADMIN_HASH` for the masterclass route.
- `.dev.vars.example` documents the same keys for local `wrangler dev`.

## Step 5 — Run Supabase schema
I'll give you the `schema.sql` to paste into Supabase → SQL Editor → Run. This creates the 5 tables with the unique constraint. (I cannot run it myself — you have the Supabase access.)

## Step 6 — Local test
Run `wrangler dev` and smoke-test: `GET /`, `GET /shared/slots`, `POST /meeting/book/add`, then POST again to confirm 409 conflict, then `GET /meeting/book/list`.

## Step 7 — Deploy to production
`wrangler deploy`. Then the custom domain: in the Cloudflare dashboard → Workers & Pages → your worker → Settings → Domains & Routes → add custom domain. I'll guide you through the exact steps once you share the domain; if the domain isn't on Cloudflare, DNS records need updating (we'll handle that together).

## What I need from you to proceed
1. **Supabase project URL** (`https://<ref>.supabase.co`) and the **service_role key** (Project Settings → API). I'll use these as Worker secrets — paste them when I run `wrangler secret put`.
2. Your **Cloudflare account** logged into wrangler (`npx wrangler login`) — I'll kick that off.
3. The **custom domain** you want to use, when we reach step 7.
4. A value for **ADMIN_HASH** (any secret string) if you want the masterclass-create route protected — optional, can skip.

## Out of scope (per your choice)
- Library + masterclass **UI** — backend is ready, but no new frontend for those flows now.
- Booking confirmation emails / WhatsApp (the Rust backend didn't have these either).

## Notes
- Nothing existing gets deleted or overwritten. The Rust backend stays in `mypath-backend/`; root HTML and images stay put. Everything new lives in `worker/` + `worker/public/`.
- The Worker code will be plain TypeScript with `@supabase/supabase-js` — readable, ~1 file per route group, matching the structure you already have in Rust.