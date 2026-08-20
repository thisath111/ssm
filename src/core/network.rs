use winreg::enums::*;
use winreg::RegKey;

const QOS_POLICY_KEY: &str = r"Software\Policies\Microsoft\Windows\QoS";
const TCPIP_INTERFACES_KEY: &str = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";

pub struct NetworkOptimizer;

impl NetworkOptimizer {
    /// Applies CS1 DSCP QoS prioritization for background application traffic throttling.
    pub fn enable_qos_policy() -> std::io::Result<()> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (policy_key, _) = hklm.create_subkey(&format!("{}\\{}", QOS_POLICY_KEY, "SSM_QoS_BG"))?;

        policy_key.set_value("Version", &1u32)?;
        policy_key.set_value("DSCPValue", &"CS1".to_string())?;
        policy_key.set_value("ThrottleRate", &8u32)?;
        policy_key.set_value("Application", &"*".to_string())?;
        policy_key.set_value("Protocol", &"*".to_string())?;

        Ok(())
    }

    /// Disables TCP Nagle Algorithm (TCPNoDelay = 1, TcpAckFrequency = 1) for minimum gaming latency.
    pub fn disable_tcp_nagle() -> std::io::Result<()> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(interfaces) = hklm.open_subkey(TCPIP_INTERFACES_KEY) {
            for subkey_name in interfaces.enum_keys().flatten() {
                if let Ok(iface_key) = interfaces.open_subkey_with_flags(&subkey_name, KEY_ALL_ACCESS) {
                    let _ = iface_key.set_value("TcpAckFrequency", &1u32);
                    let _ = iface_key.set_value("TCPNoDelay", &1u32);
                    let _ = iface_key.set_value("TcpDelAckTicks", &0u32);
                }
            }
        }
        Ok(())
    }
}
