mod models;
mod game_manager;
mod api;
mod auth;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use game_manager::GameManager;

/// Route de santé
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "poker-server"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🎮 Démarrage du serveur de poker...");
    
    // Créer le gestionnaire de parties (partagé entre tous les workers)
    let game_manager = web::Data::new(GameManager::new());

    // Lancer le thread de maintenance (timeouts)
    let manager_clone = game_manager.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            manager_clone.run_maintenance();
        }
    });

    println!("🌐 Serveur disponible sur http://localhost:8080");
    println!("📡 API disponible sur http://localhost:8080/api");
    println!("🎯 Interface web sur http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(game_manager.clone())
            // Route de santé
            .route("/health", web::get().to(health))
            // API REST
            .configure(api::configure)
            // Fichiers statiques (UI) - Chemin relatif à la racine du workspace
            .service(fs::Files::new("/", "./poker-ui").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
