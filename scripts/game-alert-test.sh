#!/usr/bin/env bash
# Chương trình giả lập "game": chỉ chạy, không làm gì, để tự test game-alert
# (phát hiện -> kill -> gửi Discord) mà không cần Roblox thật.
#
# QUAN TRỌNG: chạy trực tiếp (./game-alert-test.sh hoặc bash game-alert-test.sh) thì
# Linux đặt tên tiến trình (comm, cái sysinfo đọc) là "bash" — KHÔNG phải tên file —
# đã verify bằng thực nghiệm. Thêm "bash" vào GAME_LIST sẽ kill mọi tiến trình bash
# khác trên máy. Phải chạy qua một bản copy của bash mang tên "game-alert-test" —
# xem scripts/README.md mục Linux.

trap 'echo "[game-alert-test] bi dung (SIGTERM/SIGINT) — binh thuong neu bi game-alert kill"; exit 0' TERM INT

echo "[game-alert-test] da khoi dong, PID $$"
while true; do
    sleep 5
    echo "[game-alert-test] van dang chay..."
done
