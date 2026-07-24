$body = @{
    title = "Rust for Beginners"
    description = "Learn the basics of Rust programming language."
    timing = "14:00-16:00"
    date = "2026-08-01"
    presenter_name = "Alice Johnson"
    admin_hash = "secret_hash"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://127.0.0.1:8090/masterclass/create" -Method Post -Body $body -ContentType "application/json"
