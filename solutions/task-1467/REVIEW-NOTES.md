# Review notes

`observe_client_error_fixture` returns an owned Vec<u32>. It reads the borrowed input and records client error fixture in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.
