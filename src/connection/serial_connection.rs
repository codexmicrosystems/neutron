// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

pub const CR: u8 = 13;
pub const LF: u8 = 10;

pub struct SerialConnection {
    serial_port: Box<dyn SerialPort>,
    serial_path: String,
    baud_rate: u32,
    data_bits: DataBits,
    flow_control: FlowControl,
    parity: Parity,
    stop_bits: StopBits,
    timeout: Duration,
}

impl SerialConnection {
    #[must_use]
    pub fn new(&self, serial_path: String) -> Self {
        let baud_rate: u32 = 115200;
        let data_bits: DataBits = DataBits::Eight;
        let flow_control: FlowControl = FlowControl::None;
        let parity: Parity = Parity::None;
        let stop_bits: StopBits = StopBits::One;
        let timeout: Duration = Duration::from_millis(10);

        let serial_port: Box<dyn SerialPort> = serialport::new(self.serial_path.clone(), self.baud_rate)
            .data_bits(self.data_bits)
            .flow_control(self.flow_control)
            .parity(self.parity)
            .stop_bits(self.stop_bits)
            .timeout(self.timeout)
            .open()
            .expect("[ERROR]: Unable to Open Serial Port");

        return Self {
            serial_port,
            serial_path,
            baud_rate,
            data_bits,
            flow_control,
            parity,
            stop_bits,
            timeout,
        };
    }

    pub fn set_params(
        &mut self,
        baud_rate: u32,
        data_bits: DataBits,
        flow_control: FlowControl,
        parity: Parity,
        stop_bits: StopBits,
        timeout: Duration,
    ) {
        self.baud_rate = baud_rate;
        self.data_bits = data_bits;
        self.flow_control = flow_control;
        self.parity = parity;
        self.stop_bits = stop_bits;
        self.timeout = timeout;
    }

    pub fn write_string(&mut self, string: &str) {
        self.serial_port
            .write(string.as_bytes())
            .expect("[ERROR]: Unable to Write String to Serial Port");
    }

    pub fn terminate_string(&mut self, string: &str, terminator: u8) -> String {
        return string.to_owned() + &terminator.to_string();
    }

    pub fn send_command(&mut self, command: &str, terminator: u8) {
        let terminated_command = &self.terminate_string(command, terminator);

        self.write_string(terminated_command);
    }
}
