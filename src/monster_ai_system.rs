use specs::prelude::*;
use super::{Viewshed, Position, Map, Monster};
use rltk::{field_of_view, Point, Console, console};

pub struct MonsterAi { }


impl<'a> System<'a> for MonsterAi {
    type SystemData = ( ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster) = data;

        for  (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join(){
            console::log("Monster considers its own existence");
        }
    }
}