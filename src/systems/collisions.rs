use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
// CommandBuffer is a special container to insert instructions for Legion to perform after the system is finished.
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer){
    let mut players = <&Point>::query().filter(component::<Player>());
    let player_pos = players.iter(ecs).nth(0).unwrap();
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
    .iter(ecs)
    .filter(|(_,pos)| *pos == player_pos)
    .for_each(|(entity,_)| commands.remove(*entity));
}