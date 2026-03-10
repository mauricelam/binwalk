use crate::extractors::common::{ExtractionResult, Extractor, ExtractorType};
#[cfg(not(target_arch = "wasm32"))]
use crate::extractors::common::Chroot;

/// Defines the internal extractor function for decrypting known encrypted firmware
///
/// ```
/// use std::io::ErrorKind;
/// use std::process::Command;
/// use binwalk::extractors::common::ExtractorType;
/// use binwalk::extractors::encfw::encfw_extractor;
///
/// match encfw_extractor().utility {
///     ExtractorType::None => panic!("Invalid extractor type of None"),
///     ExtractorType::Internal(func) => println!("Internal extractor OK: {:?}", func),
///     ExtractorType::External(cmd) => {
///         if let Err(e) = Command::new(&cmd).output() {
///             if e.kind() == ErrorKind::NotFound {
///                 panic!("External extractor '{}' not found", cmd);
///             } else {
///                 panic!("Failed to execute external extractor '{}': {}", cmd, e);
///             }
///         }
///     }
/// }
/// ```
pub fn encfw_extractor() -> Extractor {
    Extractor {
        utility: ExtractorType::Internal(encfw_decrypt),
        ..Default::default()
    }
}

/// Attempts to decrypt known encrypted firmware images
pub fn encfw_decrypt(
    file_data: &[u8],
    offset: usize,
    output_directory: Option<&str>,
) -> ExtractionResult {
    #[cfg(not(target_arch = "wasm32"))]
    const OUTPUT_FILE_NAME: &str = "decrypted.bin";

    #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
    let mut result = ExtractionResult {
        ..Default::default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(decrypted_data) = delink::decrypt(&file_data[offset..]) {
            result.success = true;

            // Write to file, if requested
            if output_directory.is_some() {
                let chroot = Chroot::new(output_directory);
                result.success = chroot.create_file(OUTPUT_FILE_NAME, &decrypted_data);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = (file_data, offset, output_directory);
    }

    result
}
