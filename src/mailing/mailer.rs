use std::{env, sync::Arc};

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use log::info;

pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    mailing_domain: Arc<str>,
}

impl Mailer {
    pub fn new(
        host: String,
        username: String,
        password: String,
        mailing_domain: Arc<str>,
    ) -> anyhow::Result<Self> {
        let credentials = Credentials::new(username, password);
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)?
            .credentials(credentials)
            .build();

        info!("Mailer initialized");

        Ok(Self {
            mailer,
            mailing_domain,
        })
    }

    pub fn new_from_env() -> anyhow::Result<Self> {
        let host = env::var("SMTP_HOST")?;
        let username = env::var("SMTP_USERNAME")?;
        let password = env::var("SMTP_PASSWORD")?;
        let mailing_domain = env::var("SMTP_MAILING_DOMAIN")?;

        Self::new(host, username, password, mailing_domain.into())
    }
}

impl Mailer {
    pub async fn send(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        body: String,
        content_type: Option<ContentType>,
    ) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(format!("{from} <{}>", self.mailing_domain).parse()?)
            .to(to.parse()?)
            .header(content_type.unwrap_or(ContentType::TEXT_PLAIN))
            .subject(subject)
            .body(body)?;

        self.mailer.send(email).await?;

        Ok(())
    }
}
