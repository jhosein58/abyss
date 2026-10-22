# Philosophy

Abyss is built on a simple premise: **you should understand what your machine is doing.**

Over the last decade, mainstream languages have added layer upon layer of abstraction. Writing modern software sometimes feels like wearing three pairs of oven mitts just to pick up a coin: safe, perhaps, but clumsy and disconnected from reality.

Abyss takes off the oven mitts. It pairs clean, modern syntax with the uncompromising transparency of C.

---

## 1. Mechanical Sympathy

Hardware is not an abstract mathematical canvas. It has L1/L2/L3 cache lines, branch predictors, registers, and memory buses. 

Abyss encourages data layout and code patterns that work *with* the hardware rather than fighting it. Linear memory arrays beat pointer-chasing linked lists every day of the week, and Abyss makes writing data-oriented code feel natural.

## 2. No Garbage Collector

A garbage collector is convenient—until it pauses your application to tidy up right in the middle of an audio buffer callback or a competitive game frame.

Abyss has no garbage collector. You allocate memory when you need it, and you free it when you're done. Your latency remains completely deterministic from the first microsecond to the last.

## 3. No Hidden Costs

What you see in your source code is what actually executes on the processor:

- **No hidden copy constructors** copying megabytes of memory behind your back.
- **No implicit allocations** secretly calling `malloc` when you concatenate two values.
- **No mysterious runtime daemons** managing threads or checking stacks.

If an operation takes memory or time, it is visible in the code.

## 4. Modern Ergonomics, Low-Level Soul

Low-level does not have to mean archaic. Abyss draws ergonomic inspiration from modern languages like Odin and Jai:

- Clean type inference (`:=`).
- First-class types that can be passed to and returned from functions.
- Expressive compile-time evaluation (`cmpt`) powered by an integrated bytecode VM.
- Functions without pointless boilerplate: if a function doesn't return anything, you don't need to specify a return type.

## 5. Portability Through C99

Rather than dragging a multi-gigabyte compiler backend infrastructure into every installation, the Abyss compiler emits standard, portable ISO C99. 

Any platform with a working C compiler—whether it's a Linux server, a Windows desktop, an embedded ARM board, or a retro machine—can compile and run Abyss code at maximum optimization.

---

[Next: Architecture &rarr;](architecture.md)
