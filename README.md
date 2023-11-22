# imageutil
utility for image crate. 
current in alpha version.

# benchmark
```rs
use image::{DynamicImage, Rgba};
use imageutil::gradiation::{linear_gradient, Gradiation};

fn main() {
    let mut img = DynamicImage::new_rgba8(100, 100);
    for _ in 0..100 {
        linear_gradient(
            &mut img,
            &Gradiation {
                start_color: Rgba([0, 0, 255, 255]),
                end_color: Rgba([255, 255, 255, 255]),
                start_point: (0, 0),
                end_point: (100, 100),
            },
        );
    }
}
```

```sh
> hyperfine ./target/release/imageutil -w 5
Benchmark 1: ./target/release/imageutil
  Time (mean ± σ):       8.3 ms ±   1.6 ms    [User: 7.7 ms, System: 0.5 ms]
  Range (min … max):     5.9 ms …  16.3 ms    325 runs
```