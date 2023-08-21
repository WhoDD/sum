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

fn create_groups(mut players: Vec<Player>) -> Vec<Vec<Player>> {
    let mut groups: Vec<Vec<Player>> = Vec::new();
    let mut current_group: Vec<Player> = Vec::new();

    players.sort_by_key(|player| player.queue_time);

    while let Some(player) = players.pop() {
        if player.queue_time > 5 {
            current_group.push(player);
            if current_group.len() == 4 {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            match player.role {
                Role::Tank | Role::Healer | Role::Damager => {
                    current_group.push(player);
                }
                Role::Others => {
                    current_group.push(player);
                }
            }
            if current_group.len() == 4 {
                groups.push(current_group);
                current_group = Vec::new();
            }
        }
    }
    
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

fn main() {
    
    let players = vec![
        Player {
            id: 1,
            role: Role::Tank,
            queue_time: 3,
        },
        Player {
            id: 2,
            role: Role::Damager,
            queue_time: 8,
        },
        Player {
            id: 3,
            role: Role::Healer,
            queue_time: 2,
        },
        Player {
            id: 4,
            role: Role::Others,
            queue_time: 7,
        },
        Player {
            id: 5,
            role: Role::Damager,
            queue_time: 1,
        },
        Player {
            id: 6,
            role: Role::Tank,
            queue_time: 10,
        },
        Player {
            id: 7,
            role: Role::Healer,
            queue_time: 6,
        },
    ];

    let groups = create_groups(players);

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}: {:?}", i + 1, group);
    }
    
    delay();
}
fn delay(){
    let mut close = String::new();
    println!("Нажмите Enter для закрытия программы");
    io::stdin().read_line(&mut close).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_groups() {
        let players = vec![
            Player { id: 1, queue_time: 2, role: Role::Tank },
            Player { id: 2, queue_time: 6, role: Role::Healer },
            Player { id: 3, queue_time: 4, role: Role::Others },
            Player { id: 4, queue_time: 1, role: Role::Damager },
            Player { id: 5, queue_time: 7, role: Role::Others },
            Player { id: 6, queue_time: 3, role: Role::Tank },
            Player { id: 7, queue_time: 8, role: Role::Damager },
            Player { id: 8, queue_time: 9, role: Role::Healer },
        ];

        let expected_groups = vec![
            vec![Player { id: 8, queue_time: 9, role: Role::Healer }, Player { id: 7, queue_time: 8, role: Role::Damager }, Player { id: 5,  queue_time: 7, role: Role::Others }, Player { id: 2, queue_time: 6, role: Role::Healer }],
            vec![Player { id: 3, queue_time: 4, role: Role::Others }, Player { id: 6, queue_time: 3, role: Role::Tank }, Player { id: 1, queue_time: 2, role: Role::Tank }, Player { id: 4, queue_time: 1, role: Role::Damager }],
        ];

        let cloned_players: Vec<Player> = players.iter().cloned().collect();

        let result = create_groups(cloned_players);
        assert_eq!(result.len(), expected_groups.len());

        for (result_group, expected_group) in result.iter().zip(expected_groups.iter()) {
            assert_eq!(result_group.len(), expected_group.len());

            for (result_player, expected_player) in result_group.iter().zip(expected_group.iter()) {
                assert_eq!(result_player, expected_player);
            }
        }
    }
}
