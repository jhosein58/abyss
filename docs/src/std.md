# Memory & Standard Library Core

The standard library keeps ownership in ordinary data structures. An arena owns chunks; a vector owns a heap buffer; a slice or `Str` borrows bytes. Nothing in the type system automatically frees storage or extends its lifetime.

## Choose a storage policy

| Storage | Owns allocation? | Release/reuse operation | Typical use |
| --- | --- | --- | --- |
| Ordinary local scalar/struct | C automatic storage | Leaving its C scope/function | Small values and headers |
| `Arena` | Chunk headers and byte buffers | `reset()` / `destroy()` | Related temporary allocations |
| `VecU8`, `VecU32`, etc. | One heap element buffer | `reset(mark)` / `destroy()` | Growing independent lists |
| `ArenaVecU8`, `ArenaVecI32` | Borrows arena; stores element-buffer pointer | Logical clear / arena release | Arena-lifetime builders |
| `Slice*`, `Str` | No | None | Borrowed contiguous views |
| `String` | Arena-backed buffer header | Owning arena release | Growable byte text |

```abyss
import Arena from "std/allocator.a"
import VecU32 from "std/vec/u32.a"

two_policies :: () {
    arena := Arena.new(4096)
    scratch := arena.alloc(64)
    *scratch = 0 as u8

    ids := VecU32.new()
    ids.push(7)
    ids.destroy()
    arena.destroy()
}
```

Import paths here assume a source file at repository root. Prefer an owner that reflects the actual lifetime; do not copy an owning header and expect an independent allocation.

## Arena allocation

`Arena` holds a pointer to the current `ArenaChunk` and a default chunk size. A chunk contains its next-chunk pointer, memory pointer, used offset, and capacity.

| Operation | Result | Behavior |
| --- | --- | --- |
| `Arena.new(default_chunk_size)` | `Arena` | Allocates an initial chunk header and buffer |
| `arena.alloc(size)` | `&u8` | Rounds size to a multiple of eight; advances current offset |
| `arena.realloc(old, old_size, new_size)` | `&u8` | Reuses or grows last allocation, otherwise allocates and copies |
| `arena.reset()` | `unit` | Sets every linked chunk offset to zero |
| `arena.destroy()` | `unit` | Frees each buffer and chunk header; clears current pointer |

```abyss
import Arena from "std/allocator.a"

arena_bytes :: () {
    arena := Arena.new(4096)
    bytes := arena.alloc(16)
    *bytes = 65 as u8
    grown := arena.realloc(bytes, 16, 32)
    *(grown + 1) = 66 as u8
    arena.destroy()
}
```

If the current chunk cannot fit an allocation, `alloc` prepends a new chunk whose capacity is the larger of the default chunk size and the aligned request size. Existing allocations remain in their older chunks until reset/release.

`align_up` computes `(size + 7) & ~7`. The implementation provides eight-byte allocation increments and has no requested-alignment parameter. Do not assume support for types requiring greater alignment. `ArenaChunk.size()` returns a hard-coded 32; it is a target assumption, not a computed size.

An allocation is raw storage, not zero-initialized typed storage. Cast only when the required size and alignment are available, and initialize before reading. There is no checked object-allocation primitive or automatically calculated type size.

### Reset and reallocation details

```abyss
reuse_scratch :: () {
    arena := Arena.new(1024)
    first := arena.alloc(64)
    *first = 1 as u8
    arena.reset()
    second := arena.alloc(64)
    *second = 2 as u8
    arena.destroy()
}
```

After `reset`, treat earlier allocation contents and views as invalid for continued use: later allocations can overwrite them. `reset` clears all offsets but retains the same current chunk. `alloc` does not search older reset chunks for space; it can prepend another chunk when the current one fills. Resetting offsets is therefore not a guarantee of optimal reuse across all retained chunks.

For `realloc`, shrinking or equal size returns the old pointer unchanged. Growth can extend the most recent allocation in the current chunk if enough space remains. Otherwise it allocates new storage and copies `old_size` bytes; the old area stays allocated in the arena. Pass accurate old sizes and keep only the new pointer as the authoritative buffer address.

