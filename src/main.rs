use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Role {
    Tank,
    Healer,
    Damager,
    Others,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
    id: u32,
    role: Role,
    queue_time: u32,
}

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

fn main() {
    let mut players = vec![
        Player { id: 1, role: Role::Tank, queue_time: 3 },
            Player { id: 2, role: Role::Damager, queue_time: 8 },
            Player { id: 3, role: Role::Healer, queue_time: 2 },
            Player { id: 4, role: Role::Others, queue_time: 7 },
            Player { id: 5, role: Role::Damager, queue_time: 1 },
            Player { id: 6, role: Role::Tank, queue_time: 10 },
            Player { id: 7, role: Role::Healer, queue_time: 6 },
            Player { id: 8, role: Role::Others, queue_time: 3 },
    ];

    let groups = players.create_groups();

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}: {:?}", i + 1, group);
    }

    delay();
}

fn delay() {
    let mut close = String::new();
    println!("Нажмите Enter для закрытия программы");
    io::stdin().read_line(&mut close).unwrap();
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
