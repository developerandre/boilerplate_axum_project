use app::app_axum;
use tokio::net::TcpListener;
mod app;

#[tokio::main]
async fn main() {
    let port = 3000_u16;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await;

    match listener {
        Ok(l) => {
            println!("Le serveur a été démarré sur le port {port}");
            axum::serve(l, app_axum()).await.unwrap();
        }
        Err(e) => {
            println!(
                "Le serveur n'a pas pu démarré sur le port {port}. Raison {:?}",
                e
            );
        }
    }
}