There is no individual arena `free`, automatic destructor, or allocation-failure result type. The allocator dereferences its allocation results without a complete failure-handling path. After `destroy`, `alloc` is invalid until a new arena is constructed.

## Str: a borrowed byte view

`Str` contains `ptr: &u8` and `len: u64`. It owns no storage and does not require termination at `len`. It is byte-oriented, with no Unicode decoding or character-index API.

| API | Meaning |
| --- | --- |
| `Str.of(c_string)` / `Str_from(c_string)` | Measures a zero-terminated pointer with external `str_len` |
| `Str_new(pointer, length)` | Wraps an explicit pointer/length |
| `Str_empty()` | Null pointer and zero length |
| `Str_sub(view, start, length)` | Borrows a subrange without checks |
| `Str_eq(left, right)` | Length check followed by byte equality |
| `view.print()` / `Str_print(view)` | Prints exactly `len` bytes with `print_char` |
| `Str_println(view)` | Prints the view and one newline |
| `Str_size()` | Returns hard-coded 16 |

```abyss
import Str, Str_from, Str_sub, Str_eq, Str_println from "std/string.a"

inspect_text :: () {
    text := Str_from("Abyss")
    prefix := Str_sub(text, 0, 2)
    if Str_eq(prefix, Str_from("Ab")) {
        Str_println(prefix)
    }
}
```

`Str_sub` neither allocates nor validates the range. A subview can be valid for length-based operations while not being a zero-terminated C string at its end. Do not pass its pointer to a C-string function unless termination is independently guaranteed.

Comparing `Str` values with `==` is not supported struct comparison. Use `Str_eq`; it compares content, whereas raw pointer equality compares addresses.

## String: an arena-backed builder

`String` contains `buf: ArenaVecU8`. Its API uses free functions rather than inline methods:

| Function | Effect |
| --- | --- |
| `String_new(&arena)` | Creates an arena-backed buffer with initial capacity four |
| `String_push_str(&builder, view)` | Appends every byte of a `Str` |
| `String_push_char(&builder, byte)` | Appends one `u8` |
| `String_push_u32(&builder, value)` | Appends decimal digits |
| `String_as_str(&builder)` | Borrows current pointer/length |

```abyss
import Arena from "std/allocator.a"
import Str_from, Str_println, String_new, String_push_str,
       String_push_char, String_push_u32, String_as_str from "std/string.a"

build_message :: () {
    arena := Arena.new(1024)
    message := String_new(&arena)
    String_push_str(&message, Str_from("item "))
    String_push_u32(&message, 42)
    String_push_char(&message, *"!")
    view := String_as_str(&message)
    Str_println(view)
    arena.destroy()
}
```

Appending does **not** add a terminating zero automatically. Print with `Str_println` or use another explicit-length consumer. `print(view.ptr)` is not generally safe for a builder view.

The view does not copy data. A later append can move the buffer or overwrite bytes visible through old views; an arena reset can invalidate its contents, and destruction releases them. Take a fresh view after mutation. There is no `String.destroy()` operation: the arena owns the bytes.

## Slices: pointer and length

The concrete modules are `SliceU8`, `SliceU32`, `SliceI32`, `SliceBool`, and `SliceStr`. They each alias a module-local named struct with `ptr` and `len` fields. They neither allocate nor release storage.

```abyss
import SliceU32 from "std/slice/u32.a"

sum_slice :: (values SliceU32) u32 {
    total: u32 = 0
    index: u64 = 0
    while index < values.len {
        total = total + *(values.ptr + index)
        index = index + 1
    }
    total
}
```

Construct a slice with an explicit typed literal, or a vector's `as_slice()`:

```abyss
import VecU32 from "std/vec/u32.a"
import SliceU32 from "std/slice/u32.a"

borrow_values :: () {
    values := VecU32.new()
    values.push(11)
    slice: SliceU32 = .{ ptr: values.ptr, len: values.len }
    first := *(slice.ptr + 0)
    values.destroy()
}
```

