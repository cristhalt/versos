#![no_std]
#![no_main]

mod panic;

#[uefi::entry]
fn uefi_main() -> uefi::Status {
    uefi::Status::SUCCESS
}
