use leptos::prelude::*;

#[cfg(feature = "ssr")]
use lettre::message::header::ContentType;
#[cfg(feature = "ssr")]
use lettre::transport::smtp::authentication::Credentials;
#[cfg(feature = "ssr")]
use lettre::{Message, AsyncSmtpTransport, Tokio1Executor, AsyncTransport};
#[cfg(feature = "ssr")]
use std::env;

#[server(SendContactEmail, "/api")]
pub async fn send_contact_email(name: String, email: String, message: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let smtp_username = env::var("SMTP_USERNAME").map_err(|_| ServerFnError::new("SMTP_USERNAME not set"))?;
        let smtp_password = env::var("SMTP_PASSWORD").map_err(|_| ServerFnError::new("SMTP_PASSWORD not set"))?;
        let smtp_host = env::var("SMTP_HOST").map_err(|_| ServerFnError::new("SMTP_HOST not set"))?;
        let receiver = env::var("CONTACT_RECEIVER").unwrap_or_else(|_| smtp_username.clone());

        let email_content = format!(
            "New Contact Form Submission\n\nName: {}\nEmail: {}\n\nMessage:\n{}",
            name, email, message
        );

        let email_msg = Message::builder()
            .from(format!("Portfolio Contact <{}>", smtp_username).parse().map_err(|e| ServerFnError::new(format!("Invalid From Address: {}", e)))?)
            .reply_to(format!("{} <{}>", name, email).parse().map_err(|e| ServerFnError::new(format!("Invalid Reply-To: {}", e)))?)
            .to(receiver.parse().map_err(|e| ServerFnError::new(format!("Invalid Receiver: {}", e)))?)
            .subject(format!("New Message from {}", name))
            .header(ContentType::TEXT_PLAIN)
            .body(email_content)
            .map_err(|e| ServerFnError::new(format!("Failed to build email: {}", e)))?;

        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(|e| ServerFnError::new(format!("Invalid SMTP Host: {}", e)))?
            .credentials(creds)
            .build();

        mailer.send(email_msg).await.map_err(|e| ServerFnError::new(format!("Failed to send email: {}", e)))?;
        
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("This function is only executed on the server")
    }
}
