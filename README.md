# Async Executor Instrumentation Observability Utility (aeiou)

This crate provides nice debugging output for Tokio's tracing instrumentation.

## Examples

### Basic examples

The basic examples don't use tokio and can be run as normal.

```sh
cargo run --example events
```

Giving the output:

```
2023-11-23T11:54:54.643911Z  INFO a message
2023-11-23T11:54:54.644014Z DEBUG field="value" my message
2023-11-23T11:54:54.644040Z ERROR field="only one"
```

```sh
cargo run --example spans
```

Output:

```
2023-11-23T11:55:14.978625Z  INFO a message
2023-11-23T11:55:14.978796Z DEBUG span.info[1]{mog=4, gom="cow"} field="value" my message
2023-11-23T11:55:14.978827Z ERROR span.info[1]{mog=4, gom="cow"} span.trace[2]{} field="only one"
```

### Tokio Examples

To run the Tokio based examples, the `tokio_unstable` flag must be used.

```sh
RUSTFLAGS="--cfg tokio_unstable" cargo run --example tokio-task
```

The output will then be from the `tracing` instrumentation in Tokio:

```
2023-11-23T11:56:39.211023Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} duration=101, duration.unit="ms", duration.op="override"
2023-11-23T11:56:39.211123Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op_name="poll_elapsed", is_ready=true
2023-11-23T11:56:39.211163Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op="waker.clone", task.id=1
2023-11-23T11:56:39.211199Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op_name="poll_elapsed", is_ready=false
2023-11-23T11:56:39.313345Z TRACE op="waker.wake", task.id=1
2023-11-23T11:56:39.313457Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op_name="poll_elapsed", is_ready=true
2023-11-23T11:56:39.313521Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op="waker.clone", task.id=1
2023-11-23T11:56:39.313568Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} runtime.resource[274877906945]{concrete_type="Sleep", kind="timer", loc.file="examples/tokio-task.rs", loc.line=14, loc.col=9} runtime.resource.async_op[274877906946]{source="Sleep::new_timeout"} runtime.resource.async_op.poll[274877906947]{} op_name="poll_elapsed", is_ready=true
2023-11-23T11:56:39.313763Z TRACE runtime.spawn[1]{kind=task, task.name=, task.id=18, loc.file="examples/tokio-task.rs", loc.line=13, loc.col=5} op="waker.drop", task.id=1
2023-11-23T11:56:39.314000Z TRACE op_name="poll_recv", is_ready=false
2023-11-23T11:56:39.314128Z TRACE op_name="poll_recv", is_ready=true
2023-11-23T11:56:39.314102Z TRACE tx_dropped=true, tx_dropped.op="override"
2023-11-23T11:56:39.314430Z TRACE rx_dropped=true, rx_dropped.op="override"
```