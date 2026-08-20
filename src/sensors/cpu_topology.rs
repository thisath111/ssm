use windows::Win32::System::SystemInformation::{
    GetLogicalProcessorInformationEx, RelationProcessorCore,
    SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
};

#[derive(Debug, Clone)]
pub struct CpuCoreInfo {
    pub core_index: usize,
    pub affinity_mask: usize,
    pub is_performance_core: bool,
}

#[derive(Debug, Clone)]
pub struct CpuTopology {
    pub total_logical_cores: usize,
    pub physical_cores: usize,
    pub p_core_mask: usize,
    pub e_core_mask: usize,
    pub cores: Vec<CpuCoreInfo>,
}

impl CpuTopology {
    pub fn detect() -> Self {
        let mut total_logical = num_cpus();
        if total_logical == 0 {
            total_logical = 4;
        }

        let mut topology = Self {
            total_logical_cores: total_logical,
            physical_cores: total_logical / 2,
            p_core_mask: usize::MAX,
            e_core_mask: usize::MAX,
            cores: Vec::new(),
        };

        unsafe {
            let mut buf_size: u32 = 0;
            let _ = GetLogicalProcessorInformationEx(
                RelationProcessorCore,
                None,
                &mut buf_size,
            );

            if buf_size > 0 {
                let mut buffer: Vec<u8> = vec![0; buf_size as usize];
                if GetLogicalProcessorInformationEx(
                    RelationProcessorCore,
                    Some(buffer.as_mut_ptr() as *mut _),
                    &mut buf_size,
                )
                .is_ok()
                {
                    let mut p_mask: usize = 0;
                    let mut e_mask: usize = 0;
                    let mut ptr = buffer.as_ptr();
                    let end = ptr.add(buf_size as usize);

                    let mut core_idx = 0;
                    while ptr < end {
                        let info = &*(ptr as *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX);
                        if info.Relationship == RelationProcessorCore {
                            let core = &info.Anonymous.Processor;
                            let group_mask = core.GroupMask[0].Mask as usize;
                            let efficiency = core.EfficiencyClass;

                            let is_p_core = efficiency > 0;
                            if is_p_core {
                                p_mask |= group_mask;
                            } else {
                                e_mask |= group_mask;
                            }

                            topology.cores.push(CpuCoreInfo {
                                core_index: core_idx,
                                affinity_mask: group_mask,
                                is_performance_core: is_p_core,
                            });
                            core_idx += 1;
                        }

                        let size = info.Size as usize;
                        if size == 0 {
                            break;
                        }
                        ptr = ptr.add(size);
                    }

                    if p_mask > 0 && e_mask > 0 {
                        topology.p_core_mask = p_mask;
                        topology.e_core_mask = e_mask;
                    }
                }
            }
        }

        // Fallback: If no distinct E-cores detected, assign upper 50% as secondary, lower 50% as primary
        if topology.p_core_mask == usize::MAX || topology.e_core_mask == usize::MAX {
            let half = topology.total_logical_cores / 2;
            if half > 0 {
                topology.p_core_mask = (1 << half) - 1;
                topology.e_core_mask = !topology.p_core_mask & ((1 << topology.total_logical_cores) - 1);
            }
        }

        topology
    }
}

fn num_cpus() -> usize {
    unsafe {
        let mut sys_info = std::mem::zeroed();
        windows::Win32::System::SystemInformation::GetSystemInfo(&mut sys_info);
        sys_info.dwNumberOfProcessors as usize
    }
}
