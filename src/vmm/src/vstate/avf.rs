use std::io::{stdin, stdout};

use virt_fwk::{
    VirtualMachine,
    VirtualMachineConfiguration,
    LinuxBootLoader,
    VirtioBlockDeviceConfiguration,
    FileHandleSerialPortAttachment,
    VirtioConsoleDeviceSerialPortConfiguration,
    VirtioNetworkDeviceConfiguration,
    NATNetworkDeviceAttachment,
    MACAddress,
    VirtioTraditionalMemoryBalloonDeviceConfiguration,
};

/// Errors associated with the wrappers over AVF functions.
#[derive(Debug, PartialEq, Eq, thiserror::Error, displaydoc::Display)]
pub enum AvfError {
    /// AVF is not supported on this system
    Unsupported,
    /// The AVF configuration was invalid
    InvalidConfiguration,
}


/// Wrapper for AVF.
#[derive(Debug)]
pub struct Avf {
    config: VirtualMachineConfiguration,
}

impl Avf {
    /// Creates a new `Avf` struct.
    pub fn new(
        kernel_path: &str,
        command_line: &str,
        cpu_count: u8,
        mem_size_mib: usize,
        block_devices: Vec<VirtioBlockDeviceConfiguration>
    ) -> Result<Self, AvfError> {

        if !VirtualMachine::supported() {
            println!("Apple Virtualization Framework is not supported on this system.");
            return Err(AvfError::Unsupported);
        }

        let initrd_url = String::new();
        let boot_loader = LinuxBootLoader::new(kernel_path, &initrd_url, command_line);

        let config = VirtualMachineConfiguration::new(
            boot_loader, cpu_count as usize, (mem_size_mib as u64) << 20
        );

        // serial port
        let std_in = stdin();
        let std_out = stdout();
        let attachment =
            FileHandleSerialPortAttachment::new(&std_in, &std_out);
        let serial_port =
            VirtioConsoleDeviceSerialPortConfiguration::new_with_attachment(attachment);

        config.set_serial_ports(vec![serial_port]);

        // block devices
        config.set_storage_devices(block_devices);

        // network devices
        let network_device = VirtioNetworkDeviceConfiguration::new_with_attachment(
            NATNetworkDeviceAttachment::new(),
        );
        network_device.set_mac_address(MACAddress::new_with_random_locally_administered_address());

        config.set_network_devices(vec![network_device]);

        // memory balloon
        let memory_balloon = VirtioTraditionalMemoryBalloonDeviceConfiguration::new();
        config.set_memory_balloon_devices(vec![memory_balloon]);

        // config validation
        if let Err(msg) = config.validate() {
            println!("Invalid Configuration: {}", msg);
            return Err(AvfError::InvalidConfiguration);
        }

        Ok(Self {config})
    }

    /// Creates a new virtual machine.
    pub fn create_vm(&self) -> VirtualMachine {
        VirtualMachine::new(&self.config)
    }
}
