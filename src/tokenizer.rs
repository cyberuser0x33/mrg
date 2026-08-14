use anyhow::{Result, anyhow, bail};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::fs;
use std::path::PathBuf;
use tokenizers::tokenizer::Tokenizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum ModelKind {
    Gpt4oO1O3Mini,
    Gpt4TurboGpt35Turbo,
    GeminiGemma7b,
    Claude35SonnetOpus,
    Llama32,
    DeepSeekV2V3R1,
    DeepSeekV4FlashPro,
    DeepSeek32Exp,
    MistralCodestral,
    Phi3Phi4,
    CohereCommandRPlus,
    Glm52_51_50_47Flash,
    Qwen36_37_37Max_37Plus,
    MiniMaxM3,
    Gemma4_31b,
    KimiK3,
}

pub struct ModelInfo {
    pub kind: ModelKind,
    pub display_name: &'static str,
    pub remote_url: &'static str,
    pub filename: &'static str,
}

pub const SUPPORTED_MODELS: &[ModelInfo] = &[
    ModelInfo {
        kind: ModelKind::Gpt4oO1O3Mini,
        display_name: "GPT4-O1-O3-Mini",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/gpt4o_o1_o3mini.json",
        filename: "gpt4o_o1_o3mini.json",
    },
    ModelInfo {
        kind: ModelKind::Gpt4TurboGpt35Turbo,
        display_name: "GPT4-Turbo-GPT3.5-Turbo",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/gpt4_turbo_gpt3.5_turbo.json",
        filename: "gpt4_turbo_gpt3.5_turbo.json",
    },
    ModelInfo {
        kind: ModelKind::GeminiGemma7b,
        display_name: "Gemini-Gemma7B",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/gemini_gemma7b.json",
        filename: "gemini_gemma7b.json",
    },
    ModelInfo {
        kind: ModelKind::Claude35SonnetOpus,
        display_name: "Claude3.5-Sonnet-Opus",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/claude3_3.5_sonnet_opus.json",
        filename: "claude3_3.5_sonnet_opus.json",
    },
    ModelInfo {
        kind: ModelKind::Llama32,
        display_name: "LLAMA3-3.1-3.2",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/llama3.2.json",
        filename: "llama3.2.json",
    },
    ModelInfo {
        kind: ModelKind::DeepSeekV2V3R1,
        display_name: "DeepSeekV2-V3-R1",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/deepseek_v2_v3_r1.json",
        filename: "deepseek_v2_v3_r1.json",
    },
    ModelInfo {
        kind: ModelKind::DeepSeekV4FlashPro,
        display_name: "DeepSeek-V4-Flash-Pro",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/deepseek_v4-flash_pro.json",
        filename: "deepseek_v4-flash_pro.json",
    },
    ModelInfo {
        kind: ModelKind::DeepSeek32Exp,
        display_name: "DeepSeek-3.2-3.2Exp",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/deepseek-3.2_3.2exp.json",
        filename: "deepseek-3.2_3.2exp.json",
    },
    ModelInfo {
        kind: ModelKind::MistralCodestral,
        display_name: "Mistral-Codestral",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/mistral_codestral.json",
        filename: "mistral_codestral.json",
    },
    ModelInfo {
        kind: ModelKind::Phi3Phi4,
        display_name: "Phi3-Phi4",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/phi3_phi4.json",
        filename: "phi3_phi4.json",
    },
    ModelInfo {
        kind: ModelKind::CohereCommandRPlus,
        display_name: "Cohere-CommandR-R+",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/cohere_command_r_r%2B.json",
        filename: "cohere_command_r_r_plus.json",
    },
    ModelInfo {
        kind: ModelKind::Glm52_51_50_47Flash,
        display_name: "GLM-5.2-5.1-5.0-4.7Flash",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/glm-5.2_5.1_5.0_4.7flash.json",
        filename: "glm-5.2_5.1_5.0_4.7flash.json",
    },
    ModelInfo {
        kind: ModelKind::Qwen36_37_37Max_37Plus,
        display_name: "Qwen3.6-3.7-3.7Max-3.7Plus",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/qwen3.6_3.7_3.7max_3.7plus.json",
        filename: "qwen3.6_3.7_3.7max_3.7plus.json",
    },
    ModelInfo {
        kind: ModelKind::MiniMaxM3,
        display_name: "MiniMax-M3",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/minimax-m3.json",
        filename: "minimax-m3.json",
    },
    ModelInfo {
        kind: ModelKind::Gemma4_31b,
        display_name: "Gemma-4-31B",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/gemma-4-31b.json",
        filename: "gemma-4-31b.json",
    },
    ModelInfo {
        kind: ModelKind::KimiK3,
        display_name: "Kimi-K3",
        remote_url: "https://huggingface.co/datasets/cyberuser0x33/model-tokenizers/resolve/main/tokenizers/kimi-k3.json",
        filename: "kimi-k3.json",
    },
];

