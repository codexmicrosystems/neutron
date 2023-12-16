use neutron::connection::serial_connection::{SerialConnection, SerialConnectionConfig};

fn main() {
    let serial_conn_config: SerialConnectionConfig = SerialConnectionConfig::default();

    let serial_conn: SerialConnection = SerialConnection::new();
}
