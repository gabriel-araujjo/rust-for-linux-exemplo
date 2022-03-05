DIR ?= ../linux
CLANG ?= clang-13

all: ramdisk

exemplo.ko: exemplo.rs
	make -C $(DIR) M=$$PWD CC=$(CLANG)

ramdisk: exemplo.ko
	cp exemplo.ko ../busybox/_install
	cd ../busybox/_install/ && find . | cpio -o -H newc | gzip > ../initrd.img

clean:
	make -C $(DIR) M=$$PWD CC=$(CLANG) clean
