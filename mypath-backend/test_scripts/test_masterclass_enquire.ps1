$body = @{
    master_class_id = 1
    contact = "charlie@example.com"
    message = "Will this class be recorded?"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://127.0.0.1:8090/masterclass/enquire" -Method Post -Body $body -ContentType "application/json"
