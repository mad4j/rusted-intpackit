# intpackit
Rust Integer Pack Iterator

Library tha allow to extract integer values packaged in a larger integer.

```rust
let mut it = 0x89ABCDEFu32.unpack(16, 9);
assert_eq!(it.next(), Some(0xF));
assert_eq!(it.next(), Some(0xE));
assert_eq!(it.next(), Some(0xD));
assert_eq!(it.next(), Some(0xC));
assert_eq!(it.next(), Some(0xB));
assert_eq!(it.next(), Some(0xA));
assert_eq!(it.next(), Some(0x9));
assert_eq!(it.next(), Some(0x8));
assert_eq!(it.next(), Some(0x0));
assert_eq!(it.next(), None);
assert_eq!(it.next(), None); 
```

It works also on values that are not powers of two.

```rust
let v = (1*1 + 2*3 + 0*9 + 2*27 + 1*81) as u32;
let mut it = v.unpack(3, 6);
assert_eq!(it.next(), Some(1));
assert_eq!(it.next(), Some(2));
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(2));
assert_eq!(it.next(), Some(1));
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), None);
assert_eq!(it.next(), None);
```
