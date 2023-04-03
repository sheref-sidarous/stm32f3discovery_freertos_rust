#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use freertos_rust::*;
use core::alloc::Layout;

use cortex_m;

use stm32f3xx_hal::{pac, prelude::*};

extern crate panic_halt; // panic handler

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;


#[entry]
fn main() -> ! {
    //let dp = Peripherals::take().unwrap();
    //let gpioc = dp.GPIOC.split();
    //let mut device = MyDevice::from_pins(gpioc.pc13.into_push_pull_output());
    //device.set_led(false);

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led1 = gpioe
          .pe13
          .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut led2 = gpioe
          .pe14
          .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);


    Task::new().name("Task1").stack_size(128).priority(TaskPriority(2)).start(move |_| {
        loop{
            led1.toggle().unwrap();
            freertos_rust::CurrentTask::delay(Duration::ms(300));
        }
    }).unwrap();

    Task::new().name("Task2").stack_size(128).priority(TaskPriority(2)).start(move |_| {
        loop{
            led2.toggle().unwrap();
            freertos_rust::CurrentTask::delay(Duration::ms(400));
        }
    }).unwrap();


    FreeRtosUtils::start_scheduler();
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    asm::bkpt();
    loop {}
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}

#[no_mangle]
#[allow(non_snake_case)]
fn vApplicationStackOverflowHook(_pxTask: FreeRtosTaskHandle, _pcTaskName: FreeRtosCharPtr) {
    asm::bkpt();
    loop {}
}

#[no_mangle]
#[allow(non_snake_case)]
fn vApplicationTickHook () {
    
}

#[no_mangle]
#[allow(non_snake_case)]
fn vApplicationIdleHook() {
    
}

#[no_mangle]
#[allow(non_snake_case)]
fn vApplicationMallocFailedHook() {
    asm::bkpt();
    loop {}
}
