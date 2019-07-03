use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CheckOpt {
    #[structopt(long = "interface", short = "i", raw(multiple = "true"))]
    pub interface_files: Vec<PathBuf>,
    pub wasm_module_file: PathBuf,
}

fn load_interfaces(interface_paths: &[PathBuf]) -> Result<Vec<wasm_interface::Interface>, String> {
    let mut ret = vec![];
    for ip in interface_paths {
        let interface_contents = std::fs::read_to_string(&ip).map_err(|e| {
            format!(
                "Could not read interface file {}: {}",
                ip.to_string_lossy(),
                e
            )
        })?;
        let interface =
            wasm_interface::parser::parse_interface(&interface_contents).map_err(|e| {
                format!(
                    "Failed to parse interface in file {}: {}",
                    ip.to_string_lossy(),
                    e
                )
            })?;
        ret.push(interface);
    }

    Ok(ret)
}

pub fn check(check_opt: CheckOpt) -> Result<(), String> {
    if check_opt.interface_files.len() == 0 {
        println!("WARN: no interfaces given, checking against the empty interface");
    }
    let interfaces = load_interfaces(&check_opt.interface_files)?;

    let mut interface = wasm_interface::Interface::default();
    for int in interfaces {
        interface = interface.merge(int)?;
    }

    let wasm = std::fs::read(&check_opt.wasm_module_file)
        .map_err(|e| format!("Could not read in wasm module data: {}", e))?;
    match wasm_interface::validate::validate_wasm_and_report_errors(&wasm, &interface) {
        Ok(_) => {
            println!(
                "✅ Module satisfies interface{}",
                if check_opt.interface_files.len() > 1 {
                    "s"
                } else {
                    ""
                }
            );
            Ok(())
        }
        Err(wasm_interface::validate::WasmValidationError::InvalidWasm { error }) => {
            Err(format!("Wasm module is invalid: {}", error))
        }
        Err(wasm_interface::validate::WasmValidationError::InterfaceViolated { errors }) => {
            Err(format!(
                "Wasm interface violated, {} errors detected: {}",
                errors.len(),
                errors
                    .into_iter()
                    .fold(String::new(), |a, b| a + "\n❌ " + &b)
            ))
        }
        Err(wasm_interface::validate::WasmValidationError::UnsupportedType { error }) => {
            Err(format!("Unsupported type found in Wasm module: {}", error))
        }
    }
}
