# Review notes

`observe_service_wait` returns an owned Vec<u32>. It reads the borrowed input and records service wait in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Label each enqueue attempt with its arrival index when auditing repeated payloads. Count a rejection at the full-queue decision, and reconcile accepted IDs against the ordered deliveries and remaining queue.
