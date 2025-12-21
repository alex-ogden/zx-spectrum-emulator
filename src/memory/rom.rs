use std::fs;

const ZX_SPECTRUM_48K_ROM_SIZE: usize = 0x4000; // 16KB ROM

pub fn load_rom(rom_path: &str) -> Result<Vec<u8>, String> {
    let rom_data = fs::read(rom_path).map_err(|e| format!("Failed to read ROM: {}", e))?;

    // Verify ROM size
    if rom_data.len() != ZX_SPECTRUM_48K_ROM_SIZE {
        return Err(format!(
            "ROM is not expected size: Expected {} bytes, got {} bytes",
            ZX_SPECTRUM_48K_ROM_SIZE,
            rom_data.len()
        ));
    }

    Ok(rom_data)
}
