# Chapter 4 Ownership

Owner ship is a the most unique feature in Rust.  
It enable Rust to make memory safety without gc(garbage collection).  

Features related to **Ownership**:
* borrowing
* slices

## What is Ownership?

Old memory management strategy:
* GC
* mem allocattion and free by programmer

Rust way: A system of Ownership.  
Let's learn ownership with data structure: String.

pre knowledge: stack & heap.  

### Ownership rules
* Each value in Rust has an owner;
* There can only one owner at a time;
* When the owner goes out of scope, the value will be dropped.

### Variable Scope
let's coding some example to see.  
refer to project ownership.
