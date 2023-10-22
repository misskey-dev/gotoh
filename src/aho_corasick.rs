use aho_corasick;

pub struct AhoCorasick {
  inner: aho_corasick::AhoCorasick
}

#[napi(js_name = "AhoCorasick")]
pub struct JsAhoCorasick {
  aho_corasick: AhoCorasick
}

#[napi]
impl JsAhoCorasick {
  #[napi]
  pub fn is_match(&self, input: String) -> bool {
    self.aho_corasick.inner.is_match(&input)
  }

  #[napi(factory)]
  pub fn with_patterns(patterns: Vec<String>) -> Self {
    let inner = aho_corasick::AhoCorasick::new(patterns).unwrap();

    Self {
      aho_corasick: AhoCorasick { inner }
    }
  }
}
