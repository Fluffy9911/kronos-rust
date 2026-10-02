use std::mem;
use crate::box2d::Box2d;

pub struct Tree {
    children: Option<Vec<Tree>>,
    nodes: Vec<(Box2d,usize)>,
    bounds: Box2d,
    max_depth: usize,
    max_objects: usize,
}

impl Tree {

    pub fn new(bounds: Box2d) -> Tree {

        Tree {children: None, nodes: vec![], bounds,max_depth:15,max_objects:10}

    }

    pub fn split(&mut self){
        if self.children.is_some()
        {
            //error
        }
        self.children = Some(Vec::new());
        let childs = self.children.as_mut().unwrap();
        let bounds_split = self.bounds.split();

        for bound in bounds_split {
         childs.push(Tree::new(bound));
        }
    }

    pub fn is_inside(&self, check:&Box2d) -> bool {
        self.bounds.is_inside_entirely(check)
    }

    pub fn append(&mut self,bounds:Box2d,id:usize){
        if self.is_inside(&bounds) {
            if self.children.is_some()
            {
                for child in self.children.as_mut().unwrap()
                {
                    let inside =  child.is_inside(&bounds);
                    if inside {
                        child.append(bounds, id);
                        return;
                    }
                }

            }   else if self.nodes.len() == self.max_objects {
                self.split();
                let copy = mem::replace(&mut self.nodes, Vec::new());
                for child in copy {
                    self.append(child.0, child.1)
                }
            } else {
                self.nodes.push((bounds, id));

            }
        }
    }

    pub fn search(&self, check:&Box2d,mut func:impl FnMut(&Box2d, usize) -> bool){

        for item in self.nodes.iter(){

            if func(check, item.1) {
                return;
            }
        }

    }

    pub fn closest(&self,check:&Box2d) -> &Box2d{

        if self.intersects



    }

}

