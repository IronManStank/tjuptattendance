use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::anyhow;
use reqwest::{Client, ClientBuilder};
use reqwest_cookie_store::CookieStoreMutex;

use crate::{Error, User};

#[derive(Debug)]
pub struct AttBot {
    pub(crate) user: User,
    pub(crate) client: Client,
    pub(crate) cookie: Arc<CookieStoreMutex>,
    /// 如果设置了路径则进行保存，如果是None则不保存
    pub(crate) cookie_path: Option<PathBuf>,
}

impl AttBot {
    pub fn new(user: User, cookie_path: Option<PathBuf>) -> Self {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                    AppleWebKit/537.36 (KHTML, like Gecko) \
                    Chrome/100.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        let cookie = Arc::new(CookieStoreMutex::default());
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .cookie_store(true)
            .cookie_provider(cookie.clone())
            .default_headers(header)
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .unwrap();

        Self {
            client,
            cookie,
            user,
            cookie_path,
        }
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn clinet(&self) -> &Client {
        &self.client
    }

    pub fn clear_cookie(&self) -> Result<(), Error> {
        let mut lock = self.cookie.lock().map_err(|e| anyhow!("内部错误 {e}"))?;
        lock.clear();
        Ok(())
    }

    pub fn cookie_path(&self) -> &Option<PathBuf> {
        &self.cookie_path
    }

    pub(crate) fn save_cookie_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let mut f = File::create(path)?;
        let lock = self.cookie.lock().map_err(|e| anyhow!("内部错误 {e}"))?;
        lock.save_json(&mut f)
            .map_err(|e| anyhow!("无法保存cookie {} {e}", path.display()))?;
        Ok(())
    }

    pub fn save_cookie(&self) -> Result<(), Error> {
        if let Some(ref path) = self.cookie_path {
            self.save_cookie_to(path)?;
        }
        Ok(())
    }

    pub(crate) fn load_cookie_from<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let f = File::open(path).map(std::io::BufReader::new)?;
        let cookie = reqwest_cookie_store::CookieStore::load_json(f).map_err(|e| anyhow!("{e}"))?;
        let mut lock = self.cookie.lock().map_err(|e| anyhow!("{}", e))?;
        *lock = cookie;
        Ok(())
    }

    pub fn load_cookie(&self) -> Result<(), Error> {
        if let Some(ref path) = self.cookie_path {
            self.load_cookie_from(path)?;
        }
        Ok(())
    }
}

impl Drop for AttBot {
    fn drop(&mut self) {
        if let Err(e) = self.save_cookie() {
            eprintln!("{} 保存 cookie 失败 Err: {}", self.user.name(), e);
        }
    }
}
