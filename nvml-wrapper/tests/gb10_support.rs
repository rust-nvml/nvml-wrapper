// Test to verify that the GPU architecture is recognized (e.g., Blackwell/GB10)

#[cfg(test)]
mod tests {
    use nvml_wrapper::enums::device::DeviceArchitecture;
    use nvml_wrapper::Nvml;

    #[test]
    fn test_gpu_architecture_recognized() {
        // Initialize NVML (will error if library not present)
        let nvml = Nvml::init().expect("NVML should initialize");
        // Get first device (index 0). May panic if no GPU present.
        let device = nvml
            .device_by_index(0)
            .expect("GPU device should be present");
        let arch = device.architecture().expect("Should retrieve architecture");
        // Ensure we get a known architecture (not Unknown)
        assert_ne!(
            arch,
            DeviceArchitecture::Unknown,
            "GPU architecture reported as Unknown"
        );
        // Optionally print the architecture for manual verification
        println!("Detected GPU architecture: {}", arch);
    }
}
