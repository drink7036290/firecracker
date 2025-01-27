id $USER
ls -al /dev/kvm
[ -r /dev/kvm ] && [ -w /dev/kvm ] && echo "OK" || echo "FAIL"

# =================================

*userfaultfd
*seccomp
seccompiler
*timerfd
    *stats_timer
    *update_timer_state
rebase-snap
jailer
netns
cgroups
chroot
KVM
balloon
snapshot
micro-http
vmm_sys_util
    epoll
    eventfd
    sock_ctrl_msg
vhost
src/firecracker
fdt
acpi
DSDT
any #[cfg(test)]
*   setup_serial_device
*   mmio_device_manager
*   vcpus_exit_evt
*   attach_boot_timer_device
*   attach_legacy_devices_aarch64
*   attach_entropy_device
*   attach_net_devices
*   attach_unixsock_vsock_device
*   attach_balloon_device
*   get_bus_device
*   update_block_device
update_block_rate_limiter
update_vhost_user_block_config
update_net_rate_limiters


*   emulate_serial_init
*   EmulateSerialInit
BuildMicrovmFromSnapshotError
create_snapshot
balloon_config
latest_balloon_stats
update_balloon_config
update_balloon_stats_config

event-manager
    MutEventSubscriber
    #replaced with FileHandleSerialPortAttachment

rpc_interface

#=========
Parse JSON → produce a VmmConfig.
Validate/transform that VmmConfig → produce a VmResources.
Use VmResources in build_microvm → create the actual microVM.
#=========

[Preserve]

vmm/src/vmm_config/drive.rs
*    BlockDeviceConfig
vmm/src/devices/virtio/block/device.rs
    Block
vmm/src/vmm_config/boot_source.rs
*    BootSourceConfig
*    BootConfig


BlockDeviceConfig
VirtioBlockConfig
VirtioBlock


vmm/src/vmm_config/machine_config.rs
*    MachineConfigError
*    MachineConfig

firecracker/src/main.rs
*    main_exec()
*    run_without_api()
*    build_microvm_from_json()  # server side if no-api
*    BuildFromJsonError

vmm/src/resources.rs
*    VmmConfig
*    VmResources
*        from_json()
*    ResourcesError

Vmm/src/builder.rs
*    build_and_boot_microvm()
^        build_microvm_for_boot()
*            load_kernel()
*            load_initrd_from_config()
*                load_initrd()
^            create_vmm_and_vcpus()
*                create_vcpus()
^            attach_block_devices()
*            configure_system_for_boot()
    vmm.resume_vm()
Vmm
	instance_info
    kvm: Kvm,
    avf: Avf,
    vm: Vm,
	version()
	resume_vm()
	pause_vm()
	stop()
Vm # per vm content, vmm/src/vstate/vm.rs




#=========

#[cfg(target_os = "linux")]

#=========

cargo build --target aarch64-apple-darwin --bin firecracker

ARCH="aarch64"

mkdir -p ../firecracker_run
cp build/cargo_target/$ARCH-unknown-linux-musl/debug/firecracker ../firecracker_run/

cd ../firecracker_run
codesign -f --entitlement ./firecracker.entitlements -s - ./firecracker
sudo strace -ff -o fc-strace.log ./firecracker --no-api --config-file macos.json

#=========

macos.json

{
    "boot-source": {
        "kernel_image_path": "assets/kernel-6.1.26",
        "boot_args": "console=hvc0 root=/dev/vda"
    },
    "drives": [
        {
            "path_on_host": "assets/ubuntu-24.04.img",
            "is_read_only": false,
            "is_root_device": true
        }
    ],
    "machine-config": {
        "vcpu_count": 2,
        "mem_size_mib": 1024,
        ...
    },
}

# =================================

cd ../firecracker_run
brew install wget

latest=$(
  wget "http://spec.ccfc.min.s3.amazonaws.com/?prefix=firecracker-ci/v1.11/$ARCH/vmlinux-6.1&list-type=2" \
    -O - 2>/dev/null \
  | sed -nE 's#.*<Key>(firecracker-ci/v1\.11/'"$ARCH"'/vmlinux-6\.1\.[0-9]+)</Key>.*#\1#p' \
  | tail -1
)
echo "Found: $latest"

# Download a linux kernel binary
wget "https://s3.amazonaws.com/spec.ccfc.min/${latest}"

# Download a rootfs
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

// ==================================
