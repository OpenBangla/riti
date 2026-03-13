use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;

use ndarray::Array2;
use ort::session::{Session, builder::GraphOptimizationLevel};
use ort::value::Tensor;
use sentencepiece::SentencePieceProcessor;

use crate::config::Config;

pub(crate) const CONTEXT_LEN: usize = 8;

pub(crate) const NUM_SPECIAL_TOKENS: usize = 4;

pub(crate) struct Predictor {
    pub(crate) session: Session,
    pub(crate) spp: SentencePieceProcessor,
    /// id -> piece string (without the \u{2581} marker)
    pub(crate) vocab: Vec<String>,
}

impl Predictor {
    pub(crate) fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let model_dir = config.get_database_dir();
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(model_dir.join("model.onnx"))?;

        let spp = SentencePieceProcessor::open(model_dir.join("sp_bangla.model"))?;
        let vocab = load_vocab(include_str!("../data/sp_bangla.vocab"))?;

        Ok(Self {
            session,
            spp,
            vocab,
        })
    }

    pub(crate) fn encode(&self, text: &str) -> Vec<i64> {
        let pieces = self.spp.encode(text).unwrap_or_default();
        pieces.iter().map(|p| p.id as i64).collect()
    }

    pub(crate) fn prepare_context(&self, tokens: &[i64]) -> Array2<i64> {
        let mut ctx = vec![0i64; CONTEXT_LEN];
        if tokens.len() >= CONTEXT_LEN {
            let start = tokens.len() - CONTEXT_LEN;
            ctx.copy_from_slice(&tokens[start..]);
        } else {
            let offset = CONTEXT_LEN - tokens.len();
            ctx[offset..].copy_from_slice(tokens);
        }
        Array2::from_shape_vec((1, CONTEXT_LEN), ctx).unwrap()
    }

    pub(crate) fn run_model(&mut self, input: Array2<i64>) -> Vec<f32> {
        let tensor = Tensor::from_array(input).unwrap();
        let outputs = self
            .session
            .run(ort::inputs!["input_ids" => tensor])
            .unwrap();
        let logits = outputs["logits"].try_extract_array::<f32>().unwrap();
        logits.as_slice().unwrap().to_vec()
    }

    pub(crate) fn predict(&mut self, text: &str, prefix: &str, top_k: usize) -> Vec<(String, f32)> {
        let tokens = self.encode(text);
        if tokens.is_empty() {
            return vec![];
        }

        let input = self.prepare_context(&tokens);
        let logits = self.run_model(input);

        if prefix.is_empty() {
            self.top_k_from_logits(&logits, top_k)
        } else {
            self.top_k_with_prefix(&logits, prefix, top_k)
        }
    }

    /// Standard top-k: softmax over all logits, pick the best k (skipping special tokens).
    pub(crate) fn top_k_from_logits(&self, logits: &[f32], top_k: usize) -> Vec<(String, f32)> {
        let probs = softmax(logits);
        let mut indexed: Vec<(usize, f32)> = probs.into_iter().enumerate().collect();
        indexed.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        indexed
            .into_iter()
            .filter(|(idx, _)| *idx >= NUM_SPECIAL_TOKENS)
            .filter_map(|(idx, prob)| {
                let piece = self.vocab.get(idx)?;
                if piece.is_empty() {
                    return None;
                }
                Some((piece.clone(), prob))
            })
            .take(top_k)
            .collect()
    }

    /// Prefix-aware top-k: mask non-matching tokens before softmax.
    pub(crate) fn top_k_with_prefix(
        &self,
        logits: &[f32],
        prefix: &str,
        top_k: usize,
    ) -> Vec<(String, f32)> {
        let mut masked = vec![f32::NEG_INFINITY; logits.len()];
        for (idx, &logit) in logits.iter().enumerate() {
            if idx >= NUM_SPECIAL_TOKENS {
                if let Some(piece) = self.vocab.get(idx) {
                    if !piece.is_empty() && piece.starts_with(prefix) {
                        masked[idx] = logit;
                    }
                }
            }
        }

        let probs = softmax(&masked);
        let mut indexed: Vec<(usize, f32)> = probs.into_iter().enumerate().collect();
        indexed.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        indexed
            .into_iter()
            .filter(|(_, prob)| *prob > 0.0 && prob.is_finite())
            .filter_map(|(idx, prob)| {
                let piece = self.vocab.get(idx)?;
                Some((piece.clone(), prob))
            })
            .take(top_k)
            .collect()
    }
}

pub(crate) fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&x| (x - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    if sum == 0.0 {
        return vec![0.0; logits.len()];
    }
    exps.into_iter().map(|e| e / sum).collect()
}

/// Parse the SentencePiece vocab file (tsv: piece\tscore) into a Vec indexed by id.
/// Strips the \u{2581} word-boundary marker from pieces.
pub(crate) fn load_vocab(content: &str) -> Result<Vec<String>, io::Error> {
    let mut vocab = Vec::new();
    for line in content.lines() {
        let piece = line.split('\t').next().unwrap_or("");
        let clean = piece.replace('\u{2581}', "");
        vocab.push(clean);
    }
    Ok(vocab)
}
