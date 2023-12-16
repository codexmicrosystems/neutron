// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

pub const CR: u8 = 13;
pub const LF: u8 = 10;

pub struct SerialConnectionConfig {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub timeout: Duration,
}

impl SerialConnectionConfig {
    pub fn default(&self) -> Self {
        let baud_rate: u32 = 115200;
        let data_bits: DataBits = DataBits::Eight;
        let flow_control: FlowControl = FlowControl::None;
        let parity: Parity = Parity::None;
        let stop_bits: StopBits = StopBits::One;
        let timeout: Duration = Duration::from_millis(10);

        return Self {
            baud_rate,
            data_bits,
            flow_control,
            parity,
            stop_bits,
            timeout,
        };
    }

    pub fn new(
        &self,
        baud_rate: u32,
        data_bits: DataBits,
        flow_control: FlowControl,
        parity: Parity,
        stop_bits: StopBits,
        timeout: Duration,
    ) -> Self {
        return Self {
            baud_rate,
            data_bits,
            flow_control,
            parity,
            stop_bits,
            timeout,
        };
    }
}

pub struct SerialConnection {
    serial_port: Box<dyn SerialPort>,
    serial_path: String,
    serial_connection_config: SerialConnectionConfig,
}

impl SerialConnection {
    #[must_use]
    pub fn new(serial_path: String, serial_connection_config: SerialConnectionConfig) -> Self {
        let serial_port: Box<dyn SerialPort> = serialport::new(serial_path.clone(), serial_connection_config.baud_rate)
            .data_bits(serial_connection_config.data_bits)
            .flow_control(serial_connection_config.flow_control)
            .parity(serial_connection_config.parity)
            .stop_bits(serial_connection_config.stop_bits)
            .timeout(serial_connection_config.timeout)
            .open()
            .expect("[ERROR]: Unable to Open Serial Port");

        return Self {
            serial_port,
            serial_path,
            serial_connection_config,
        };
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
