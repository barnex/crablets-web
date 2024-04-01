/*
use crate::prelude::*;

static ASSETS_DIR: OnceLock<AssetsDir> = OnceLock::new();

/// Find the "assets" directory.
/// Must be called before `assets()`.
pub fn init_assets_dir(assets_base_name: &str) -> Result<()> {
	match ASSETS_DIR.set(AssetsDir::find(assets_base_name)?) {
		Ok(()) => Ok(()),
		Err(_prev) => bail!("BUG: init_assets_dir called more than once"),
	}
}

/// Absolute path to the "assets" directory.
/// `init_assets_dir` must be called prior.
pub fn asset_dir() -> &'static AssetsDir {
	ASSETS_DIR.get().expect("BUG: init_assets_dir has not been called")
}

// Path to the `assets/` directory.
// Typestate pattern ensures correct use.
#[derive(Clone)]
pub struct AssetsDir(PathBuf);

impl AssetsDir {
	/// Find the absolute path of the `assets_base_name` (default: "assets") directory.
	/// Search in the current working directory and the executable's directory.
	fn find(assets_base_name: &str) -> Result<Self> {
		if let Ok(dir) = std::env::current_dir() {
			log::info!("searching for assets in working directory: {}", dir.to_string_lossy());
			let abs = dir.join(assets_base_name);
			if abs.exists() {
				return Ok(Self(abs));
			}
		}

		let exe = std::env::current_exe()?;
		if let Some(dir) = exe.parent() {
			log::info!("searching for assets in executable directory: {}", dir.to_string_lossy());
			let abs = dir.join(assets_base_name);
			if abs.exists() {
				return Ok(Self(abs));
			}
		}

		Err(anyhow!("assets directory not found.\nBe sure to run this program form a directory that contains 'assets/'."))
	}

	/// Find absolute path to a texture file with `base` name. E.g.:
	///   "lava" => "/path/to/textures/lava.png"
	pub fn find_texture(&self, base: &str) -> Result<PathBuf> {
		Self::find_asset(&self.textures_dir(), base, &["png", "jpg", "jpeg"])
	}

	/// Find the absolute path of an asset file. E.g.:
	///   find_asset("/path/to/assets/textures", "lava", &["png", "jpg"])? =>  /path/to/assets/textures/lava.jpg
	fn find_asset(dir: &Path, base: &str, extensions: &[&str]) -> Result<PathBuf> {
		for ext in extensions {
			let file = dir.join(base.to_owned() + "." + ext); // note: do not use .with_extension, *replaces* extension.
			if file.exists() {
				return Ok(file);
			}
		}
		Err(anyhow!("asset not found: {:?} with extension {}", dir.join(base), extensions.join(", ")))
	}

	pub fn audio_dir(&self) -> PathBuf {
		self.0.join("audio")
	}

	pub fn music_dir(&self) -> PathBuf {
		self.0.join("music")
	}

	pub fn settings_file(&self, file: &str) -> Result<PathBuf> {
		Ok(self.0.parent().ok_or(anyhow!("assets parent directory not found"))?.join(file))
	}

	fn textures_dir(&self) -> PathBuf {
		self.0.join("textures")
	}

	fn dir(&self) -> &Path {
		&self.0
	}
}
*/