There is no `slice[index]` parser implementation. Dereference pointer arithmetic explicitly. The type carries a length but the pointer operation does not enforce it.

The current `Slice*_new(pointer, length)` wrappers call `Self_new()` **without forwarding their two arguments**. The underlying `Self_new` requires both. Treat these wrappers as defective and use a literal or `as_slice()`; do not present them as working constructors.

## Heap vectors

`VecU8`, `VecU32`, `VecI32`, `VecBool`, and `VecStr` are concrete named types with `ptr`, `len`, and `cap`. They allocate through `malloc`/`realloc` and release with `free`. Element sizes come from helpers: 1 for `u8` and `bool`, 4 for `u32` and `i32`, and hard-coded 16 for `Str`.

```abyss
import VecU32 from "std/vec/u32.a"

vector_example :: () u32 {
    values := VecU32.with_cap(8)
    values.push(10)
    values.push(20)
    values.set(0, 11)
    first := values.get(0)
    values.destroy()
    first
}
```

### Vector API and preconditions

| Method | Behavior | Preconditions/notes |
| --- | --- | --- |
| `Type.new()` | Calls `with_cap(4)` | Static |
| `Type.with_cap(capacity)` | Allocates, clamps capacity to at least four | Static |
| `reserve(additional)` | Ensures space for `len + additional` | Does not change length |
| `grow()` | Reserves one when length equals capacity | Used by `push` |
| `push(value)` | Writes at length and increments it | Can reallocate |
| `get(index)` | Dereferences indexed element | `index < len`; unchecked |
| `set(index, value)` | Writes indexed element | `index < len`; unchecked |
| `pop()` | Decrements length and reads last element | Nonempty; unchecked |
| `last()` | Reads element at `len - 1` | Nonempty; unchecked |
| `is_empty()` | Tests length for zero | Does not inspect allocation |
| `extend(pointer, count)` | Reserves and byte-copies elements | Valid source; avoid aliasing destination buffer |
| `mark()` | Returns current length | A logical stack marker |
| `reset(mark)` | Shrinks length when mark is no greater than length | Retains capacity |
| `clear()` | Intended logical clear | Only `VecU32` uses correct `.len` form in this checkout |
| `ptr_at(offset)` | Returns raw element pointer | No bounds check |
| `swap_remove(index)` | Replaces removed slot with last element and shrinks | Nonempty, valid index; order changes |
| `as_slice()` | Returns borrowed pointer/length | Growth/destruction can invalidate view |
| `destroy()` | Frees buffer and zeros pointer, length, capacity | Header must own its buffer |

```abyss
scratch_ids :: () u64 {
    ids := VecU32.new()
    ids.push(1)
    saved := ids.mark()
    ids.push(2)
    ids.push(3)
    ids.reset(saved)
    remaining := ids.len
    ids.destroy()
    remaining
}
```

Growth doubles capacity until `len + additional` fits, with a minimum of four. Allocation failure and integer overflow are not handled by a result type. A successful `reserve`/`push` can change `ptr`; discard stale element pointers and slices after growth.

`swap_remove` is constant work but changes order. `reset` shrinks the logical list and does not destroy elements. `VecStr.destroy()` frees the array of `Str` headers, not the buffers each header points to.

In `VecU8`, `VecI32`, `VecBool`, and `VecStr`, `clear` uses `len = 0` rather than `.len = 0`. It does not access the implicit field correctly. Avoid those methods; use `reset(0)` to clear logically. `VecU32.clear()` uses `.len` correctly. This is a current implementation defect, not intentional language omission.

Avoid extending a vector from its own element storage: reserve may move the source, and the copy routine is not an overlap-safe vector operation. The current prelude's `mem_copy` uses `memcpy` and silently skips copies over 100 MiB, so a very large `extend` or arena relocation cannot be assumed to copy correctly.

