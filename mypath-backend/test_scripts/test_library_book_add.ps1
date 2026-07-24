$body = @{
    date = "2026-07-25"
    first_name = "John"
    last_name = "Doe"
    contact = "john.doe@example.com"
    time_slot = "09:00-10:00"
    attendees = "Two"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://127.0.0.1:8090/library/book/add" -Method Post -Body $body -ContentType "application/json"
