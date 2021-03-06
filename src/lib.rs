// Copyright 2018 The HuggingFace Inc. team.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


pub mod preprocessing;

pub use preprocessing::vocab::{base_vocab::BaseVocab, bert_vocab::BertVocab, openai_gpt_vocab::OpenAiGptVocab, gpt2_vocab::Gpt2Vocab, roberta_vocab::RobertaVocab};
pub use preprocessing::tokenizer::bert_tokenizer;
use pyo3::prelude::*;
use crate::preprocessing::tokenizer::bert_tokenizer::BertTokenizer;
use crate::preprocessing::tokenizer::base_tokenizer::{MultiThreadedTokenizer, TruncationStrategy, TokenizedInput, Tokenizer};
use pyo3::exceptions;
use crate::preprocessing::vocab::base_vocab::Vocab;
use crate::preprocessing::tokenizer::ctrl_tokenizer::CtrlTokenizer;
use crate::preprocessing::tokenizer::gpt2_tokenizer::Gpt2Tokenizer;
use crate::preprocessing::tokenizer::roberta_tokenizer::RobertaTokenizer;
use crate::preprocessing::tokenizer::openai_gpt_tokenizer::OpenAiGptTokenizer;

#[macro_use]
extern crate lazy_static;

trait PyTokenizer<T: Tokenizer<U>, U: Vocab> {
    fn tokenizer(&self) -> &T;

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        Ok(self.tokenizer().tokenize(&text))
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        Ok(self.tokenizer().tokenize_list(text_list))
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer().encode(&text, None, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer().encode(&text_a, Some(&text_b), max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer().encode_list(text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer().encode_pair_list(text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }
}

trait PyMultiThreadTokenizer<T: MultiThreadedTokenizer<U>, U: Vocab>
    where Self: PyTokenizer<T, U> {
    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        Ok(MultiThreadedTokenizer::tokenize_list(self.tokenizer(), text_list))
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(MultiThreadedTokenizer::encode_list(self.tokenizer(), text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(MultiThreadedTokenizer::encode_pair_list(self.tokenizer(), text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }
}

#[pyclass(module = "rust_transformers")]
struct PyBertTokenizer {
    tokenizer: BertTokenizer,
}

impl PyTokenizer<BertTokenizer, BertVocab> for PyBertTokenizer {
    fn tokenizer(&self) -> &BertTokenizer {
        &self.tokenizer
    }
}

impl PyMultiThreadTokenizer<BertTokenizer, BertVocab> for PyBertTokenizer {}

#[pymethods]
impl PyBertTokenizer {
    #[new]
    fn new(obj: &PyRawObject, path: String) {
        obj.init(PyBertTokenizer {
            tokenizer: BertTokenizer::from_file(&path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        <Self as PyTokenizer<BertTokenizer, BertVocab>>::tokenize(&self, text)
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        <Self as PyMultiThreadTokenizer<BertTokenizer, BertVocab>>::tokenize_list(&self, text_list)
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<BertTokenizer, BertVocab>>::encode(&self, text, max_len, truncation_strategy, stride)
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<BertTokenizer, BertVocab>>::encode_pair(&self, text_a, text_b, max_len, truncation_strategy, stride)
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyMultiThreadTokenizer<BertTokenizer, BertVocab>>::encode_list(&self, text_list, max_len, truncation_strategy, stride)
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyMultiThreadTokenizer<BertTokenizer, BertVocab>>::encode_pair_list(&self, text_list, max_len, truncation_strategy, stride)
    }
}

#[pyclass(module = "rust_transformers")]
struct PyCtrlTokenizer {
    tokenizer: CtrlTokenizer,
}

impl PyTokenizer<CtrlTokenizer, OpenAiGptVocab> for PyCtrlTokenizer {
    fn tokenizer(&self) -> &CtrlTokenizer {
        &self.tokenizer
    }
}

#[pymethods]
impl PyCtrlTokenizer {
    #[new]
    fn new(obj: &PyRawObject, vocab_path: String, merges_path: String) {
        obj.init(PyCtrlTokenizer {
            tokenizer: CtrlTokenizer::from_file(&vocab_path, &merges_path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::tokenize(&self, text)
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::tokenize_list(&self, text_list)
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::encode(&self, text, max_len, truncation_strategy, stride)
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::encode_pair(&self, text_a, text_b, max_len, truncation_strategy, stride)
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::encode_list(&self, text_list, max_len, truncation_strategy, stride)
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<CtrlTokenizer, OpenAiGptVocab>>::encode_pair_list(&self, text_list, max_len, truncation_strategy, stride)
    }
}


#[pyclass(module = "rust_transformers")]
struct PyGpt2Tokenizer {
    tokenizer: Gpt2Tokenizer,
}

impl PyTokenizer<Gpt2Tokenizer, Gpt2Vocab> for PyGpt2Tokenizer {
    fn tokenizer(&self) -> &Gpt2Tokenizer {
        &self.tokenizer
    }
}

#[pymethods]
impl PyGpt2Tokenizer {
    #[new]
    fn new(obj: &PyRawObject, vocab_path: String, merges_path: String) {
        obj.init(PyGpt2Tokenizer {
            tokenizer: Gpt2Tokenizer::from_file(&vocab_path, &merges_path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::tokenize(&self, text)
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::tokenize_list(&self, text_list)
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::encode(&self, text, max_len, truncation_strategy, stride)
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::encode_pair(&self, text_a, text_b, max_len, truncation_strategy, stride)
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::encode_list(&self, text_list, max_len, truncation_strategy, stride)
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<Gpt2Tokenizer, Gpt2Vocab>>::encode_pair_list(&self, text_list, max_len, truncation_strategy, stride)
    }
}

#[pyclass(module = "rust_transformers")]
struct PyRobertaTokenizer {
    tokenizer: RobertaTokenizer,
}

impl PyTokenizer<RobertaTokenizer, RobertaVocab> for PyRobertaTokenizer {
    fn tokenizer(&self) -> &RobertaTokenizer {
        &self.tokenizer
    }
}

#[pymethods]
impl PyRobertaTokenizer {
    #[new]
    fn new(obj: &PyRawObject, vocab_path: String, merges_path: String) {
        obj.init(PyRobertaTokenizer {
            tokenizer: RobertaTokenizer::from_file(&vocab_path, &merges_path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::tokenize(&self, text)
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::tokenize_list(&self, text_list)
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::encode(&self, text, max_len, truncation_strategy, stride)
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::encode_pair(&self, text_a, text_b, max_len, truncation_strategy, stride)
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::encode_list(&self, text_list, max_len, truncation_strategy, stride)
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<RobertaTokenizer, RobertaVocab>>::encode_pair_list(&self, text_list, max_len, truncation_strategy, stride)
    }
}

#[pyclass(module = "rust_transformers")]
struct PyOpenAiGptTokenizer {
    tokenizer: OpenAiGptTokenizer,
}

impl PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab> for PyOpenAiGptTokenizer {
    fn tokenizer(&self) -> &OpenAiGptTokenizer {
        &self.tokenizer
    }
}

#[pymethods]
impl PyOpenAiGptTokenizer {
    #[new]
    fn new(obj: &PyRawObject, vocab_path: String, merges_path: String) {
        obj.init(PyOpenAiGptTokenizer {
            tokenizer: OpenAiGptTokenizer::from_file(&vocab_path, &merges_path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::tokenize(&self, text)
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::tokenize_list(&self, text_list)
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::encode(&self, text, max_len, truncation_strategy, stride)
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::encode_pair(&self, text_a, text_b, max_len, truncation_strategy, stride)
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::encode_list(&self, text_list, max_len, truncation_strategy, stride)
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        <Self as PyTokenizer<OpenAiGptTokenizer, OpenAiGptVocab>>::encode_pair_list(&self, text_list, max_len, truncation_strategy, stride)
    }
}

#[pymodule]
fn rust_transformers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBertTokenizer>()?;
    m.add_class::<PyCtrlTokenizer>()?;
    m.add_class::<PyGpt2Tokenizer>()?;
    m.add_class::<PyRobertaTokenizer>()?;
    m.add_class::<PyOpenAiGptTokenizer>()?;
    Ok(())
}