## Arena vectors

`std/avec/u8.a` and `std/avec/i32.a` provide `ArenaVecU8` and `ArenaVecI32`. Their operations are free functions with explicit vector pointers. Each header stores a pointer to its arena, element storage, length, and capacity.

```abyss
import Arena from "std/allocator.a"
import ArenaVecU8_new, ArenaVecU8_push, ArenaVecU8_get,
       ArenaVecU8_clear from "std/avec/u8.a"

arena_vector_example :: () u8 {
    arena := Arena.new(1024)
    bytes := ArenaVecU8_new(&arena)
    ArenaVecU8_push(&bytes, 65)
    first := ArenaVecU8_get(&bytes, 0)
    ArenaVecU8_clear(&bytes)
    arena.destroy()
    first
}
```

`*_new` starts at four elements. `*_with_cap` uses the requested capacity without the heap-vector minimum; zero capacity leaves doubling stuck at zero, so use a positive value. Growth delegates to `arena.realloc`, with the alignment, relocation, and copy limitations already described.

`get`, `set`, and `pop` are unchecked. `clear` sets length to zero and retains the buffer. There is no individual vector destructor: destroy the arena after all dependent headers and views are finished. The stored `&Arena` must also remain valid while operations grow the buffer; do not return a vector pointing to a dead local arena header.

## Raw memory and files

`std/mem.a` declares external `malloc`, `realloc`, `free`, `mem_copy`, and `mem_set`. Allocation returns `&u8`; typed element pointers are made with explicit casts. These are native operations, without ownership tracking or automatic cleanup.

```abyss
import malloc, free, mem_set from "std/mem.a"

raw_buffer :: () {
    bytes := malloc(16)
    if bytes != (0 as &u8) {
        mem_set(bytes, 0, 16)
        free(bytes)
    }
}
```

`std/fs.a` exposes `read_file(&arena, path)` returning a `Str`, and `write_file(path, view)` / `append_file(path, view)` returning `bool`. File reads allocate `file_size + 1`, add a terminating zero after the bytes actually read, and return that byte count. The functions print an error and call `fall()` on their handled failures; they do not return a recoverable error union.

```abyss
import Arena from "std/allocator.a"
import read_file from "std/fs.a"
import Str_println from "std/string.a"

show_file :: () {
    arena := Arena.new(4096)
    contents := read_file(&arena, "input.txt")
    Str_println(contents)
    arena.destroy()
}
```

`std/sys.a` wraps external `abyss_fall`. The project prelude exits the process in that function. No exception, `defer`, destructor, or recovery construct is added by these helpers.

Other compiler support modules include `FastMapU32` with a sentinel-based lookup, `Interner` for content-to-ID mapping, byte/string hashing, and `std/fmt.a` for terminal escape views. They are concrete support APIs, not evidence of a generic collection or formatting framework.

## Ownership rules to keep in view

```abyss
safe_borrow :: (values &VecU32) u32 {
    if values.len == 0 { ret 0 }
    values.get(0)
}
```

In this fragment `VecU32` must be imported, and the caller retains the allocation. The function reads the vector without taking responsibility for freeing it.

- Keep one authoritative owner for each heap-vector buffer and arena chunk chain. Copies of their headers share storage.
- Release a heap vector once; keep borrowed slices and element pointers out of use after release or reallocation.
- Keep arena-backed data and the arena header alive together. Reset invalidates old allocation contents for reuse.
- Treat every index, pointer cast, and size helper as an explicit precondition. Length fields do not create automatic checks.
- Distinguish zero-terminated C strings from `Str` lengths and unterminated builder buffers.

These rules follow from the actual operations and C emission; the compiler does not enforce them as an ownership type system.

**Implementation notes:** `std/allocator.a`, `std/string.a`, every concrete module in `std/slice/`, `std/vec/`, `std/avec/`, `std/mem.a`, `std/fs.a`, `std/sys.a`, and the definitions of `mem_copy` and `abyss_fall` in `prelude.c`.
