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

        // Fallback: If no distinct E-cores detected, split upper/lower halves as soft affinity groups.
        // Guard: if only 1-2 cores exist, disable splitting to prevent mask=0 stalls.
        if topology.p_core_mask == usize::MAX || topology.e_core_mask == usize::MAX {
            let total = topology.total_logical_cores;
            if total >= 4 {
                let half = total / 2;
                let all_mask = (1usize << total).saturating_sub(1);
                topology.p_core_mask = (1 << half) - 1;
                topology.e_core_mask = !topology.p_core_mask & all_mask;
            }
            // For 2-core or fewer: keep usize::MAX (use all cores — no splitting)
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
