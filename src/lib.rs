pub mod script;
pub mod dashboard_server;
pub mod network;
pub enum DefaultPortName {
    Primary,
    Secondary,
    RealTime,
    RealTimeDataExchange,
    DashboardServer,
}

pub fn get_default_port_number(port_name :DefaultPortName) -> u32 {
    match port_name {
        DefaultPortName::Primary => 30001,
        DefaultPortName::Secondary => 30002,
        DefaultPortName::RealTime => 30003,
        DefaultPortName::RealTimeDataExchange => 30004,
        DefaultPortName::DashboardServer=> 2999,
    }
}
