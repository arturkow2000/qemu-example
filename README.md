# `qemu-example`

An example bare-metal Cortex-A application (using
[cortex-a-rt](https://github.com/arturkow2000/cortex-a-rt)) running on Qemu virt
platform. Application parses Device Tree to find UART and interrupt controller,
then initializes UART, interrupt controller and CP15 timer, triggers SGI
interrupt and as a bonus initializes [embassy](https://github.com/embassy-rs/embassy)
executor.

## Running application

```shell
$ cargo build && qemu-system-arm -M virt-4.2 -cpu cortex-a7 -nographic -m 128M -kernel target/armv7a-none-eabi/debug/qemu-example | defmt-print -e target/armv7a-none-eabi/debug/qemu-example
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
INFO  Hello
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:22
DEBUG Memory:
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:24
DEBUG   0x40000000 - 0x47FFFFFF
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:29
INFO  GIC distributor @ 0x08000000
└─ qemu_example::irq::init @ src/irq.rs:48
INFO  GIC CPU interface @ 0x08010000
└─ qemu_example::irq::init @ src/irq.rs:52
INFO  Triggering interrupt
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:37
INFO  Back from IRQ handler
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:40
DEBUG Hello from IRQ handler, irq=15
└─ qemu_example::irq::irq_handler::{closure#0} @ src/irq.rs:25
INFO  CP15 Generic Timer frequency: 62500000 Hz
└─ qemu_example::__cortex_a_rt_main @ src/main.rs:43
INFO  Kernel starting ...
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:6
INFO  Ticking every 3 seconds
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:7
INFO  Timer tick
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:11
INFO  Timer tick
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:11
INFO  Timer tick
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:11
INFO  Timer tick
└─ qemu_example::kernel::__kmain_task::{async_fn#0} @ src/kernel/mod.rs:11
```

## Debugging

Application can be debugged using Visual Studio Code with
[CodeLLDB](https://github.com/vadimcn/vscode-lldb) plugin. This repo already
configuration for starting debugging session but due to limitations of VSCode
you have to run socat manually prior to starting debugging (for UART output).

```shell
cargo build && socat UNIX-LISTEN:/tmp/qemu-example-uart0 - | defmt-print -e target/armv7a-none-eabi/debug/qemu-example
```

Now you can start debugging from VSCode.
