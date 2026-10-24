# Chương trình giả lập "game": chỉ chạy, không làm gì, để tự test game-alert
# (phát hiện -> kill -> gửi Discord) trên Windows mà không cần Roblox thật.
#
# KHÔNG chạy trực tiếp "powershell -File game-alert-test.ps1" — Task Manager sẽ thấy
# tiến trình tên "powershell.exe", và nếu thêm tên đó vào GAME_LIST thì game-alert
# sẽ kill MỌI cửa sổ PowerShell đang mở trên máy, kể cả cái không liên quan.
#
# Cách đúng: chạy 1 lần để tạo bản sao PowerShell mang tên "game-alert-test.exe",
# rồi luôn khởi động bằng bản sao đó (xem scripts/README.md).

Write-Host "[game-alert-test] da khoi dong, PID $PID"
while ($true) {
    Start-Sleep -Seconds 5
    Write-Host "[game-alert-test] van dang chay..."
}
