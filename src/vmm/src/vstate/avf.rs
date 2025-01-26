use std::io::{stdin, stdout};

use virt_fwk::{
    VirtualMachine,
    VirtualMachineConfiguration,
    LinuxBootLoader,
    DiskImageStorageDeviceAttachment,
    FileHandleSerialPortAttachment,
    VirtioConsoleDeviceSerialPortConfiguration,
};

#[derive(Debug)]
pub enum AvfError {
    Unsupported,
    InvalidConfiguration,
}

#[derive(Debug)]
pub struct Avf {
    config: VirtualMachineConfiguration,
}

impl Avf {
    pub fn new(
        kernel_path: &str,
        command_line: &str,
        cpu_count: u8,
        mem_size_mib: u64
    ) -> Result<Self, AvfError> {

        if !VirtualMachine::supported() {
            println!("Apple Virtualization Framework is not supported on this system.");
            return Err(AvfError::Unsupported);
        }

        let initrd_url = String::new();
        let boot_loader = LinuxBootLoader::new(&kernel_path, &initrd_url, command_line);

        let config = VirtualMachineConfiguration::new(boot_loader, cpu_count, mem_size_mib as u64 << 20);

        // serial port
        let std_in = stdin();
        let std_out = stdout();
        let attachment =
            FileHandleSerialPortAttachment::new(&std_in, &std_out);
        let serial_port =
            VirtioConsoleDeviceSerialPortConfiguration::new_with_attachment(attachment);

        // config validation
        if let Err(msg) = config.validate() {
            println!("Invalid Configuration: {}", msg);
            return Err(AvfError::InvalidConfiguration);
        }

        Ok(Self (VirtualMachine::new(&config)))
    }

    pub fn attach_block_devices(
        &mut self,
        block_devices: Vec<VirtioBlockDeviceConfiguration>
    ) -> Result<(), AvfError> {
        self.config.set_storage_devices(block_devices);

        // config validation
        if let Err(msg) = self.config.validate() {
            println!("Invalid Configuration: {}", msg);
            return Err(AvfError::InvalidConfiguration);
        }

        Ok(())
    }
}
