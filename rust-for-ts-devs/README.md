
##  Some Rust for TS Devs


```rust
struct Foo {
  propertis...
  pub propertis...
}

impl Foo {
  // these are both static methods
  fn this()
  pub fn this() // public everyone can use  

  fn this(&self)
  fn this(&mut self)

  pub fn this(self)

}

```

```txt
Something people think is complicated
String
* well string is a heap allocated
* can be mutable

&str
* this points to a sequence of utf-8 char
* its immutable
* its analogous to &[u8]
```
