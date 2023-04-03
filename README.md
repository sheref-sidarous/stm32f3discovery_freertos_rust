This is an example project on how to use [FreeRTOS-rust](https://github.com/lobaro/FreeRTOS-rust) on [STM32F3Discovery](https://www.st.com/en/evaluation-tools/stm32f3discovery.html)
The project comes with 2 tasks, each blinking an LED on board.

## How to build
* Check out FreeRTOS and FreeRTOS-rust which are referenced as git submodules
```
$ git submodule init && git submodule update
```
* Use Nighly rust with `thumbv7em-none-eabihf` target enabled
```
$ rustup toolchain add nightly-x86_64-unknown-linux-gnu
$ rustup target add thumbv7em-none-eabihf
```
* Build via cargo
```
$ cargo build
```

## How to flash and run:
I'm using openocd and gdb to load and run the compiled image.

* Have OpneOCD and gdb-multiatch installed, this was done on my ubuntu via 
```
$ sudo apt install openocd gdb-multiarch 
```
* On one terminal run OpenOCD
```
$ openocd -f openocd.cfg

```
* Then, on another terminal launch gdb-multiarch
```
$ gdb-multiarch target/thumbv7em-none-eabihf/debug/stm32f3discovery_freertos_rust -ex "target remote localhost:3333"
```
where target/thumbv7em-none-eabihf/debug/stm32f3discovery_freertos_rust is the compiled image

* Finally, from inside gdb terminal you can flash and run the new image via
```
(gdb) load
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x4e58 lma 0x8000194
Loading section .rodata, size 0x684 lma 0x8004ff0
Loading section .data, size 0x8 lma 0x8005674
Start address 0x08000194, load size 22136
Transfer rate: 21 KB/sec, 4427 bytes/write.
(gdb) continue
Continuing.
```

Those instructions are mostly copied from (The Rust Embedded book)[https://docs.rust-embedded.org/book/start/hardware.html#debugging], check it out for more details, explanation and troubleshooting.
