use zbus::export::futures_util::future::try_join3;

pub struct Progress {
    pub current_step: u32,
    pub max_steps: u32,
    pub current_title: String,
    pub finished: bool
}

impl Progress {
    pub async fn from_proxy(proxy: &crate::proxies::Progress1Proxy<'_>) -> zbus::Result<Self> {
        let ((current_step, current_title), max_steps, finished) = 
            try_join3(proxy.current_step(), proxy.total_steps(), proxy.finished()).await?;
        
        Ok(Self{ current_step, current_title, max_steps, finished})
    }
}