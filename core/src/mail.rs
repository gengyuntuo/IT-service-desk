use lettre::{
    transport::smtp::Error,
    Message, SmtpTransport, Transport,
    message::{header, MultiPart, SinglePart},
};
use tokio::time::{timeout, Duration};

pub trait Mail {
    async fn send(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    
    async fn send_html(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SmtpMailer {
    host: String,
    port: u16,
    username: String,
    password: String,
    tls: bool,
    auth: bool,
    timeout: u64,
}

impl SmtpMailer {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        tls: bool,
        auth: bool,
        timeout: u64,
    ) -> Self {
        SmtpMailer {
            host,
            port,
            username,
            password,
            tls,
            auth,
            timeout,
        }
    }
    
    pub fn builder() -> SmtpMailerBuilder {
        SmtpMailerBuilder::new()
    }
    
    fn create_transport(&self) -> Result<SmtpTransport, Error> {
        let mut transport_builder = SmtpTransport::relay(&self.host)?
            .port(self.port);
        
        if self.auth {
            transport_builder = transport_builder.credentials((
                self.username.clone(),
                self.password.clone(),
            ).into());
        }
        
        if self.tls {
            transport_builder = transport_builder_TLS();
        }
        
        Ok(transport_builder.build())
    }
    
    fn build_message(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        body: &str,
    ) -> Result<Message, Error> {
        Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| e.into())
    }
    
    fn build_html_message(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<Message, Error> {
        let text_content = text_body.unwrap_or("Please view this email in HTML format");
        
        let multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(text_content.to_string()),
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html_body.to_string()),
            );
        
        Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .multipart(multipart)
            .map_err(|e| e.into())
    }
}

impl Mail for SmtpMailer {
    async fn send(&self, to: &str, from: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = self.build_message(to, from, subject, body)?;
        let transport = self.create_transport()?;
        
        // Apply timeout if configured
        if self.timeout > 0 {
            let result = timeout(
                Duration::from_secs(self.timeout),
                async { transport.send(&message) }
            ).await;
            
            match result {
                Ok(send_result) => {
                    send_result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                }
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Email sending timed out"
                    )));
                }
            }
        } else {
            transport.send(&message)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }
        
        Ok(())
    }
    
    async fn send_html(
        &self,
        to: &str,
        from: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message = self.build_html_message(to, from, subject, html_body, text_body)?;
        let transport = self.create_transport()?;
        
        // Apply timeout if configured
        if self.timeout > 0 {
            let result = timeout(
                Duration::from_secs(self.timeout),
                async { transport.send(&message) }
            ).await;
            
            match result {
                Ok(send_result) => {
                    send_result.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                }
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Email sending timed out"
                    )));
                }
            }
        } else {
            transport.send(&message)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }
        
        Ok(())
    }
}

pub struct SmtpMailerBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    tls: bool,
    auth: bool,
    timeout: u64,
}

impl SmtpMailerBuilder {
    pub fn new() -> Self {
        SmtpMailerBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
            tls: false,
            auth: false,
            timeout: 30, // Default 30 seconds timeout
        }
    }
    
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn credentials(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self.auth = true;
        self
    }
    
    pub fn tls(mut self, enable: bool) -> Self {
        self.tls = enable;
        self
    }
    
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }
    
    pub fn build(self) -> Result<SmtpMailer, Box<dyn std::error::Error>> {
        let host = self.host.ok_or("SMTP host is required")?;
        let port = self.port.unwrap_or(587); // Default SMTP port
        
        let (username, password) = if self.auth {
            (
                self.username.ok_or("Username is required when auth is enabled")?,
                self.password.ok_or("Password is required when auth is enabled")?,
            )
        } else {
            (String::new(), String::new())
        };
        
        Ok(SmtpMailer::new(
            host,
            port,
            username,
            password,
            self.tls,
            self.auth,
            self.timeout,
        ))
    }
}

// Utility functions
pub fn create_gmail_mailer(username: String, password: String, timeout: u64) -> SmtpMailer {
    SmtpMailer::new(
        "smtp.gmail.com".to_string(),
        587,
        username,
        password,
        true,
        true,
        timeout,
    )
}

pub fn create_outlook_mailer(username: String, password: String, timeout: u64) -> SmtpMailer {
    SmtpMailer::new(
        "smtp-mail.outlook.com".to_string(),
        587,
        username,
        password,
        true,
        true,
        timeout,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_creation() {
        let mailer = SmtpMailer::builder()
            .host("smtp.example.com".to_string())
            .port(587)
            .credentials("user@example.com".to_string(), "password".to_string())
            .tls(true)
            .timeout(30)
            .build()
            .unwrap();
            
        assert_eq!(mailer.host, "smtp.example.com");
        assert_eq!(mailer.port, 587);
        assert_eq!(mailer.username, "user@example.com");
        assert_eq!(mailer.password, "password");
        assert!(mailer.tls);
        assert!(mailer.auth);
        assert_eq!(mailer.timeout, 30);
    }
    
    #[test]
    fn test_gmail_factory() {
        let mailer = create_gmail_mailer("test@gmail.com".to_string(), "password".to_string(), 60);
        assert_eq!(mailer.host, "smtp.gmail.com");
        assert_eq!(mailer.port, 587);
        assert!(mailer.tls);
        assert!(mailer.auth);
        assert_eq!(mailer.timeout, 60);
    }
}
