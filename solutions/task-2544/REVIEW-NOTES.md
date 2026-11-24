# Review notes

`observe_abi_status_mask` returns an owned Vec<u32>. It reads the borrowed input and records abi status mask in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The target C ABI determines the callback's calling convention. The extern declaration and the function-pointer type both carry that ABI.
