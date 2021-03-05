# rust-uefi-chainloader
A UEFI program written in rust that loads another EFI program and executes it.

Of particular interest to other developers may be the protocol definitions in the `src/protocols` folder, and the `device_path.rs` file in particular which has a number of convenience functions based on those found in the EDK2 libraries. Additionally a number of functions in `utils.rs` may be useful, including signature scanning function. The `hooks.rs` file contains an `X64Hook` object for inserting function hooks. Boot service hooks can be placed in `boot_services.rs`.

# Use
First place an EFI program in `qemu/hda-contents`.
Change the `PROGRAM_TO_RUN_PATH` in `src/main.rs` to match up with the efi executable you placed in `qemu/hda-contents/` and simply execute `build.sh`. To test execute `test.sh`.

Try on real hardware at your own risk :)

# Acknowledgements
[x1tan](https://github.com/x1tan/rust-uefi-runtime-driver)
[Aidan Khoury](https://github.com/ajkhoury/UEFI-Bootkit)
[r-efi team](https://github.com/r-efi/r-efi/)
