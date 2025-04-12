use std::array;

use bevy::app::App;
use wacky_bag::structures::grid::Grid2D;

use super::grid::{GridCell, GridResource};


pub fn grid_plugin<const XSIZE:usize,const YSIZE:usize>(func:impl Fn((usize,usize)))->impl FnOnce(&mut App) {
    return move |app|{
        /*
        let grid_entities=Grid2DVec::<Entity>::new(size, |pos|{
            app.world_mut().spawn((
                func(pos),
                GridCell(pos)
            )).id()
        }); */
        let grid_entities=Grid2D::new( array::from_fn(|y|{array::from_fn(|x|{
            let pos=(x,y);
            app.world_mut().spawn((
                func(pos),
                GridCell(pos)
            )).id()
        })}));
        let resource=GridResource::<XSIZE,YSIZE> { grid_entities };
        app.insert_resource(resource);
        //let dawdawd:usize=Num::ONE.into();
    };
}
