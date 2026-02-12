#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{kprobe, kretprobe, map},
    maps::HashMap,
    programs::{ProbeContext, RetProbeContext},
    helpers::{bpf_get_current_pid_tgid, bpf_ktime_get_ns},
};

#[map]
static mut START_TIME: HashMap<u32, u64> = HashMap::with_max_entries(1024, 0);
#[map]
static mut IO_LATENCY: HashMap<u32, u64> = HashMap::with_max_entries(1024, 0);

#[kprobe]
pub fn aether_io_trace(_ctx: ProbeContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    let ts = unsafe { bpf_ktime_get_ns() };
    unsafe { let _ = START_TIME.insert(&pid, &ts, 0); }
    0
}

#[kretprobe]
pub fn aether_io_ret(_ctx: RetProbeContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    let ts_end = unsafe { bpf_ktime_get_ns() };
    unsafe {
        if let Some(ts_start) = START_TIME.get(&pid) {
            let latency = ts_end.saturating_sub(*ts_start);
            let _ = IO_LATENCY.insert(&pid, &latency, 0);
            let _ = START_TIME.remove(&pid);
        }
    }
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
