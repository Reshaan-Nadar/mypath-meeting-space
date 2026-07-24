$body = @{
    date = "2026-07-26"
    organizer_name = "Jane Smith"
    topic = "Project Sync"
    contact = "jane.smith@example.com"
    time_slot = "10:00-11:00"
    room_name = "Conference Room A"
    attendees = "Five"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://127.0.0.1:8090/meeting/book/add" -Method Post -Body $body -ContentType "application/json"
