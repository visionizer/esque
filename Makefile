ARCH ?= x86_64
MEMLIM ?= 256M

OUTDIR = build
BINDIR = binaries
FINAL = Esque.img
FPATH = $(OUTDIR)/$(FINAL)
MODE ?= release

QEMU = qemu-system-$(ARCH)
QEMUFLAGS = \
	-drive file=$(FPATH),format=raw \
	-m $(MEMLIM) \
	-enable-kvm \
	-machine q35,accel=kvm:tcg \
	-cpu host \
	-drive if=pflash,format=raw,unit=0,file=$(BINDIR)/OVMF/OVMF_CODE.fd,readonly=on \
	-drive if=pflash,format=raw,unit=1,file=$(BINDIR)/OVMF/OVMF_VARS.fd \
	-net none -d int,cpu_reset,guest_errors,page,strace -D log.log \
	-no-shutdown -no-reboot

rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))
INITRDFILES = $(call rwildcard,initramfs,*.*)
INITRAMFS = $(OUTDIR)/initramfs.tar

all: $(INITRAMFS) kernel boot strip mkimg run

check: kernel-check

build: format kernel boot strip mkimg

format:
	cargo fmt

clean:
#	rust-analyzer may place weird files into target/debug/deps that cannot be removed
	rm -rf build || rm -rf target/{kernel,boot} || true
	rm -rf target || true
	mkdir build || true


.PHONY: kernel
kernel:
	@$(MAKE) -C kernel build ARCH=$(ARCH) MODE=$(MODE)
	@cp target/kernel/$(MODE)/kernel $(OUTDIR)/esque

.PHONY: boot
boot:
	@$(MAKE) -C boot build ARCH=$(ARCH) MODE=$(MODE)
	@cp target/boot/$(MODE)/boot.efi $(OUTDIR)/BOOTX64.EFI 

.PHONY: kernel-check
kernel-check:
	@$(MAKE) -C kernel check ARCH=$(ARCH) MODE=$(MODE)


.PHONY: strip
strip:
	@strip $(OUTDIR)/esque
	@strip $(OUTDIR)/BOOTX64.EFI


mkimg:
	@dd if=/dev/zero of=$(FPATH) bs=512 count=93750
	@mkfs.vfat -F 32 $(FPATH)
	@mmd -i $(FPATH) ::/EFI
	@mmd -i $(FPATH) ::/EFI/BOOT
	@mcopy -i $(FPATH) $(OUTDIR)/BOOTX64.EFI ::/EFI/BOOT
	@mcopy -i $(FPATH) $(OUTDIR)/esque ::
	@mcopy -i $(FPATH) $(BINDIR)/font/font.psf ::
	@mcopy -i $(FPATH) $(BINDIR)/efi-shell/startup.nsh ::

$(INITRAMFS): $(INITRDFILES)
	tar -cvf $(INITRAMFS) initramfs

run:
	$(QEMU) $(QEMUFLAGS)

cloc: clean
	@echo $(shell (( find ./ -name "*.rs" -print0 | xargs -0 cat ) | wc -l ))

unsafe-counter:
	@printf "A total of %d occurences have been found (%d LOC, %d Percent)\n" $(shell grep -Rnw --include=\*.rs -e "unsafe" | wc -l) $(shell (( find ./ -name "*.rs" -print0 | xargs -0 cat ) | wc -l )) $(shell expr  $(shell grep -Rnw --include=\*.rs -e "unsafe" | wc -l) / $(shell find ./ -name "*.rs" -print0 | xargs -0 cat  | wc -l))