use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const CMD_CLONE_SPIRV_HEADERS: [&'static str; 4] = [
    "git", "clone", "https://github.com/KhronosGroup/SPIRV-Headers.git", "spirv-tools/external/spirv-headers"
];
const CMD_CLONE_EFFCEE: [&'static str; 4] = [
    "git", "clone", "https://github.com/google/effcee.git", "spirv-tools/external/effcee"
];
const CMD_CLONE_RE2: [&'static str; 4] = [
    "git", "clone", "https://github.com/google/re2.git", "spirv-tools/external/re2"
];

/// Describes where in the build process an error occured
#[derive(Debug)]
enum BuildError {
    CloneDependencies(&'static str),
    ConfigureProject,
    BuildProject
}

/// Check for and build spirv-tools, then configure the compiler to 
fn main() -> Result<(), BuildError> {
    clone_dependencies()?;
    configure_project()?;
    build_project()?;

    // Point the compiler at the built library
    let current_dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}\\spirv-tools\\build\\source\\Release", current_dir.display());
    println!("cargo:rustc-link-search=native={}\\spirv-tools\\build\\source\\opt\\Release", current_dir.display());
    println!("cargo:rustc-link-lib=static=SPIRV-Tools");
    println!("cargo:rustc-link-lib=static=SPIRV-Tools-opt");

    Ok(())
}

/// Clone any missing dependencies
fn clone_dependencies() -> Result<(), BuildError> {
    let path = PathBuf::from("./spirv-tools/external/spirv-headers");
    if !path.exists() {
        Command::new("cmd")
            .arg("/C")
            .args(&CMD_CLONE_SPIRV_HEADERS)
            .stdout(Stdio::inherit())
            .output()
            .map_err(|_| BuildError::CloneDependencies("Failed to clone spirv-headers"))?;
    }

    let path = PathBuf::from("./spirv-tools/external/effcee");
    if !path.exists() {
        Command::new("cmd")
            .arg("/C")
            .args(&CMD_CLONE_EFFCEE)
            .stdout(Stdio::inherit())
            .output()
            .map_err(|_| BuildError::CloneDependencies("Failed to clone effcee"))?;
    }

    let path = PathBuf::from("./spirv-tools/external/re2");
    if !path.exists() {
        Command::new("cmd")
            .arg("/C")
            .args(&CMD_CLONE_RE2)
            .stdout(Stdio::inherit())
            .output()
            .map_err(|_| BuildError::CloneDependencies("Failed to clone re2"))?;
    }

    Ok(())
}

/// Configure CMake for building
fn configure_project() -> Result<(), BuildError> {
    Command::new("cmd")
        .arg("/C")
        .args(&["mkdir", "spirv-tools/build", "&", "cd", "spirv-tools/build", "&&", "cmake", "../"])
        .stdout(Stdio::inherit())
        .output()
        .map_err(|_| BuildError::ConfigureProject)?;

    Ok(())
}

/// Invoke CMake and build the project
fn build_project() -> Result<(), BuildError> {
    Command::new("cmd")
        .arg("/C")
        .args(&["cmake", "--build", "./spirv-tools/build", "--config", "Release"])
        .stdout(Stdio::inherit())
        .output()
        .map_err(|_| BuildError::BuildProject)?;

    Ok(())
}
