/// Process Intent Category predicted by the AI engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessIntent {
    Gaming,
    CreativeWorkstation,
    SoftwareDevelopment,
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
            Self::InteractiveUi => "Interactive UI / Productivity",
            Self::BackgroundWorker => "Background Worker / Daemon",
            Self::SystemCore => "Windows System Core",
        }
    }
}

/// Pure-Rust Zero-Overhead AI Process Intent Classifier.
/// Uses heuristic decision trees & behavioural pattern weights for sub-microsecond classification.
pub struct ProcessIntentClassifier;

impl ProcessIntentClassifier {
    /// Classifies a process based on name, memory footprint (MB), CPU usage, and whether it holds the foreground window.
    pub fn classify(
        name: &str,
        memory_mb: u64,
        cpu_usage: f32,
        is_foreground: bool,
        thread_count: usize,
    ) -> ProcessIntent {
        let name_lower = name.to_lowercase();

        // 1. Critical System Core
        if Self::is_system_core(&name_lower) {
            return ProcessIntent::SystemCore;
        }

        // 2. Developer / Compiler Tools
        if Self::is_dev_tool(&name_lower) {
            return ProcessIntent::SoftwareDevelopment;
        }

        // 3. Creative / Media Workstations
        if Self::is_creative_app(&name_lower) || (memory_mb > 2500 && thread_count > 30 && cpu_usage > 40.0) {
            return ProcessIntent::CreativeWorkstation;
        }

        // 4. Gaming / Real-time 3D Simulation
        if Self::is_game(&name_lower) || (is_foreground && memory_mb > 1200 && cpu_usage > 10.0 && thread_count > 16) {
            return ProcessIntent::Gaming;
        }

        // 5. Interactive UI / General Apps
        if is_foreground || memory_mb > 200 {
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

    fn is_dev_tool(name: &str) -> bool {
        name.contains("code")
            || name.contains("devenv")
            || name.contains("rustc")
            || name.contains("cargo")
            || name.contains("clippy")
            || name.contains("gcc")
            || name.contains("clang")
            || name.contains("msbuild")
            || name.contains("node")
            || name.contains("git")
            || name.contains("py")
            || name.contains("idea")
            || name.contains("pycharm")
    }

    fn is_creative_app(name: &str) -> bool {
        name.contains("premiere")
            || name.contains("afterfx")
            || name.contains("photoshop")
            || name.contains("blender")
            || name.contains("davinci")
            || name.contains("resolve")
            || name.contains("cinema4d")
            || name.contains("maya")
            || name.contains("3dsmax")
            || name.contains("obs64")
            || name.contains("ffmpeg")
            || name.contains("handbrake")
            || name.contains("ableton")
            || name.contains("fl64")
    }

    fn is_game(name: &str) -> bool {
        name.contains("game")
            || name.contains("steam")
            || name.contains("unreal")
            || name.contains("unity")
            || name.contains("valorant")
            || name.contains("cs2")
            || name.contains("dota")
            || name.contains("gta")
            || name.contains("cyberpunk")
            || name.contains("fortnite")
            || name.contains("minecraft")
            || name.contains("epicgames")
            || name.contains("riotclient")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classification() {
        assert_eq!(
            ProcessIntentClassifier::classify("dwm.exe", 150, 2.0, false, 8),
            ProcessIntent::SystemCore
        );

        assert_eq!(
            ProcessIntentClassifier::classify("Code.exe", 800, 5.0, true, 24),
            ProcessIntent::SoftwareDevelopment
        );

        assert_eq!(
            ProcessIntentClassifier::classify("Blender.exe", 3500, 60.0, true, 48),
            ProcessIntent::CreativeWorkstation
        );

        assert_eq!(
            ProcessIntentClassifier::classify("Valorant.exe", 2000, 25.0, true, 32),
            ProcessIntent::Gaming
        );

        assert_eq!(
            ProcessIntentClassifier::classify("mystery_worker.exe", 50, 0.1, false, 2),
            ProcessIntent::BackgroundWorker
        );
    }
}
