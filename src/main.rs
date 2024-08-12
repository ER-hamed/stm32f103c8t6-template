#![no_main]
#![no_std]

extern crate alloc;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::flash::FlashExt;
#[allow(unused_imports)]
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::{_fugit_RateExtU32, _stm32_hal_afio_AfioExt, _stm32_hal_rcc_RccExt, _stm32f4xx_hal_timer_SysCounterExt};

fn init_alloc() {
    #[global_allocator]
    static HEAP: Heap = Heap::empty();
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[entry]
fn main() -> ! {
    // Initialize system
    rtt_init_print!();
    rprintln!("Start");
    init_alloc();
    let Some(dp) = pac::Peripherals::take() else { panic!() };
    let Some(cp) = cortex_m::peripheral::Peripherals::take() else { panic!() };
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(8.MHz()) // Use the 8 MHz external oscillator
        .sysclk(72.MHz()) // Set the system clock to 72 MHz
        .pclk1(36.MHz())  // Set APB1 to 36 MHz
        .pclk2(72.MHz())  // Set APB2 to 72 MHz
        .freeze(&mut flash.acr);
    #[allow(unused_variables, unused_mut)]
    let mut delay = cp.SYST.delay(&clocks);
    #[allow(unused_variables, unused_mut)]
    let mut afio = dp.AFIO.constrain();
    rprintln!("Initialized");

    //let mut gpio_a = dp.GPIOA.split();
    //let mut gpio_b = dp.GPIOB.split();
    let mut gpio_c = dp.GPIOC.split();

    let mut led = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);

    loop {
        led.toggle();
        delay.delay_ms(1000u32);
    }
}


#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}
