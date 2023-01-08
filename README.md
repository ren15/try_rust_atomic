# How to run


```bash
cargo r --release
```

You will get output like:
```bash
trying: x: 0, y: 0
b thread joined at count: 2
a thread joined
x: 0, y: 0 happened
--------------------------------
trying: x: 10, y: 0
b thread joined at count: 365887
a thread joined
x: 10, y: 0 happened
--------------------------------
trying: x: 10, y: 20
b thread joined at count: 1
a thread joined
x: 10, y: 20 happened
--------------------------------
trying: x: 0, y: 20
b thread joined at count: 1006524053
a thread joined
x: 0, y: 20 happened
--------------------------------
```