#[allow(dead_code)]
pub struct AnalysisResult {
    pub words: usize,
    pub chars: usize,
    pub gpt_string: String,
    pub gemini_string: String,
    pub claude_string: String,
}

pub struct AICounter {
    tokenizers: std::collections::HashMap<ModelKind, Tokenizer>,
}

impl AICounter {
    pub fn new(folder_name: &str, load_all: bool) -> Result<Self> {
        let base_dir =
            dirs::data_local_dir().ok_or_else(|| anyhow!("Local data directory not found"))?;
        let tokenizers_dir = base_dir.join("mrgfile").join(folder_name);

        if !tokenizers_dir.exists() {
            fs::create_dir_all(&tokenizers_dir)?;
        }

        // Determine which models need to be downloaded
        let mut to_download = Vec::new();
        for model in SUPPORTED_MODELS {
            let path = tokenizers_dir.join(model.filename);
            if !path.exists() {
                to_download.push((model, path));
            }
        }

        // First, download any files that do not exist (always download all 10)
        if !to_download.is_empty() {
            let pb = ProgressBar::new(to_download.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{bar:40.cyan/white}] Downloading tokenizers {pos}/{len}... {msg}",
                    )
                    .unwrap()
                    .progress_chars("▰▰▱"),
            );
            for (model, path) in to_download {
                pb.set_message(model.display_name);
                Self::download_tokenizer(model.display_name, model.remote_url, &path, &pb)?;
                pb.inc(1);
            }
            pb.finish_with_message("All tokenizers downloaded!");
        }

        let mut tokenizers = std::collections::HashMap::new();
        // Load the ones we need
        for model in SUPPORTED_MODELS {
            let path = tokenizers_dir.join(model.filename);
            let should_load = if load_all {
                true
            } else {
                model.kind == ModelKind::Gpt4oO1O3Mini
                    || model.kind == ModelKind::GeminiGemma7b
                    || model.kind == ModelKind::Claude35SonnetOpus
            };

            if should_load {
                let t = Tokenizer::from_file(&path).map_err(|e| {
                    let _ = fs::remove_file(&path);
                    anyhow!(
                        "Dictionary error {}: {}. File was deleted, please try again.",
                        model.display_name,
                        e
                    )
                })?;
                tokenizers.insert(model.kind, t);
            }
        }

        Ok(Self { tokenizers })
    }

    /// Creates an AICounter that downloads all missing tokenizer files,
    /// but only loads a single specific model into memory.
    pub fn new_single(folder_name: &str, target_kind: ModelKind) -> Result<Self> {
        let base_dir =
            dirs::data_local_dir().ok_or_else(|| anyhow!("Local data directory not found"))?;
        let tokenizers_dir = base_dir.join("mrgfile").join(folder_name);

        if !tokenizers_dir.exists() {
            fs::create_dir_all(&tokenizers_dir)?;
        }

        // Determine which models need to be downloaded
        let mut to_download = Vec::new();
        for model in SUPPORTED_MODELS {
            let path = tokenizers_dir.join(model.filename);
            if !path.exists() {
                to_download.push((model, path));
            }
        }

        if !to_download.is_empty() {
            let pb = ProgressBar::new(to_download.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{bar:40.cyan/white}] Downloading tokenizers {pos}/{len}... {msg}",
                    )
                    .unwrap()
                    .progress_chars("▰▰▱"),
            );
            for (model, path) in to_download {
                pb.set_message(model.display_name);
                Self::download_tokenizer(model.display_name, model.remote_url, &path, &pb)?;
                pb.inc(1);
            }
            pb.finish_with_message("All tokenizers downloaded!");
        }

        let mut tokenizers = std::collections::HashMap::new();
        // Load only the target model
        for model in SUPPORTED_MODELS {
            if model.kind == target_kind {
                let path = tokenizers_dir.join(model.filename);
                let t = Tokenizer::from_file(&path).map_err(|e| {
                    let _ = fs::remove_file(&path);
                    anyhow!(
                        "Dictionary error {}: {}. File was deleted, please try again.",
                        model.display_name,
                        e
                    )
                })?;
                tokenizers.insert(model.kind, t);
                break;
            }
        }

        Ok(Self { tokenizers })
    }

    fn download_tokenizer(name: &str, url: &str, path: &PathBuf, pb: &ProgressBar) -> Result<()> {
        pb.set_message(format!("Downloading {}...", name));

        let client = Client::builder().build()?;
        let response = client
            .get(url)
            .header(USER_AGENT, "rust-tokenizer-app/1.0")
            .send()?;

        let status = response.status();

        if !status.is_success() {
            let err_text = response.text().unwrap_or_else(|_| "Unknown error".into());
            bail!("Server error {}: {}. Response: {}", name, status, err_text);
        }

        let bytes = response.bytes()?;
        fs::write(path, bytes)?;
        pb.println(format!("[+] Successfully downloaded {}", name));
        Ok(())
    }

    fn count_tokens_chunked(tokenizer: &Tokenizer, text: &str) -> usize {
        const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB
        let mut total = 0;
        let mut start = 0;
        let bytes = text.as_bytes();
        while start < bytes.len() {
            let mut end = start + CHUNK_SIZE;
            if end >= bytes.len() {
                end = bytes.len();
            } else {
                while end > start && !text.is_char_boundary(end) {
                    end -= 1;
                }
                if end == start {
                    end = start + CHUNK_SIZE;
                    while end < bytes.len() && !text.is_char_boundary(end) {
                        end += 1;
                    }
                }
            }
            let chunk = &text[start..end];
            if let Ok(encoding) = tokenizer.encode(chunk, true) {
                total += encoding.get_ids().len();
            }
            start = end;
        }
        total
    }

    pub fn count_tokens_raw(&self, text: &str) -> (usize, usize, usize) {
        let gpt = self
            .tokenizers
            .get(&ModelKind::Gpt4oO1O3Mini)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);
        let gemini = self
            .tokenizers
            .get(&ModelKind::GeminiGemma7b)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);
        let claude = self
            .tokenizers
            .get(&ModelKind::Claude35SonnetOpus)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);
        (gpt, gemini, claude)
    }

    pub fn count_tokens_all(&self, text: &str) -> std::collections::HashMap<ModelKind, usize> {
        let mut counts = std::collections::HashMap::new();
        for (kind, tokenizer) in &self.tokenizers {
            counts.insert(*kind, Self::count_tokens_chunked(tokenizer, text));
        }
        counts
    }

    pub fn count_tokens_for(&self, kind: ModelKind, text: &str) -> usize {
        self.tokenizers
            .get(&kind)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn count_string(&self, text: &str) -> AnalysisResult {
        let words = text.split_whitespace().count();
        let chars = text.chars().count();

        let gpt_count = self
            .tokenizers
            .get(&ModelKind::Gpt4oO1O3Mini)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);
        let gemini_count = self
            .tokenizers
            .get(&ModelKind::GeminiGemma7b)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);
        let claude_count = self
            .tokenizers
            .get(&ModelKind::Claude35SonnetOpus)
            .map(|t| Self::count_tokens_chunked(t, text))
            .unwrap_or(0);

        AnalysisResult {
            words,
            chars,
            gpt_string: format!("GPT4-O1-O3-Mini: ~{}", gpt_count),
            gemini_string: format!("Gemini-Gemma7B: ~{}", gemini_count),
            claude_string: format!("Claude3.5-Sonnet-Opus: ~{}", claude_count),
        }
    }
}
