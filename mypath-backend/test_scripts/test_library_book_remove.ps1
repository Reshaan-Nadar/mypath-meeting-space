$id = 1
Invoke-RestMethod -Uri "http://127.0.0.1:8090/library/book/remove/$id" -Method Delete
