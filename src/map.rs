use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize; //number of tiles in our map

#[derive(Copy, Clone, PartialEq)]
//clone - adds clone function to type, copy - copy copies values, partialeq lets you compare with == operator
pub enum TileType {
    Wall,
    Floor,
    Exit
}

// struct describing our map, which contains a vector of TileType, which is enum defined earlier.
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>
}

// algorithm2d - trait, translates between my map and bracket-lib's generic mapping stuff.
// requires i implement a trait that maps my system to bracket-lib's system
// algorithm2d includes the following:
//    dimensions - size of map
//    in_bounds - determines if an x/y coordinate is valid and within map
//    point2d_to_index - our map indexing function pretty much
//    index_to_point2d - opposite of our indexing function
// bracket-lib automatically defines point2d_to_index and index_to_point2d
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point { //dimensions - returns point, which is size of map
	Point::new(SCREEN_WIDTH, SCREEN_HEIGHT) 
    }

    fn in_bounds(&self, point: Point) -> bool { //boolean checking if point is in bounds
	self.in_bounds(point)
    }
    //these two provide sufficient information for bracket-lib to implement the other trait functions.
}

// we now need a second trait - basemap
// basemap tells bracket-lib how you can travel through the map
// basemap requires these two functions:
//    get_available_exits - looks at tile, provides list of possible exits from that tile
//    get_pathing_distance - estimates distance between any two points
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool{
	self.tiles[idx as usize] != TileType::Floor
    }
    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
	let mut exits = SmallVec::new(); //return vector
	let location = self.index_to_point2d(idx); //current location of target

	//try each of the four tiles - if it's valid, add it to the vector.
	if let Some(idx) = self.valid_exit(location, Point::new(-1,0)) {exits.push((idx,1.0))}
	if let Some(idx) = self.valid_exit(location, Point::new(1,0))  {exits.push((idx,1.0))}
	if let Some(idx) = self.valid_exit(location, Point::new(0,-1)) {exits.push((idx,1.0))}
	if let Some(idx) = self.valid_exit(location, Point::new(0,1)) {exits.push((idx,1.0))}
	exits //return our list of valid exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
	DistanceAlg::Pythagoras               // use pythagoreanm theorem. Distancealg is an enum.
	    .distance2d(
		self.index_to_point2d(idx1),
		self.index_to_point2d(idx2)
	    )
    }
}


//Indexing map - here, we will be using row-first encoding, ie, 012,345,678, from left toright. With this encoding, we can calculate our tile index by let index = (y * WIDTH) + x;
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self { //ctor
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], // fills map with floor tiles
	    revealed_tiles: vec![false; NUM_TILES] // empty revealed tile vector
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
	let destination = loc + delta; // destination -> current location + movement
	if self.in_bounds(destination) { // is our destination in bounds?
	    if self.can_enter_tile(destination) { //can you enter destination?
		let idx = self.point2d_to_index(destination); //turn destination to index
		Some(idx) // Return index if valid
	    } else { None } // Can't enter? Return nothing.
	} else { None } // Not in bounds? Return nothing.

    }

    // checking bounds
    pub fn in_bounds(&self, point: Point) -> bool { 
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // check if player can enter a tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && (
	    self.tiles[map_idx(point.x, point.y)] == TileType::Floor ||
		self.tiles[map_idx(point.x, point.y)] == TileType::Exit
		)
    }

    //try an index
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) { //are we OOB?
            None //return nothing
        } else { //are we in bounds?
            Some(map_idx(point.x, point.y)) //return an index (good!)
        }
    }
}
