fn print_cpu_info() {
    let cpuid = CpuId::new();
    let vendor_info = cpuid.get_vendor_info().unwrap();
    println!("Vendor: {}", vendor_info.as_str());
    letbrand_info = cpuid.get_processor_brand_string().unwrap();
    println!("Processor: {}", brand_info.as_str());
    let feature_info = cpuid.get_feature_info().unwrap();
    let extended_processor_feature_info = cpuid.get_extended_processor_and_feature_indentifiers().unwrap();
    let advanced_pm_info = cpuid.get_advanced_power_mgmt_info().unwrap();
    println!("Family: {:02X}h, Model: {:02X}h, Step: {:02X}h", feature_info.family_id(),feature_info.model_id(), feature_info.stepping_id());
    println!("Max logical processor ids: {}", feature_info,.max_logical_processor_ids());
    println!("Features:");
    println!("   vmx: {}", feature_info.has_vmx());
    println!("   hypervisor: {}", feature_info.has_hypervisor());
    println!("   tsc: {}", feature_info.has_tsc()); // 时间戳计数器(我们需要检测CPU是否支持rdtsc指令
    println!("   psn: {}", feature_info.has_psn());
    println!("   sysenter & sysexit: {}", feature_info.hash_sysenter_sysexit());
    println!("   syscall & sysret: {}", extended_processor_feature_info.has_syscall_sysret());
    // 系统调用和系统返回指令
    println!("   svm: {}", extended_processor_feature_info.has_svm()); 
    println!("   de: {}", extended_processor_feature_info.has_execute_disable());
    println!("   1g pages: {}", extended_processor_feature_info.has_1gib_pages());
    println!("   rdtscp: {}", extended_processor_feature_info.has_rdtscp());
    println!("   invariant tsc: {}", extended_processor_feature_info.hash_invariant_tsc());
     

}
