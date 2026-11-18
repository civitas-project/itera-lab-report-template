// @impl:FR-CLI-19
//! Bilingual localization engine and language management.

use std::fmt;

/// Supported interface languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// English (default international standard).
    #[default]
    En,
    /// Bahasa Indonesia.
    Id,
}

impl Language {
    /// Returns canonical ISO language code ("en" or "id").
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Id => "id",
        }
    }

    /// Parses language from user input string or environment variable.
    #[must_use]
    pub fn from_input(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "2" | "id" | "indonesia" | "indonesian" | "in" => Self::Id,
            _ => Self::En,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Catalog of standard system message keys for localization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKey {
    /// Welcome banner heading.
    Welcome,
    /// Prompt for language selection.
    LanguagePrompt,
    /// Prompt for student full name.
    NamePrompt,
    /// Prompt for student identification number (NIM).
    NimPrompt,
    /// Prompt for student class / section.
    ClassPrompt,
    /// Prompt offering initial project scaffolding.
    ScaffoldPrompt,
    /// Confirmation message after profile initialization.
    SuccessInit,
    /// Recommended next commands.
    NextSteps,
    /// Lint passed message.
    LintPassed,
    /// Lint failed message.
    LintFailed,
    /// Compilation success report.
    CompilationSuccess,
    /// Verification passed report.
    VerificationPassed,
    /// Verification failed report.
    VerificationFailed,
    /// Submission archive packaged report.
    PackSuccess,
}

/// Resolves a localized message text for a specific [`Language`] and [`MessageKey`].
#[must_use]
pub fn tr(lang: Language, key: MessageKey) -> &'static str {
    match (lang, key) {
        (Language::En, MessageKey::Welcome) => "🎓 Welcome to ITERA Lab Report Toolchain!",
        (Language::Id, MessageKey::Welcome) => "🎓 Selamat datang di Toolchain Laporan Praktikum ITERA!",

        (Language::En, MessageKey::LanguagePrompt) => "Choose language: [1] English (default), [2] Bahasa Indonesia",
        (Language::Id, MessageKey::LanguagePrompt) => "Pilih bahasa: [1] English, [2] Bahasa Indonesia (default)",

        (Language::En, MessageKey::NamePrompt) => "Full Name",
        (Language::Id, MessageKey::NamePrompt) => "Nama Lengkap",

        (Language::En, MessageKey::NimPrompt) => "Student ID (NIM)",
        (Language::Id, MessageKey::NimPrompt) => "NIM",

        (Language::En, MessageKey::ClassPrompt) => "Class (e.g. RA, RB)",
        (Language::Id, MessageKey::ClassPrompt) => "Kelas (misal RA, RB)",

        (Language::En, MessageKey::ScaffoldPrompt) => "Scaffold a new lab report starter project now? (y/n)",
        (Language::Id, MessageKey::ScaffoldPrompt) => "Buat starter project laporan praktikum sekarang? (y/n)",

        (Language::En, MessageKey::SuccessInit) => "Profile and workspace configuration saved successfully to profile.toml.",
        (Language::Id, MessageKey::SuccessInit) => "Profil dan konfigurasi workspace berhasil disimpan ke profile.toml.",

        (Language::En, MessageKey::NextSteps) => "Next steps:\n  • Compile report:  lab-report build\n  • Interactive TUI: lab-report tui\n  • Audit rules:     lab-report lint --fix",
        (Language::Id, MessageKey::NextSteps) => "Langkah selanjutnya:\n  • Kompilasi PDF:   lab-report build\n  • Menu interaktif: lab-report tui\n  • Cek aturan:      lab-report lint --fix",

        (Language::En, MessageKey::LintPassed) => ">> LINT PASSED: All typography & quality invariants conform to zero tolerance.",
        (Language::Id, MessageKey::LintPassed) => ">> LINT BERHASIL: Semua aturan tipografi & kualitas terpenuhi dengan toleransi nol.",

        (Language::En, MessageKey::LintFailed) => ">> LINT FAILED: Typography issues detected.",
        (Language::Id, MessageKey::LintFailed) => ">> LINT GAGAL: Ditemukan pelanggaran aturan tipografi.",

        (Language::En, MessageKey::CompilationSuccess) => ">> Compilation successful.",
        (Language::Id, MessageKey::CompilationSuccess) => ">> Kompilasi berhasil.",

        (Language::En, MessageKey::VerificationPassed) => ">> All declarative verification tests passed!",
        (Language::Id, MessageKey::VerificationPassed) => ">> Semua verifikasi deklaratif berhasil lulus!",

        (Language::En, MessageKey::VerificationFailed) => "ERROR: Declarative verification failed!",
        (Language::Id, MessageKey::VerificationFailed) => "ERROR: Verifikasi deklaratif gagal!",

        (Language::En, MessageKey::PackSuccess) => ">> Submission package created successfully.",
        (Language::Id, MessageKey::PackSuccess) => ">> Paket pengumpulan tugas berhasil dibuat.",
    }
}

