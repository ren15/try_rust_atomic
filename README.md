# Why this repo

This repo demostrates that `std::sync::atomic::Ordering::Relaxed` will have ordering "issues" that surprise people who don't have experience in atmoic programming. The code is derieved from examples in https://marabos.nl/atomics/ 

## How to run


```bash
cargo r --release
```

You will get output like:
```bash
Running for 10s...
a_iter: 131730996
b_iter: 56274065
(10,  0) =   52946676  94.087%
(10, 20) =    2220297  3.946%
( 0,  0) =    1106986  1.967%
( 0, 20) =        106  0.000%
```

Some people will expect the result to be 100% (10,0)
