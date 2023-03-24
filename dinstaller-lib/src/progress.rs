use crate::proxies::Progress1Proxy;
use futures::stream::StreamExt;
use futures::stream::{select_all, SelectAll};
use futures_util::future::try_join3;
use std::error::Error;
use zbus::{Connection, PropertyStream};

#[derive(Default, Debug)]
pub struct Progress {
    pub current_step: u32,
    pub max_steps: u32,
    pub current_title: String,
    pub finished: bool,
    pub object_path: String,
}

impl Progress {
    pub async fn from_proxy(proxy: &crate::proxies::Progress1Proxy<'_>) -> zbus::Result<Progress> {
        let ((current_step, current_title), max_steps, finished) =
            try_join3(proxy.current_step(), proxy.total_steps(), proxy.finished()).await?;

        Ok(Self {
            current_step,
            current_title,
            max_steps,
            finished,
            object_path: proxy.path().to_string(),
        })
    }
}

pub async fn build_progress_monitor(
    connection: Connection,
) -> Result<ProgressMonitor<'static>, Box<dyn Error>> {
    let builder = ProgressMonitorBuilder::new(connection)
        .add_proxy(
            "org.opensuse.DInstaller",
            "/org/opensuse/DInstaller/Manager1",
        )
        .add_proxy(
            "org.opensuse.DInstaller.Software",
            "/org/opensuse/DInstaller/Software1",
        )
        .add_proxy(
            "org.opensuse.DInstaller.Storage",
            "/org/opensuse/DInstaller/Storage1",
        );
    builder.build().await
}

pub struct ProgressMonitorBuilder {
    proxies: Vec<(String, String)>,
    connection: Connection,
}

impl<'a> ProgressMonitorBuilder {
    pub fn new(connection: Connection) -> Self {
        Self {
            proxies: vec![],
            connection,
        }
    }

    pub fn add_proxy(mut self, destination: &str, path: &str) -> Self {
        self.proxies.push((destination.to_owned(), path.to_owned()));
        self
    }

    pub async fn build(self) -> Result<ProgressMonitor<'a>, Box<dyn Error>> {
        let mut monitor = ProgressMonitor::default();

        for (destination, path) in self.proxies {
            let proxy = Progress1Proxy::builder(&self.connection)
                .path(path)?
                .destination(destination)?
                .build()
                .await?;
            monitor.add_proxy(proxy);
        }
        Ok(monitor)
    }
}

#[derive(Default)]
pub struct ProgressMonitor<'a> {
    pub proxies: Vec<Progress1Proxy<'a>>,
}

impl<'a> ProgressMonitor<'a> {
    pub fn add_proxy(&mut self, proxy: Progress1Proxy<'a>) {
        self.proxies.push(proxy);
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut changes = self.build_stream().await;

        while let Some(_change) = changes.next().await {
            // todo: move the presentation logic to a presenter struct
            for proxy in &self.proxies {
                let path = proxy.path().to_string();
                let (step, description) = proxy.current_step().await?;
                let total = proxy.total_steps().await?;
                if proxy.finished().await? {
                    eprintln!("{path} finished");
                } else {
                    eprintln!("{path} {step}/{total}: {description}");
                }
            }

            if self.is_finished().await {
                return Ok(());
            }
        }

        Ok(())
    }

    async fn is_finished(&self) -> bool {
        for proxy in &self.proxies {
            if !proxy.finished().await.unwrap_or(false) {
                return false;
            }
        }
        true
    }

    async fn build_stream(&self) -> SelectAll<PropertyStream<(u32, String)>> {
        let mut streams = vec![];
        for proxy in &self.proxies {
            let s = proxy.receive_current_step_changed().await;
            streams.push(s);
        }

        select_all(streams)
    }
}
