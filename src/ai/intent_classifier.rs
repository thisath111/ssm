// intent_classifier.rs
// Pure-Rust Zero-Overhead AI Process Intent Classifier.
// Dynamically classifies process intent using behavioral telemetry, thread counts,
// memory footprint, and window/session topology with ZERO hardcoded 3rd party names.

/// Process Intent Category predicted by the AI engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessIntent {
    Gaming,
    CreativeWorkstation,
    SoftwareDevelopment,
    InputMethodEditor,
    InteractiveUi,
    BackgroundWorker,
    SystemCore,
}

impl ProcessIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gaming => "Gaming / Real-Time 3D",
            Self::CreativeWorkstation => "Creative Workstation (Video/3D/Audio)",
            Self::SoftwareDevelopment => "Developer / Compilation",
            Self::InputMethodEditor => "Input Method / Real-Time Typing Helper",
            Self::InteractiveUi => "Interactive UI / Productivity",
            Self::BackgroundWorker => "Background Worker / Daemon",
            Self::SystemCore => "Windows System Core",
        }
    }
}

pub struct ProcessIntentClassifier;

impl ProcessIntentClassifier {
    /// Dynamically classifies a process using pure behavioural metrics & OS topologies.
    pub fn classify(
        name: &str,
        memory_mb: u64,
        cpu_usage: f32,
        is_foreground: bool,
        thread_count: usize,
        has_window: bool,
    ) -> ProcessIntent {
        let name_lower = name.to_lowercase();

        // 1. Critical System Core
        if Self::is_system_core(&name_lower) {
            return ProcessIntent::SystemCore;
        }

        // 2. Gaming / Real-time 3D Simulation (Dynamic: Foreground + High memory + Multi-threaded)
        if is_foreground && memory_mb >= 600 && thread_count >= 12 && cpu_usage >= 5.0 {
            return ProcessIntent::Gaming;
        }

        // 3. Creative / Media Heavy Workstation (High Memory + Extreme Thread Count)
        if memory_mb >= 2000 && thread_count >= 24 && cpu_usage >= 25.0 {
            return ProcessIntent::CreativeWorkstation;
        }

        // 4. Input Method Editor / Typing Tool / UI Hook Helper (Dynamic: Has Window, Low Footprint, Responsive)
        // Automatically captures Helakuru, Keyman, Wijesekara, AutoHotkey, IMEs without hardcoding
        if has_window && !is_foreground && memory_mb <= 350 && thread_count <= 20 {
            return ProcessIntent::InputMethodEditor;
        }

        // 5. Interactive UI / General Foreground Application
        if is_foreground || has_window {
            return ProcessIntent::InteractiveUi;
        }

        // 6. Background Worker by default
        ProcessIntent::BackgroundWorker
    }

    fn is_system_core(name: &str) -> bool {
        matches!(
            name,
            "system"
                | "csrss.exe"
                | "lsass.exe"
                | "winlogon.exe"
                | "services.exe"
                | "dwm.exe"
                | "smss.exe"
                | "fontdrvhost.exe"
                | "explorer.exe"
                | "audiodg.exe"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classification() {
        assert_eq!(
            ProcessIntentClassifier::classify("dwm.exe", 150, 2.0, false, 8, true),
            ProcessIntent::SystemCore
        );

        // Gaming detected dynamically by metrics
        assert_eq!(
            ProcessIntentClassifier::classify("some_game.exe", 2000, 35.0, true, 32, true),
            ProcessIntent::Gaming
        );

        // Creative heavy workstation detected by metrics
        assert_eq!(
            ProcessIntentClassifier::classify("render_node.exe", 4500, 80.0, false, 64, true),
            ProcessIntent::CreativeWorkstation
        );

        // Background typing helper / IME detected dynamically (has window, low RAM, low threads)
        assert_eq!(
            ProcessIntentClassifier::classify("custom_typing_tool.exe", 85, 0.5, false, 6, true),
            ProcessIntent::InputMethodEditor
        );

        // Headless worker
        assert_eq!(
            ProcessIntentClassifier::classify("daemon_worker.exe", 40, 0.1, false, 2, false),
            ProcessIntent::BackgroundWorker
        );
    }
}
