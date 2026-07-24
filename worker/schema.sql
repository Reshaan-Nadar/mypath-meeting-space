-- mypath — Supabase schema
-- Run once in Supabase: SQL Editor → New query → paste → Run.
-- Mirrors mypath-backend/src/db.rs, using Postgres types.
-- All tables intentionally created in the `public` schema (Supabase default).

-- 1) bookings — shared by Meeting and Library (discriminated by booking_type)
CREATE TABLE IF NOT EXISTS public.bookings (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    booking_date    DATE         NOT NULL,
    time_slot       VARCHAR(50)  NOT NULL,
    booking_type    VARCHAR(50)  NOT NULL,            -- 'Meeting' | 'Library'
    contact         VARCHAR(50)  NOT NULL,
    first_name      VARCHAR(255),
    last_name       VARCHAR(255),
    attendees       VARCHAR(50),
    organizer_name  VARCHAR(255),
    topic           VARCHAR(255),
    room_name       VARCHAR(100)
);

-- One room, one slot, one date = at most one booking. Double-booking returns 409.
CREATE UNIQUE INDEX IF NOT EXISTS bookings_unique_booking_datetime
    ON public.bookings (booking_date, time_slot, room_name);

-- 2) master_classes
CREATE TABLE IF NOT EXISTS public.master_classes (
    id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title          VARCHAR(255) NOT NULL,
    description    TEXT         NOT NULL,
    timing         VARCHAR(50)  NOT NULL,
    date           DATE         NOT NULL,
    presenter_name VARCHAR(255) NOT NULL
);

-- 3) master_class_enrollments
CREATE TABLE IF NOT EXISTS public.master_class_enrollments (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    master_class_id BIGINT       NOT NULL REFERENCES public.master_classes(id) ON DELETE CASCADE,
    first_name      VARCHAR(255) NOT NULL,
    last_name       VARCHAR(255) NOT NULL,
    contact         VARCHAR(50)  NOT NULL
);

-- 4) master_class_enquiries
CREATE TABLE IF NOT EXISTS public.master_class_enquiries (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    master_class_id BIGINT       NOT NULL REFERENCES public.master_classes(id) ON DELETE CASCADE,
    contact         VARCHAR(50)  NOT NULL,
    message         TEXT         NOT NULL
);

-- Expose all four tables to PostgREST so the Worker (service-role key) can use them.
ALTER TABLE public.bookings                     ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.master_classes               ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.master_class_enrollments     ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.master_class_enquiries       ENABLE ROW LEVEL SECURITY;

-- The Worker uses the service_role key, which bypasses RLS. No policies needed
-- for the API. Add explicit policies later if you want anon/browser reads.
