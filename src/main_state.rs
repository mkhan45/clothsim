use std::cell::{Cell, RefCell};

use crate::error::SimError;
use egui_macroquad::macroquad::prelude::*;

const DT: f32 = 0.1;
const G: f32 = 0.0;
const NODE_RADIUS: f32 = 10.0;
const TARGET_DIST: f32 = 80.0;
const ELASTICITY: f32 = 1.0;

use generational_arena::{self, Arena};

#[derive(Clone)]
pub struct Node {
    pos: Vec2,
    last_pos: Vec2,
    vel: Vec2,
    force: Vec2,
    mass: f32,
    fixed: bool,
    adjs: Vec<generational_arena::Index>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            last_pos: Default::default(),
            vel: Default::default(),
            force: Default::default(),
            mass: 1.0,
            adjs: Default::default(),
            fixed: Default::default(),
        }
    }
}

impl Node {
    pub fn with_pos_and_mass(pos: Vec2, mass: f32) -> Node {
        Node {
            pos,
            last_pos: pos,
            mass,
            ..Node::default()
        }
    }

    pub fn with_pos(pos: Vec2) -> Node {
        Node::with_pos_and_mass(pos, 1.0)
    }

    #[allow(dead_code)]
    pub fn with_xym(x: f32, y: f32, m: f32) -> Node {
        Node::with_pos_and_mass(Vec2::new(x, y), m)
    }

    pub fn with_xy(x: f32, y: f32) -> Node {
        Node::with_pos(Vec2::new(x, y))
    }

    pub fn fixed(mut self) -> Node {
        self.fixed = true;
        self
    }

    pub fn integrate(&mut self) {
        if self.fixed {
            return;
        }

        let last_vel = (self.pos - self.last_pos) / DT;
        let last_accel = (last_vel - self.vel) / DT;
        let new_accel = self.force / self.mass;

        self.last_pos = self.pos;
        self.vel = self.vel + (last_accel + new_accel) * 0.5 * DT;
        self.pos = self.pos + self.vel * DT;
        self.force = Vec2::ZERO;
    }

    pub fn apply_gravity(&mut self) {
        if self.fixed {
            return;
        }

        self.force += Vec2::new(0.0, G);
    }

    pub fn simulate(&mut self, arena: &Arena<Node>) {
        for &i in self.adjs.iter() {
            let adj = arena.get(i).unwrap();

            let r = self.pos - adj.pos;
            let dist = r.length();
            let mult = 1.0 + (TARGET_DIST - dist);

            self.force += r * mult * ELASTICITY;
        }
    }
}

pub struct MainState {
    arena: generational_arena::Arena<Node>,
}

impl Default for MainState {
    fn default() -> Self {
        let mut arena = Arena::new();
        let mid = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        let a = arena.insert(Node::with_pos(mid + Vec2::new(20.0, 30.0)));
        let b = arena.insert(Node::with_pos(mid + Vec2::new(-20.0, -30.0)));

        arena.get_mut(a).unwrap().adjs.push(b);
        arena.get_mut(b).unwrap().adjs.push(a);


        Self { arena }
    }
}

impl MainState {
    pub fn simulate(&mut self) {
        todo!();
    }

    pub fn update(&mut self) -> Result<(), SimError> {
        for (_, node) in self.arena.iter_mut() {
            node.apply_gravity();
            todo!(); // simulate
            node.integrate();
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), SimError> {
        for (_, node) in self.arena.iter_mut() {
            draw_circle(node.pos.x, node.pos.y, NODE_RADIUS, WHITE);
        }

        Ok(())
    }
}
