# DECISIONS.md

Every non-trivial divergence from the original C source, and why we made it.

## 1. Started from a raw `c2rust` transpile rather than a hand-written rewrite
We used `c2rust transpile` to produce an initial, mechanically-faithful (heavily `unsafe`) Rust translation, then refined it incrementally. This preserves the original logic exactly as a starting point, so any behavioral divergence we introduce during cleanup is visible as a diff against a known-faithful baseline, rather than being buried inside a from-scratch rewrite.

## 2. Kept the `extern "C"` ABI identical to the original C API
All public functions keep their original C signatures (same names, same parameter types, same calling convention) so the existing test suite, fuzz driver (`fuzz.c`), and any other C code can link against the Rust library without modification. Internal implementation details changed; the external contract did not.

## 3. Replaced manual byte-swap + `memcpy` with `to_be_bytes()` in the write path
The raw transpile of `mpack_store_u16`/`u32`/`u64` used `val.swap_bytes()` followed by a native-endianness `memcpy`. We replaced this with Rust's standard `to_be_bytes()`. This is not just a style cleanup: the original pattern only produces correct big-endian byte order on a little-endian host, while `to_be_bytes()` is correct on any target architecture — a portability fix, not just a safety one.

## 4. Replaced C-style union type-punning with `f32::to_bits()` / `f64::to_bits()`
`mpack_store_float`/`mpack_store_double` originally reinterpreted a float's bits via a C union (translated faithfully by c2rust into a Rust union). We replaced this with the standard library's `to_bits()` methods, which do the same bit reinterpretation without a union, removing a category of undefined-behavior risk tied to union-based punning in Rust.

## 5. Narrowed `unsafe` scope via reference reborrowing instead of raw-pointer-everywhere
Functions like the writer/tree tracking helpers (`mpack_writer_track_push`, `mpack_tree_root`, etc.) originally dereferenced raw pointers directly throughout the function body. We changed these to take one `unsafe { &mut *ptr }` (or `&*ptr`) reborrow at the top of the function, with a `SAFETY` comment stating the FFI contract being relied on, then use safe reference access for the rest of the function body.

## 6. Added a differential fuzzing harness and test-only shim code, not part of the original library
`harness.py`, `fuzz_test_shim.c`, and the `fuzz-c`/`fuzz-rust`/`fuzz-expect-c`/`fuzz-expect-rust` build targets are new files we added for verification; none of them modify or replace any file in the original `mpack` source or its existing unit test suite, which we run unmodified.

## 7. Fixed a fuzz-harness build configuration mismatch (`MPACK_EXTENSIONS`)
The test harness binary linked against the Rust library was initially compiled with `MPACK_EXTENSIONS` unset while the Rust library itself was transpiled with `MPACK_EXTENSIONS=1`. This caused the harness to mishandle a code path the library actually supported, producing crashes that weren't a port bug but a build-configuration inconsistency. We aligned both build configurations in `build_fuzz_binaries.sh`.

## 8. Verified broadly before refining narrowly
Rather than deeply cleaning up one module before checking it worked end-to-end, we prioritized getting the full differential fuzzing pipeline (reader → writer → tree-parse, via the shared `fuzz.c` driver) running early and passing, then did unsafe-reduction/refinement on top of a module already confirmed behaviorally correct. This meant every cleanup pass had a fast regression check (re-run the fuzzer) rather than discovering behavioral drift only at the end.

## 9. Extended differential fuzzing to the `mpack_expect` module via a second, purpose-built harness
The original driver (`fuzz.c`) never exercises `mpack_expect_*` functions (they demand a specific type rather than reading whatever tag comes next, so they need a different driver shape). We added `fuzz_expect.c`, which uses the first input byte as a selector to pick which `mpack_expect_*` function to exercise against the rest of the input, giving that module the same differential-fuzzing coverage as the reader/writer/tree path instead of leaving it unverified.

## 10. Reused the existing unit-test allocator shim (`test-system.c`) for the fuzz harness instead of writing new stubs
The Rust crate was transpiled expecting `MPACK_MALLOC=test_malloc`/`MPACK_FREE=test_free` (the same configuration the unit test suite uses), so our fuzz harness links against the existing `test/unit/src/test-system.c` rather than defining new allocator functions. This keeps the allocator behavior identical between our fuzz binaries and the already-trusted unit test build, instead of introducing a second, unverified allocation path.

## 11. Built a standalone assert/break-handling shim instead of reusing `test/unit/src/test.c`
The Rust crate needs `mpack_assert_fail`/`mpack_break_hit` implementations (from the unit test config), but the existing `test.c` also defines `main()`, which would collide with `fuzz.c`'s own `main()`. We wrote `fuzz_test_shim.c` to provide equivalent assert/break behavior (print and abort, matching `test.c`'s own fallback) without a competing entry point, rather than modifying the original test file.

<!-- Add further entries below as more decisions are made (error-handling design, remaining unsafe-reduction choices, etc.) -->