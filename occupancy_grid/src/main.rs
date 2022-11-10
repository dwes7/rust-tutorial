use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum NodeState{
    Occupied(u8),
    Unoccupied(u8),
    Unknown(i8),
}
#[derive(Debug)]
pub struct Idx {
    pub i_row : usize,
    pub j_col : usize,
}

// Direction of neighboring node.
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Direction{
    North = 0,
    NorthEast = 1,
    East = 2, 
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

// Hashmap to store a GridNodes neighbors. Should only have 8 entries
type Neighbors = HashMap<Direction, Option<GridNode>>;

// Data structure to hold all information regarding a node in the GridMap
#[derive(Debug)]
pub struct GridNode {
    pub state : NodeState,
    pub index : Idx,
    pub creation_time : u128,
    pub neighbors : Neighbors,
    pub index_set : bool,
    pub neighbors_set : bool,
}

impl GridNode {
    fn new() -> Self {
        let neighbor_map = Neighbors::from([
            (Direction::North, None),
            (Direction::NorthEast, None),
            (Direction::East, None),
            (Direction::SouthEast, None),
            (Direction::South, None),
            (Direction::SouthWest, None),
            (Direction::West, None),
            (Direction::NorthWest, None),
        ]);
        
        GridNode {
            state: NodeState::Unknown(-1),
            index: None,
            creation_time : SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            neighbors : neighbor_map,
            index_set : false,
            neighbors_set : false,
        }
    }
}

// Occupancy grid data structure
// TODO change NodeState to GridNode
#[derive(Debug)]
pub struct OccupancyGrid{
    pub data : Vec<GridNode>,
    pub width : usize,
    pub height : usize,
    pub resulotion : f64,
}

impl OccupancyGrid {
    pub fn new(_width : usize, _height:usize, _res : f64) -> Self{
        let capacity = _width*_height;
        let mut _data : Vec<GridNode> = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity{
            let node = GridNode::new();
            // initialize nodes here before pushing
            _data.push(node);
        }
        OccupancyGrid { data: _data, width: _width, height: _height, resulotion : _res }
    }


    // pub fn add_obstacles(&mut self, indecies : &Vec<usize>){
    //     for idx in indecies{
    //         self.data[*idx] = NodeState::Occupied(255);
    //     }
    // }

    // Example grid map indicies.
    //   jp
    // i 0  1  2  3  4
    //   5  6  7  8  9 
    //   10 11 12 13 14
    //   15 16 17 18 19
    //   20 21 22 23 24
    // i=3, j=4 , (i + 1) * (j + 1) -1 = i*j + i + j = 19
    pub fn get_node_at(&self, _idx : Idx) -> Option<&GridNode>{
        let linear_idx = _idx.i_row * _idx.j_col + _idx.i_row + _idx.j_col;
        self.data.get(linear_idx as usize)
    }

    

    pub fn add_right_columns(&mut self, _num: i32){

    }
}

#[cfg(test)]
    mod tests_ogrid{
        use crate::GridNode;
        use crate::OccupancyGrid;
        use crate::NodeState;
        use crate::Idx;
        #[test]
        fn test_ogrid_creation(){
            let ogrid = OccupancyGrid::new(10, 10,1.0);
            assert_eq!(10, ogrid.width);
            assert_eq!(10, ogrid.height);
            assert_eq!(10*10, ogrid.data.len());
            for node in ogrid.data {
                // assert_eq!(NodeState::Unknown(-1), node);
            }
        }

        #[test]
        fn test_ogrid_get_node_at(){
            let width:usize = 10;
            let height :usize= 10;
            let ogrid = OccupancyGrid::new(10, 10, 1.0);
            // fill ogrid with grid nodes
            for i in 0..(width*height){
                let mut node = GridNode::new();
                node.state = NodeState::Occupied(i as u8);
                ogrid.data[i] = node;
            }

            for i in 0..width{
                for j in 0..height{
                    let linear_idx = i*j+i+j;
                    let new_node = ogrid.get_node_at(Idx{i_row: i as usize, j_col: j as usize});
                    // still need to test
                    
                }
            }


        }
    }

fn main() {
    let mut ogrid = OccupancyGrid::new(50, 50, 1.0);

    println!("Occupancy grid {:?}", ogrid);
    println!("vector size {}", ogrid.data.len());

}
