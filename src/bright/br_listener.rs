use anyhow::{Result, bail};
use async_channel::Sender;
use nix::libc;
use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentBrightness {
    pub level: f64, // Porcentaje de 0.0 a 1.0
}

impl CurrentBrightness {
    pub fn new(s: &str) -> Result<Self> {
        // brightnessctl -m escupe algo como: intel_backlight,backlight,12000,0,120000,10%
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() < 5 {
            bail!("Unexpected format from brightnessctl: {}", s);
        }

        // Limpiamos el símbolo de porcentaje y parseamos
        let percent_str = parts[3].replace("%", "");
        let level = percent_str.trim().parse::<f64>().unwrap_or(0.0) / 100.0;

        Ok(Self { level })
    }
}

pub fn get_current_brightness() -> Result<CurrentBrightness> {
    let output = Command::new("brightnessctl")
        .arg("-m") // Modo parseable
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // Si hay varios dispositivos (ej. teclado y pantalla), brightnessctl -m tira varias líneas.
        // Tomamos la principal (la primera).
        let first_line = output_str.lines().next().unwrap_or("");
        CurrentBrightness::new(first_line)
    } else {
        bail!("Error al ejecutar \"brightnessctl\"")
    }
}

pub fn watch_brightness_changes(sender: Sender<CurrentBrightness>) {
    std::thread::spawn(move || unsafe {
        // El truco: udev sabe exactamente cuándo el hardware de retroiluminación cambia.
        // No necesitamos privilegios root para monitorizar esto.
        let mut child = Command::new("udevadm")
            .args(["monitor", "--subsystem-match=backlight"])
            .stdout(Stdio::piped())
            .pre_exec(|| {
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
                Ok(())
            })
            .spawn()
            .expect("Error al ejecutar \"udevadm\"");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut last_brightness: Option<CurrentBrightness> = None;

        for line in reader.lines().flatten() {
            // Buscamos eventos de cambio (change) reportados por udev
            if line.contains("change") {
                if let Ok(brightness) = get_current_brightness() {
                    if Some(&brightness) != last_brightness.as_ref() {
                        last_brightness = Some(brightness.clone());
                        let _ = sender.send_blocking(brightness);
                    }
                }
            }
        }
    });
}
