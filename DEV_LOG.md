> Logs and thoughts while building **Moye** – a meme language project to learn fundamentals of language design and compilers.

---

## 🗓️ Day 1: Why Moye?

Decided to build a toy language called **Moye** just for fun and learning.  
The goal is to **understand language design** and how compilers work at a fundamental level.

---

## ✅ Step 1: Create a Parser

### What is a parser?

A **parser** takes a flat structure (text source code) and converts it into a **tree structure** (like an Abstract Syntax Tree or AST).

Useful Reference: [Parsing - Wikipedia](https://en.wikipedia.org/wiki/Parsing)

---

## ✏️ Designing the Syntax

Started defining the basic **syntax rules** for Moye:
- Identifiers (variable names)
- Literals (numbers, strings)
- Keywords
- Basic expressions

---

## ❓ Why Can't Identifiers Start With a Number?

> "The compiler should be able to identify a token as an identifier or a literal after looking at the first character."

If variable names could start with digits, there would be **ambiguity**:
- Is `123` a variable name or a number?
- `123abc` – is that a malformed number or a bad identifier?

To avoid this confusion, most languages (Rust, C, Python) **require identifiers to start with a letter or underscore**, not a digit.

---

## 🧠 Compiler Design 101: 7 Phases of Compilation

1. **Lexical Analysis** (Tokenizing input text)
2. **Syntax Analysis** (Parsing into AST)
3. **Semantic Analysis** (Checking for meaning and type correctness)
4. **Intermediate Code Generation** (Turning AST into low-level IR)
5. **Code Optimization** (Speed/efficiency improvements)
6. **Code Generation** (Producing assembly/machine code)
7. **Symbol Table Management** (Tracking variables, functions, etc.)

Ref: *Compilers: Principles, Techniques, and Tools* ("Dragon Book")