/// Formats notice when course is auto-detected from current working directory.
#[must_use]
pub fn msg_auto_detected(lang: Language, name: &str) -> String {
    match lang {
        Language::En => format!("==> Auto-detected course from directory: [{name}]"),
        Language::Id => format!("==> Otomatis mendeteksi mata kuliah dari direktori: [{name}]"),
    }
}

/// Formats start notice for report compilation.
#[must_use]
pub fn msg_compiling(lang: Language, name: &str, module: &str, format: &str) -> String {
    match lang {
        Language::En => format!("==> Compiling report for [{name}] (Module {module}, Format: {format})..."),
        Language::Id => format!("==> Mengompilasi laporan untuk [{name}] (Modul {module}, Format: {format})..."),
    }
}

/// Formats start notice for file watching mode.
#[must_use]
pub fn msg_watching(lang: Language, name: &str, module: &str, format: &str) -> String {
    match lang {
        Language::En => format!("==> Watching report for [{name}] (Module {module}, Format: {format})..."),
        Language::Id => format!("==> Memantau perubahan berkas untuk [{name}] (Modul {module}, Format: {format})..."),
    }
}

/// Formats start notice for packaging submission.
#[must_use]
pub fn msg_packaging(lang: Language, name: &str) -> String {
    match lang {
        Language::En => format!("==> Packaging submission archive for [{name}]..."),
        Language::Id => format!("==> Menyiapkan paket arsip tugas untuk [{name}]..."),
    }
}

/// Formats start notice for declarative testing.
#[must_use]
pub fn msg_verifying(lang: Language, name: &str) -> String {
    match lang {
        Language::En => format!("==> Running declarative verification for [{name}]..."),
        Language::Id => format!("==> Menjalankan verifikasi deklaratif untuk [{name}]..."),
    }
}

/// Formats start notice for linter.
#[must_use]
pub fn msg_linting(lang: Language, name: &str, dry_run: bool) -> String {
    match (lang, dry_run) {
        (Language::En, true) => format!("==> Running linter in [DRY RUN] mode for [{name}]..."),
        (Language::En, false) => format!("==> Running linter for [{name}]..."),
        (Language::Id, true) => format!("==> Menjalankan linter dalam mode [SIMULASI] untuk [{name}]..."),
        (Language::Id, false) => format!("==> Menjalankan linter untuk [{name}]..."),
    }
}

/// Formats error when no course is supplied or detected.
#[must_use]
pub fn msg_course_missing(lang: Language) -> String {
    match lang {
        Language::En => "Error: Target course not specified.\n  Run inside a course folder or specify explicitly: database, computer-networks, pbo.".to_string(),
        Language::Id => "Error: Mata kuliah belum ditentukan.\n  Jalankan di dalam folder praktikum atau sebutkan eksplisit: database, computer-networks, pbo.".to_string(),
    }
}

