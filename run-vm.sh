qemu-system-x86_64 --kernel ../linux/arch/x86/boot/bzImage -append "console=ttyS0 rdinit=/sbin/init" -nographic  -initrd ../busybox/initrd.img
