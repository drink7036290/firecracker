# Run Firecracker on macOS with Apple Silicon (Proof of Concept)

This document describes how to run Firecracker built from this repository on macOS with Apple Silicon.
Notes there're LOTs of codes disabled by `#[cfg(target_os = "linux")]` for a minimal workable version.

## Current status

- Successfully creating a microvm using the `macos.json` configuration file.
- Run `openssl speed` within the microvm and confirm Apple Virtualization Framework (AVF) works by its native performance.

## Current Environment

Host and Build environment:

- macOS 15.1.1 (arm64)
- MacBook Air M2 with 8GB RAM

Guest (microvm):

- Ubuntu 24.04 LTS (aarch64)
- kernel-6.1.102

## Prerequisites

A Linux kernel ARM64 boot executable Image (uncompressed, no ELF header), e.g.,

```console
% file kernel-6.1.102
kernel-6.1.102: Linux kernel ARM64 boot executable Image, little-endian, 4K pages
```

A rootfs, e.g.,

```console
% file ubuntu-24.04.ext4
ubuntu-24.04.ext4: Linux rev 1.0 ext4 filesystem data, UUID=a7f61230-4f25-4617-97a1-f9b261111954 (extents) (64bit) (large files) (huge files)
```

### Prepare the Image (docker required)

Download the virt-fwk repository:

```console
git clone https://github.com/drink7036290/virt-fwk
cd virt-fwk
```

Build the Image (make -j1 internally to avoid out-of-memory error):

```console
sudo make build-kernel
cp assets/kernel-6.1.102 <FIRECRACKER_RUN_DIR>
```

### Prepare the rootfs

Similar as the `getting-started.md` with some tweaks for macOS.

```console
# Download a rootfs
ARCH="aarch64"
brew install wget
wget -O ubuntu-24.04.squashfs.upstream "https://s3.amazonaws.com/spec.ccfc.min/firecracker-ci/v1.11/${ARCH}/ubuntu-24.04.squashfs"

# Create an ssh key for the rootfs
sudo rm -fr squashfs-root/
brew install squashfs
unsquashfs ubuntu-24.04.squashfs.upstream

rm -f ubuntu-24.04.id_rsa
ssh-keygen -f id_rsa -N ""
cp -v id_rsa.pub squashfs-root/root/.ssh/authorized_keys
mv -v id_rsa ./ubuntu-24.04.id_rsa

# create ext4 filesystem image
sudo rm -fr ubuntu-24.04.ext4
sudo chown -R root:wheel squashfs-root
truncate -s 400M ubuntu-24.04.ext4
brew install e2fsprogs
sudo /opt/homebrew/opt/e2fsprogs/sbin/mke2fs -d squashfs-root -t ext4 ubuntu-24.04.ext4
cp ubuntu-24.04.ext4 <FIRECRACKER_RUN_DIR>
```

## Steps

1. Prepare the running directory with the JSON configuration file and the entitlements file which is required by macOS.

    ```console
    FIRECRACKER_RUN_DIR="../firecracker_run"

    mkdir -p $FIRECRACKER_RUN_DIR
    cp macos.json firecracker.entitlements $FIRECRACKER_RUN_DIR
    ```

2. Build the Firecracker binary and copy it to the running directory.

    ```console
    BUILD_TARGET="aarch64-apple-darwin"

    cargo build --target $BUILD_TARGET --bin firecracker
    cp build/cargo_target/$BUILD_TARGET/debug/firecracker $FIRECRACKER_RUN_DIR
    ```

3. Go to the running directory and sign the Firecracker binary with the entitlements file.

    ```console
    cd $FIRECRACKER_RUN_DIR
    codesign -f --entitlement ./firecracker.entitlements -s - ./firecracker
    ```

4. Run the Firecracker binary with the JSON configuration file as no-api (server-only) mode. The dtruss is just for debugging.

    ```console
    sudo dtruss -f ./firecracker --no-api --config-file macos.json
    ```

5. Within the microvm, run `openssl speed` to measure the performance.

## Running results

