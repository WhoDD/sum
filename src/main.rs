use actix_web::{get, post, web, web::Bytes, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Role {
    Tank,
    Healer,
    Damager,
    Others,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Player {
    id: u32,
    role: Role,
    queue_time: u32,
}

#[get("/")]
async fn echo_request(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    
    let data = match query.get("data") {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("Parameter 'data' is missing"),
    };

    println!("Запрос: {}", data);
    
    HttpResponse::Ok().finish()
}

async fn handle_request(players: web::Json<Vec<Player>>) -> HttpResponse {

    let players_data = players.into_inner();

    println!("Received players: {:?}", players_data);

    HttpResponse::Ok().finish()
}
//

trait GroupCreator {
    fn create_groups(&mut self) -> Vec<Vec<Player>>;
}

impl GroupCreator for Vec<Player> {
    fn create_groups(&mut self) -> Vec<Vec<Player>> {
        let mut groups: Vec<Vec<Player>> = Vec::new();
        let mut tanks: Vec<Player> = Vec::new();
        let mut healers: Vec<Player> = Vec::new();
        let mut damagers: Vec<Player> = Vec::new();
        let mut others: Vec<Player> = Vec::new();
    
        self.sort_by_key(|player| player.queue_time);
    
        for player in self.iter() {
            match player.role {
                Role::Tank => tanks.push(player.clone()),
                Role::Healer => healers.push(player.clone()),
                Role::Damager => damagers.push(player.clone()),
                Role::Others => others.push(player.clone()),
            }
        }
    
        let mut current_group: Vec<Player> = Vec::new();
    
        while !tanks.is_empty() || !healers.is_empty() || !damagers.is_empty() || !others.is_empty() {
            if current_group.len() == 4 || (!current_group.is_empty() && current_group.last().unwrap().queue_time > 5) {
                groups.push(current_group.clone());
                current_group.clear();
            }
    
            if let Some(player) = tanks.pop() {
                current_group.push(player);
            }
            if let Some(player) = healers.pop() {
                current_group.push(player);
            }
            if let Some(player) = damagers.pop() {
                current_group.push(player);
            }
            if let Some(player) = others.pop() {
                current_group.push(player);
            }
        }
    
        if !current_group.is_empty() {
            groups.push(current_group);
        }
    
        groups
    }
    
}

//
async fn handle_post(body: Bytes) -> HttpResponse {
    let mut players: Vec<Player> = match serde_json::from_slice(&body) {
        Ok(players) => players,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    let groups = players.create_groups();

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}: {:?}", i + 1, group);
    }

    println!("Запрос: {:?}", players);

    HttpResponse::Ok().body("Ok!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/mm", web::post().to(handle_post))
            .service(echo_request)
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_groups() {
        let mut players = vec![
            Player { id: 1, role: Role::Tank, queue_time: 3 },
            Player { id: 2, role: Role::Damager, queue_time: 8 },
            Player { id: 3, role: Role::Healer, queue_time: 2 },
            Player { id: 4, role: Role::Others, queue_time: 7 },
            Player { id: 5, role: Role::Damager, queue_time: 1 },
            Player { id: 6, role: Role::Tank, queue_time: 10 },
            Player { id: 7, role: Role::Healer, queue_time: 6 },
        ];

        let groups = players.create_groups();

        for group in &groups {
            let mut has_tanks = false;
            let mut has_healers = false;
            let mut has_damagers = false;

            for player in group {
                match player.role {
                    Role::Tank => has_tanks = true,
                    Role::Healer => has_healers = true,
                    Role::Damager => has_damagers = true,
                    _ => (),
                }
            }

            assert!(has_tanks && has_healers && has_damagers);
        }
    }
}
