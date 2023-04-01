This is an example project on how to use FreeRTOS-rust on (STM32F3Discovery)[https://www.st.com/en/evaluation-tools/stm32f3discovery.html]
The project comes with 2 tasks, each blinking an LED on board.


## How to flash and run:
I'm using openocd and gdb to load and run the software.

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
$ gdb-multiarch target/thumbv7em-none-eabihf/debug/FreeRTOS-rust -ex "target remote localhost:3333"
```
where target/thumbv7em-none-eabihf/debug/FreeRTOS-rust is the compiled image

* Finally from inside gdb terminal you can flash the new image via
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
