# blitsort-sys
Blitsort for rust binding.

### Usage

```rust
use blitsort_sys::BlitSort;

fn main() {
    let mut array = [0, 10, 1, 9, 2, 8, 3, 7];
    array.sorted();
    println!("{:?}", array);
}
```

OR direct use `blitsort_sys::blitsort`