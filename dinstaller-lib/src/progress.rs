pub struct Progress {
    pub current_step: u32,
    pub max_steps: u32,
    pub current_title: String,
    pub finished: bool
}

impl Progress {
    pub fn from_proxy(proxy: &crate::proxies::Progress1Proxy) -> zbus::Result<Self> {
        let (current_step, current_title) = proxy.current_step()?;
        Ok(Self{ current_step, current_title,
             max_steps: proxy.total_steps()?, finished: proxy.finished()?})
    }
}