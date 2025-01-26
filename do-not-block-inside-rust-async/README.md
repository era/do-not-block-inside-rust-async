# Blocking inside tokio runtime blocks the whole thread

```
cargo run --bin problem 

task 0 starting
task 0 finished
task 2 starting
task 1 starting
```



```
cargo run --bin solution
task 0 starting
task 0 finished
task 1 starting
task 3 starting
task 2 starting
task 4 starting
we woke up
receiving 0
task 1 finished
receiving 1
task 3 finished
receiving 3
task 2 finished
receiving 2
task 4 finished
receiving 4
result of the tasks: 0
result of the tasks: 1
result of the tasks: 2
result of the tasks: 3
result of the tasks: 4
Done :)
```
```

```
```
```
