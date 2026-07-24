$body = @{
    master_class_id = 1
    first_name = "Bob"
    last_name = "Williams"
    contact = "bob.williams@example.com"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://127.0.0.1:8090/masterclass/enroll" -Method Post -Body $body -ContentType "application/json"
