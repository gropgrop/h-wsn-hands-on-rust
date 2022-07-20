use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing( #[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer ) {
    // find entities with point position and changing-player tags
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    // list all entities with point and health components
    let mut positions = <(Entity, &Point, &Health)>::query();
    // find player
    let mut player = <(&Point, &Player)>::query();
    // open player query, get position and index
    let player_pos = player.iter(ecs).next().unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    //DIJKSTRA MAP
    let search_targets = vec![player_idx];
    //                   (MAP WIDTH, MAP HEIGHT, TARGETS, MAP BORROWED REFERENCE SO NO NEED FOR &, MAX DIST)
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    //ACTUALLY CHASING
    movers.iter(ecs).for_each(|(entity,pos,_,fov)| {
	if !fov.visible_tiles.contains(&player_pos) {
	    return;
	}
	let idx = map_idx(pos.x, pos.y);
	if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
	    let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
	    let destination = if distance > 1.2 {
		map.index_to_point2d(destination)
	    } else {
		*player_pos
	    };
	    let mut attacked = false;
	    positions
		.iter(ecs)
		.filter(|(_, target_pos,_)| **target_pos == destination)
		.for_each(|(victim, _, _)| {
		    if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
			commands.
			    push(((), WantsToAttack{
				attacker: *entity,
				victim: *victim
			    }));
		    }
		    attacked = true;
		});
	    if !attacked { commands.push(((), WantsToMove{ entity: *entity, destination})); }
	}
    });
}
    