```console
marshalllee@MarshalldeMacBook-Air firecracker % sudo dtruss -f ./firecracker --no-api --config-file macos.json
Password:
dtrace: system integrity protection is on, some features will not be available

        PID/THRD  SYSCALL(args)                  = return
2025-01-28T21:29:09.503396000 [anonymous-instance:main] Running Firecracker v1.11.0-dev
can_start: true
Starting VM...
2025-01-28T21:29:09.664413000 [anonymous-instance:main] Successfully started microvm that was configured from one single json
Waiting for VM state changes...
[    0.038780] cacheinfo: Unable to detect cache hierarchy for CPU 0
[    0.039395] loop: module loaded
[    0.039492] virtio_blk virtio1: 1/0/0 default/read/poll queues
[    0.039656] virtio_blk virtio1: [vda] 819200 512-byte logical blocks (419 MB/400 MiB)
[    0.040528] megasas: 07.719.03.00-rc1
[    0.040907] tun: Universal TUN/TAP device driver, 1.6
[    0.040978] thunder_xcv, ver 1.0
[    0.041000] thunder_bgx, ver 1.0
[    0.041019] nicpf, ver 1.0
[    0.041099] hns3: Hisilicon Ethernet Network Driver for Hip08 Family - version
[    0.041128] hns3: Copyright (c) 2017 Huawei Corporation.
[    0.041155] hclge is initializing
[    0.041175] e1000: Intel(R) PRO/1000 Network Driver
[    0.041201] e1000: Copyright (c) 1999-2006 Intel Corporation.
[    0.041229] e1000e: Intel(R) PRO/1000 Network Driver
[    0.041248] e1000e: Copyright(c) 1999 - 2015 Intel Corporation.
[    0.041274] igb: Intel(R) Gigabit Ethernet Network Driver
[    0.041292] igb: Copyright (c) 2007-2014 Intel Corporation.
[    0.041313] igbvf: Intel(R) Gigabit Virtual Function Network Driver
[    0.041336] igbvf: Copyright (c) 2009 - 2012 Intel Corporation.
[    0.041375] sky2: driver version 1.30
[    0.041446] VFIO - User Level meta-driver version: 0.3
[    0.041670] usbcore: registered new interface driver usb-storage
[    0.041859] rtc-pl031 20050000.pl031: registered as rtc0
[    0.041887] rtc-pl031 20050000.pl031: setting system clock to 2025-01-28T13:29:09 UTC (1738070949)
[    0.041971] i2c_dev: i2c /dev entries driver
[    0.042308] sdhci: Secure Digital Host Controller Interface driver
[    0.042335] sdhci: Copyright(c) Pierre Ossman
[    0.042389] Synopsys Designware Multimedia Card Interface Driver
[    0.042458] sdhci-pltfm: SDHCI platform and OF driver helper
[    0.042570] ledtrig-cpu: registered to indicate activity on CPUs
[    0.042705] usbcore: registered new interface driver usbhid
[    0.042725] usbhid: USB HID core driver
[    0.043107] NET: Registered PF_PACKET protocol family
[    0.043151] 9pnet: Installing 9P2000 support
[    0.043236] Key type dns_resolver registered
[    0.043284] registered taskstats version 1
[    0.043306] Loading compiled-in X.509 certificates
[    0.043774] input: gpio-keys as /devices/platform/gpio-keys/input/input0
[    0.043842] clk: Disabling unused clocks
[    0.043861] ALSA device list:
[    0.043916]   No soundcards found.
[    0.066135] EXT4-fs (vda): recovery complete
[    0.068636] EXT4-fs (vda): mounted filesystem with ordered data mode. Quota mode: none.
[    0.068689] VFS: Mounted root (ext4 filesystem) on device 254:0.
[    0.068813] devtmpfs: mounted
[    0.069245] Freeing unused kernel memory: 7552K
[    0.069288] Run /sbin/init as init process
[    0.108747] systemd[1]: systemd 255.4-1ubuntu8.4 running in system mode (+PAM +AUDIT +SELINUX +APPARMOR +IMA +SMACK +SECCOMP +GCRYPT -GNUTLS +OPENSSL +ACL +BLKID +CURL +ELFUTILS +FIDO2 +IDN2 -IDN +IPTC +KMOD +LIBCRYPTSETUP +LIBFDISK +PCRE2 -PWQUALITY +P11KIT +QRENCODE +TPM2 +BZIP2 +LZ4 +XZ +ZLIB +ZSTD -BPF_FRAMEWORK -XKBCOMMON +UTMP +SYSVINIT default-hierarchy=unified)
[    0.108854] systemd[1]: Detected virtualization vm-other.
[    0.108883] systemd[1]: Detected architecture arm64.

Welcome to Ubuntu 24.04.1 LTS!

[    0.109626] systemd[1]: Hostname set to <ubuntu-fc-uvm>.
[    0.176290] systemd[1]: Queued start job for default target graphical.target.
[    0.185945] systemd[1]: Created slice system-getty.slice - Slice /system/getty.
[  OK  ] Created slice system-getty.slice - Slice /system/getty.
[    0.186150] systemd[1]: Created slice system-modprobe.slice - Slice /system/modprobe.
[  OK  ] Created slice system-modprobe.slice - Slice /system/modprobe.
[    0.186295] systemd[1]: Created slice system-serial\x2dgetty.slice - Slice /system/serial-getty.
[  OK  ] Created slice system-serial\x2dget…slice - Slice /system/serial-getty.
[    0.186422] systemd[1]: Created slice user.slice - User and Session Slice.
[  OK  ] Created slice user.slice - User and Session Slice.
[    0.186491] systemd[1]: Started systemd-ask-password-console.path - Dispatch Password Requests to Console Directory Watch.
[  OK  ] Started systemd-ask-password-conso…equests to Console Directory Watch.
[    0.186564] systemd[1]: Started systemd-ask-password-wall.path - Forward Password Requests to Wall Directory Watch.
[  OK  ] Started systemd-ask-password-wall.…d Requests to Wall Directory Watch.
[    0.186638] systemd[1]: proc-sys-fs-binfmt_misc.automount - Arbitrary Executable File Formats File System Automount Point was skipped because of an unmet condition check (ConditionPathExists=/proc/sys/fs/binfmt_misc).
[    0.186698] systemd[1]: Expecting device dev-hvc0.device - /dev/hvc0...
         Expecting device dev-hvc0.device - /dev/hvc0...
[    0.186748] systemd[1]: Reached target cryptsetup.target - Local Encrypted Volumes.
[  OK  ] Reached target cryptsetup.target - Local Encrypted Volumes.
[    0.186801] systemd[1]: Reached target integritysetup.target - Local Integrity Protected Volumes.
[  OK  ] Reached target integritysetup.targ… Local Integrity Protected Volumes.
[    0.186855] systemd[1]: Reached target paths.target - Path Units.
[  OK  ] Reached target paths.target - Path Units.
[    0.186906] systemd[1]: Reached target remote-fs.target - Remote File Systems.
[  OK  ] Reached target remote-fs.target - Remote File Systems.
[    0.186956] systemd[1]: Reached target slices.target - Slice Units.
[  OK  ] Reached target slices.target - Slice Units.
[    0.187009] systemd[1]: Reached target swap.target - Swaps.
[  OK  ] Reached target swap.target - Swaps.
[    0.187057] systemd[1]: Reached target veritysetup.target - Local Verity Protected Volumes.
[  OK  ] Reached target veritysetup.target - Local Verity Protected Volumes.
[    0.187133] systemd[1]: Listening on systemd-initctl.socket - initctl Compatibility Named Pipe.
[  OK  ] Listening on systemd-initctl.socke…- initctl Compatibility Named Pipe.
[    0.187237] systemd[1]: Listening on systemd-journald-dev-log.socket - Journal Socket (/dev/log).
[  OK  ] Listening on systemd-journald-dev-…socket - Journal Socket (/dev/log).
[    0.187340] systemd[1]: Listening on systemd-journald.socket - Journal Socket.
[  OK  ] Listening on systemd-journald.socket - Journal Socket.
[    0.187399] systemd[1]: systemd-pcrextend.socket - TPM2 PCR Extension (Varlink) was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    0.187500] systemd[1]: Listening on systemd-udevd-control.socket - udev Control Socket.
[  OK  ] Listening on systemd-udevd-control.socket - udev Control Socket.
[    0.187577] systemd[1]: Listening on systemd-udevd-kernel.socket - udev Kernel Socket.
[  OK  ] Listening on systemd-udevd-kernel.socket - udev Kernel Socket.
[    0.188063] systemd[1]: Mounting dev-hugepages.mount - Huge Pages File System...
         Mounting dev-hugepages.mount - Huge Pages File System...
[    0.188296] systemd[1]: Mounting dev-mqueue.mount - POSIX Message Queue File System...
         Mounting dev-mqueue.mount - POSIX Message Queue File System...
[    0.192364] systemd[1]: Mounting sys-kernel-debug.mount - Kernel Debug File System...
         Mounting sys-kernel-debug.mount - Kernel Debug File System...
[    0.192480] systemd[1]: sys-kernel-tracing.mount - Kernel Trace File System was skipped because of an unmet condition check (ConditionPathExists=/sys/kernel/tracing).
[    0.195343] systemd[1]: Mounting tmp.mount - Temporary Directory /tmp...
         Mounting tmp.mount - Temporary Directory /tmp...
[    0.198285] systemd[1]: Mounting var-lib-systemd.mount - /var/lib/systemd...
         Mounting var-lib-systemd.mount - /var/lib/systemd...
[    0.199899] systemd[1]: Starting systemd-journald.service - Journal Service...
         Starting systemd-journald.service - Journal Service...
[    0.199988] systemd[1]: kmod-static-nodes.service - Create List of Static Device Nodes was skipped because of an unmet condition check (ConditionFileNotEmpty=/lib/modules/6.1.102/modules.devname).
[    0.206343] systemd[1]: Starting modprobe@configfs.service - Load Kernel Module configfs...
         Starting modprobe@configfs.service - Load Kernel Module configfs...
[    0.208164] systemd[1]: Starting modprobe@dm_mod.service - Load Kernel Module dm_mod...
         Starting modprobe@dm_mod.service - Load Kernel Module dm_mod...
[    0.211023] systemd-journald[113]: Collecting audit messages is disabled.
[    0.215215] systemd[1]: Starting modprobe@drm.service - Load Kernel Module drm...
         Starting modprobe@drm.service - Load Kernel Module drm...
[    0.217420] systemd[1]: Starting modprobe@efi_pstore.service - Load Kernel Module efi_pstore...
         Starting modprobe@efi_pstore.servi… - Load Kernel Module efi_pstore...
[    0.219541] systemd[1]: Starting modprobe@fuse.service - Load Kernel Module fuse...
         Starting modprobe@fuse.service - Load Kernel Module fuse...
[    0.221640] systemd[1]: Starting modprobe@loop.service - Load Kernel Module loop...
         Starting modprobe@loop.service - Load Kernel Module loop...
[    0.223648] systemd[1]: Starting systemd-modules-load.service - Load Kernel Modules...
         Starting systemd-modules-load.service - Load Kernel Modules...
[    0.223717] systemd[1]: systemd-pcrmachine.service - TPM2 PCR Machine ID Measurement was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    0.225391] systemd[1]: Starting systemd-remount-fs.service - Remount Root and Kernel File Systems...
         Starting systemd-remount-fs.servic…unt Root and Kernel File Systems...
[    0.227759] systemd[1]: Starting systemd-tmpfiles-setup-dev-early.service - Create Static Device Nodes in /dev gracefully...
         Starting systemd-tmpfiles-setup-de… Device Nodes in /dev gracefully...
[    0.227939] systemd[1]: systemd-tpm2-setup-early.service - TPM2 SRK Setup (Early) was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    0.230311] systemd[1]: Starting systemd-udev-trigger.service - Coldplug All udev Devices...
         Starting systemd-udev-trigger.service - Coldplug All udev Devices...
[    0.230970] systemd[1]: Started systemd-journald.service - Journal Service.
[  OK  ] Started systemd-journald.service - Journal Service.
[  OK  ] Mounted dev-hugepages.mount - Huge Pages File System.
[  OK  ] Mounted dev-mqueue.mount - POSIX Message Queue File System.
[  OK  ] Mounted sys-kernel-debug.mount - Kernel Debug File System.
[  OK  ] Mounted tmp.mount - Temporary Directory /tmp.
[  OK  ] Mounted var-lib-systemd.mount - /var/lib/systemd.
[  OK  ] Finished modprobe@configfs.service - Load Kernel Module configfs.
[  OK  ] Finished modprobe@dm_mod.service - Load Kernel Module dm_mod.
[  OK  ] Finished modprobe@drm.service - Load Kernel Module drm.
[  OK  ] Finished modprobe@efi_pstore.service - Load Kernel Module efi_pstore.
[  OK  ] Finished modprobe@fuse.service - Load Kernel Module fuse.
[  OK  ] Finished modprobe@loop.service - Load Kernel Module loop.
[  OK  ] Finished systemd-modules-load.service - Load Kernel Modules.
[  OK  ] Finished systemd-remount-fs.servic…mount Root and Kernel File Systems.
[  OK  ] Finished systemd-tmpfiles-setup-de…ic Device Nodes in /dev gracefully.
         Mounting sys-kernel-config.mount - Kernel Configuration File System...
         Starting systemd-journal-flush.ser…sh Journal to Persistent Storage...
         Starting systemd-random-seed.service - Load/Save OS Random Seed...
         Starting systemd-sysctl.service - Apply Kernel Variables...
         Starting systemd-tmpfiles-setup-de…eate Static Device Nodes in /dev...
[  OK  ] Mounted sys-kernel-config.mount - Kernel Configuration File System.
[    0.273903] systemd-journald[113]: Received client request to flush runtime journal.
[  OK  ] Finished systemd-journal-flush.ser…lush Journal to Persistent Storage.
[  OK  ] Finished systemd-tmpfiles-setup-de…Create Static Device Nodes in /dev.
[  OK  ] Reached target local-fs-pre.target…Preparation for Local File Systems.
[  OK  ] Reached target local-fs.target - Local File Systems.
[  OK  ] Listening on systemd-sysext.socket…tension Image Management (Varlink).
         Starting systemd-tmpfiles-setup.se…e Volatile Files and Directories...
         Starting systemd-udevd.service - R…ager for Device Events and Files...
[  OK  ] Finished systemd-sysctl.service - Apply Kernel Variables.
[  OK  ] Finished systemd-tmpfiles-setup.se…ate Volatile Files and Directories.
         Starting systemd-update-utmp.servi…ord System Boot/Shutdown in UTMP...
[  OK  ] Finished systemd-update-utmp.servi…ecord System Boot/Shutdown in UTMP.
[  OK  ] Started systemd-udevd.service - Ru…anager for Device Events and Files.
[  OK  ] Finished systemd-udev-trigger.service - Coldplug All udev Devices.
[  OK  ] Reached target sysinit.target - System Initialization.
[  OK  ] Started apt-daily.timer - Daily apt download activities.
[  OK  ] Started apt-daily-upgrade.timer - …y apt upgrade and clean activities.
[  OK  ] Started dpkg-db-backup.timer - Daily dpkg database backup timer.
[  OK  ] Started systemd-tmpfiles-clean.tim…y Cleanup of Temporary Directories.
[  OK  ] Reached target timers.target - Timer Units.
[  OK  ] Listening on ssh.socket - OpenBSD Secure Shell server socket.
[  OK  ] Reached target sockets.target - Socket Units.
[  OK  ] Reached target basic.target - Basic System.
         Starting fcnet.service...
         Starting getty-static.service - ge…bus and logind are not available...
         Starting systemd-user-sessions.service - Permit User Sessions...
[  OK  ] Found device dev-hvc0.device - /dev/hvc0.
[  OK  ] Finished fcnet.service.
[  OK  ] Finished systemd-user-sessions.service - Permit User Sessions.
[  OK  ] Started getty@tty1.service - Getty on tty1.
[  OK  ] Started serial-getty@hvc0.service - Serial Getty on hvc0.
[  OK  ] Finished getty-static.service - ge… dbus and logind are not available.
[  OK  ] Started getty@tty2.service - Getty on tty2.
[  OK  ] Started getty@tty3.service - Getty on tty3.
[  OK  ] Started getty@tty4.service - Getty on tty4.
[  OK  ] Started getty@tty5.service - Getty on tty5.
[  OK  ] Started getty@tty6.service - Getty on tty6.
[  OK  ] Reached target getty.target - Login Prompts.
[  OK  ] Reached target multi-user.target - Multi-User System.
[  OK  ] Reached target graphical.target - Graphical Interface.
         Starting systemd-update-utmp-runle…- Record Runlevel Change in UTMP...
[  OK  ] Finished systemd-update-utmp-runle…e - Record Runlevel Change in UTMP.
[  OK  ] Finished systemd-random-seed.service - Load/Save OS Random Seed.

Ubuntu 24.04.1 LTS ubuntu-fc-uvm hvc0

ubuntu-fc-uvm login: root
Welcome to Ubuntu 24.04.1 LTS (GNU/Linux 6.1.102 aarch64)

 * Documentation:  https://help.ubuntu.com
 * Management:     https://landscape.canonical.com
 * Support:        https://ubuntu.com/pro

This system has been minimized by removing packages and content that are
not required on a system that users do not log into.

To restore this content, you can run the 'unminimize' command.
root@ubuntu-fc-uvm:~# openssl speed
Doing md5 for 3s on 16 size blocks: 18025529 md5's in 2.99s
Doing md5 for 3s on 64 size blocks: 10895099 md5's in 3.00s
Doing md5 for 3s on 256 size blocks:
Vmm is stopping.
Stopping VM...

marshalllee@MarshalldeMacBook-Air firecracker_run %
```
