use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;

#[derive(Debug)]
pub struct MailSender {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl MailSender {
    pub fn new(
        host: String,
        username: String,
        password: String,
        from: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(host.as_str())?
            // Add credentials for authentication
            .credentials(Credentials::new(username, password))
            // Optionally configure expected authentication mechanism
            .authentication(vec![Mechanism::Plain])
            .build();
        Ok(MailSender {
            mailer: smtp_transport,
            from,
        })
    }
    /// Create a new MailSender instance by environment variables
    pub fn new_by_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mail_host = env::var("MAIL_HOST").expect("MAIL_HOST must be set");
        let mail_user = env::var("MAIL_USER").expect("MAIL_USER must be set");
        let mail_password = env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set");
        let mail_from = env::var("MAIL_FROM").expect("MAIL_FROM must be set");
        Ok(MailSender::new(
            mail_host,
            mail_user,
            mail_password,
            mail_from,
        )?)
    }
    pub async fn send_plain_mail(
        &mut self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = lettre::Message::builder()
            .from(self.from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        self.mailer.send(email).await?;
        Ok(())
    }

    pub async fn send_html_mail(
        &mut self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        self.mailer.send(email).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_send_mail() {
    dotenvy::dotenv().ok();

    let mut sender = MailSender::new_by_env().unwrap();
    sender
        .send_plain_mail(
            "gengyuntuo@163.com".to_string(),
            "Test Subject".to_string(),
            "Test Body <h1>Hello</h1>".to_string(),
        )
        .await
        .unwrap();
    sender
        .send_html_mail(
            "gengyuntuo@163.com".to_string(),
            "Test Subject".to_string(),
            "Test Body <h1>Hello</h1>".to_string(),
        )
        .await
        .unwrap();
    assert!(true);
}
