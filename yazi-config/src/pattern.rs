use std::path::Path;

use glob::MatchOptions;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Pattern {
	inner:     glob::Pattern,
	sensitive: bool,
	is_folder: bool,
	full_path: bool,
}

impl Pattern {
	#[inline]
	pub fn matches(&self, str: impl AsRef<str>) -> bool {
		self.inner.matches_with(str.as_ref(), MatchOptions {
			case_sensitive:              self.sensitive,
			require_literal_separator:   self.full_path,
			require_literal_leading_dot: false,
		})
	}

	#[inline]
	pub fn match_path(&self, path: impl AsRef<Path>, is_folder: bool) -> bool {
		if is_folder != self.is_folder {
			return false;
		}

		let path = path.as_ref();
		self.matches(if self.full_path {
			path.to_string_lossy()
		} else {
			path.file_name().map_or_else(|| path.to_string_lossy(), |n| n.to_string_lossy())
		})
	}

	#[inline]
	pub fn is_wildcard(&self) -> bool { self.inner.as_str() == "*" || self.inner.as_str() == "*/" }
}

impl TryFrom<&str> for Pattern {
	type Error = anyhow::Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let a = s.trim_start_matches("\\s");
		let b = a.trim_end_matches('/');
		Ok(Self {
			inner:     glob::Pattern::new(b)?,
			sensitive: a.len() < s.len(),
			is_folder: b.len() < a.len(),
			full_path: b.contains('/'),
		})
	}
}

impl TryFrom<String> for Pattern {
	type Error = anyhow::Error;

	fn try_from(s: String) -> Result<Self, Self::Error> { Self::try_from(s.as_str()) }
}
