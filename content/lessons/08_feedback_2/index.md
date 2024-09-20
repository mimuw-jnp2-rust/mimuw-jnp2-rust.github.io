+++
title = "Feedback #2"
date = 2022-11-21
weight = 1
[extra]
lesson_date = TODO
+++

## Feedback

### Conditional implementation

```rust
impl<const N: usize> Shape for SphereN<N> {
    type Volume = VolumeN<N>;
    fn volume(&self) -> Self::Volume {
        let mut volume: u32 = (f64::from(self.radius) * f64::from(self.radius) * PI) as u32;
        if N == 3 {
            volume = (f64::from(self.radius)
                * f64::from(self.radius)
                * f64::from(self.radius)
                * PI
                * 4.0_f64
                / 3.0_f64) as u32;
        }
        Self::Volume::new(volume)
    }
}
```

Instead of checking `N == 3`, you can provide different impls for `SphereN<2>` and
`SphereN<3>` (as they are different types).

### u32 and u64

They _are_ different types, but because you can easily cast one to another,
it was not sufficient to make the implementation type-safe.
