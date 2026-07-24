// Mirrors mypath-backend/src/shared/models.rs — TimeSlot enum.
// The booking calendar and backend both speak these exact slot strings.

export const TIME_SLOTS = [
  "09:00-10:00",
  "10:00-11:00",
  "11:00-12:00",
  "12:00-13:00",
  "13:00-14:00",
  "14:00-15:00",
  "15:00-16:00",
  "16:00-17:00",
  "17:00-18:00",
  "18:00-19:00",
] as const;

export type TimeSlot = (typeof TIME_SLOTS)[number];

export const MEETING_ATTENDEES = [
  "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
] as const;
export type MeetingAttendees = (typeof MEETING_ATTENDEES)[number];

export const LIBRARY_ATTENDEES = ["One", "Two", "Three", "Four"] as const;
export type LibraryAttendees = (typeof LIBRARY_ATTENDEES)[number];

/** True when `value` is one of the canonical slot strings. */
export function isTimeSlot(value: string): value is TimeSlot {
  return (TIME_SLOTS as readonly string[]).includes(value);
}
