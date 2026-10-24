# Open questions
Add when a question opens. Change its state when it moves. Move to "Closed" with the answer; do not delete.

## Open
- 2026-10-24 — Cơ chế chặn việc mở lại game: chỉ kill lặp lại theo polling, hay cần chặn sâu hơn ở mức khởi động tiến trình (driver/hook)? — Duc42195
- 2026-10-24 — Chọn crate GUI nào cho overlay chặn màn hình (`winit` vs gọi thẳng Win32 API qua `windows-rs`)? — Duc42195
- 2026-10-24 — Có nên đọc registry `GameConfigStore` (Xbox Game Bar heuristic) để gợi ý thêm exe vào `GAME_LIST` không? Chỉ dùng làm gợi ý xem thủ công, không tự động kill theo heuristic này (rủi ro false positive). — Duc42195
- 2026-10-25 — Machine id hiện dùng hostname (xem decisions-log topic machine-identification) — nếu sau này cần id sống sót qua việc đổi tên máy, có đáng thêm crate đọc `MachineGuid` trong registry Windows không (đổi platform-specific, cần cfg-gate cho build Linux dev)? — Duc42195

## Closed
- YYYY-MM-DD — question — answer (link to decision/ADR)
