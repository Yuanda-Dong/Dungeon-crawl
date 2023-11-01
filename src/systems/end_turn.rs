use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .nth(0)
        .unwrap_or(&amulet_default);
    let mut player = <(&Point, &Health)>::query().filter(component::<Player>());
    let (player_pos, player_hp) = player.iter(ecs).nth(0).unwrap();
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return, // early return
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };
    if map.tiles[map.point2d_to_index(*player_pos)] == TileType::Exit{
        new_state = TurnState::NextLevel;
    }

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }
    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }
    *turn_state = new_state;
}
