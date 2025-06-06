use crate::environment::PluginEnvironmentError::PluginEnvironmentDisabled;

use crate::virtualenv::{VenvError, find_python, initialize_venv};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginEnvironmentError {
    #[error("Package manager not available: {0}")]
    PackageManagerNotFound(String),
    #[error("External call failed: {0}")]
    InstallationFailed(#[from] std::io::Error),
    #[error("Plugin environment management is disabled")]
    PluginEnvironmentDisabled,

    #[error("Virtual environment error: {0}")]
    VenvError(#[from] VenvError),
}

pub trait PythonEnvironmentManager: Debug + Send + Sync + 'static {
    fn init_pyenv(
        &self,
        plugin_dir: &Path,
        virtual_env_location: Option<&PathBuf>,
    ) -> Result<(), PluginEnvironmentError>;
    fn install_packages(&self, packages: Vec<String>) -> Result<(), PluginEnvironmentError>;

    fn install_requirements(&self, requirements_path: String)
    -> Result<(), PluginEnvironmentError>;
}

#[derive(Debug, Copy, Clone)]
pub struct UVManager;
#[derive(Debug, Copy, Clone)]
pub struct PipManager;

#[derive(Debug, Copy, Clone)]
pub struct DisabledManager;

fn is_valid_venv(venv_path: &Path) -> bool {
    if cfg!(windows) {
        venv_path.join("Scripts").join("activate.bat").exists()
    } else {
        venv_path.join("bin").join("activate").exists()
    }
}

impl PythonEnvironmentManager for UVManager {
    fn init_pyenv(
        &self,
        plugin_dir: &Path,
        virtual_env_location: Option<&PathBuf>,
    ) -> Result<(), PluginEnvironmentError> {
        let venv_path = match virtual_env_location {
            Some(path) => path,
            None => &plugin_dir.join(".venv"),
        };

        if !is_valid_venv(venv_path) {
            Command::new("uv").arg("venv").arg(venv_path).output()?;
        }

        initialize_venv(venv_path)?;
        Ok(())
    }

    fn install_packages(&self, packages: Vec<String>) -> Result<(), PluginEnvironmentError> {
        Command::new("uv")
            .args(["pip", "install"])
            .args(&packages)
            .output()?;
        Ok(())
    }

    fn install_requirements(
        &self,
        requirements_path: String,
    ) -> Result<(), PluginEnvironmentError> {
        Command::new("uv")
            .args(["pip", "install", "-r", &requirements_path])
            .output()?;
        Ok(())
    }
}

impl PythonEnvironmentManager for PipManager {
    fn init_pyenv(
        &self,
        plugin_dir: &Path,
        virtual_env_location: Option<&PathBuf>,
    ) -> Result<(), PluginEnvironmentError> {
        let venv_path = match virtual_env_location {
            Some(path) => path,
            None => &plugin_dir.join(".venv"),
        };

        if !is_valid_venv(venv_path) {
            let python_exe = find_python();
            Command::new(python_exe)
                .arg("-m")
                .arg("venv")
                .arg(venv_path)
                .output()?;
        }

        initialize_venv(venv_path)?;
        Ok(())
    }

    fn install_packages(&self, packages: Vec<String>) -> Result<(), PluginEnvironmentError> {
        let python_exe = find_python();
        Command::new(python_exe)
            .arg("-m")
            .arg("pip")
            .arg("install")
            .args(&packages)
            .output()?;
        Ok(())
    }
    fn install_requirements(
        &self,
        requirements_path: String,
    ) -> Result<(), PluginEnvironmentError> {
        let python_exe = find_python();
        Command::new(python_exe)
            .arg("-m")
            .arg("pip")
            .args(["install", "-r", &requirements_path])
            .output()?;

        Ok(())
    }
}

impl PythonEnvironmentManager for DisabledManager {
    fn init_pyenv(
        &self,
        _plugin_dir: &Path,
        _virtual_env_location: Option<&PathBuf>,
    ) -> Result<(), PluginEnvironmentError> {
        Ok(())
    }

    fn install_packages(&self, _packages: Vec<String>) -> Result<(), PluginEnvironmentError> {
        Err(PluginEnvironmentDisabled)
    }

    fn install_requirements(
        &self,
        _requirements_path: String,
    ) -> Result<(), PluginEnvironmentError> {
        Err(PluginEnvironmentDisabled)
    }
}
