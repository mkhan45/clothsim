use crate::error::SimError;
use egui_macroquad::macroquad::prelude::*;

const DT: f32 = 0.05;
const G: f32 = 18.0;
const NODE_RADIUS: f32 = 5.5;
const ROPE_WIDTH: f32 = 5.0;
const TARGET_DIST: f32 = 12.5;
const RIGIDITY: f32 = 1.0;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    last_pos: Vec2,
    pos: Vec2,
    vel: Vec2,
    force: Vec2,
    mass: f32,
    fixed: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            last_pos: Default::default(),
            vel: Default::default(),
            force: Default::default(),
            mass: 1.0,
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

    #[allow(dead_code)]
    pub fn with_xy(x: f32, y: f32) -> Node {
        Node::with_pos(Vec2::new(x, y))
    }

    #[allow(dead_code)]
    pub fn fixed(mut self) -> Node {
        self.fixed = true;
        self
    }

    pub fn integrate(&mut self) {
        if self.fixed {
            return;
        }

        let acc = self.force / self.mass;

        self.last_pos = self.pos;
        self.vel = self.vel + acc * DT;
        self.pos = self.pos + self.vel * DT;
    }

    pub fn differentiate(&mut self) {
        if self.fixed {
            return;
        }

        self.vel = (self.pos - self.last_pos) / DT;
        self.force = Vec2::ZERO;
    }

    pub fn apply_gravity(&mut self) {
        if self.fixed {
            return;
        }

        self.force += Vec2::new(0.0, G * self.mass);
    }

    pub fn add_offs(&mut self, offs: Vec2) {
        if !self.fixed {
            self.pos += offs;
        }
    }
}

pub struct Constraint {
    a: usize,
    b: usize,
}

impl Constraint {
    pub fn solve(&self, arena: &mut Vec<Node>) {
        let (a_offs, b_offs) = {
            let a = &arena[self.a];
            let b = &arena[self.b];

            let r = b.pos - a.pos;
            let dist = r.length();

            let norm = r.normalize_or_zero();
            let diff = dist - TARGET_DIST;
            let offs = norm * diff * RIGIDITY / (a.mass + b.mass);

            (offs / a.mass, -offs / b.mass)
        };

        arena[self.a].add_offs(a_offs);
        arena[self.b].add_offs(b_offs);
    }
}

pub struct MainState {
    arena: Vec<Node>,
    constraints: Vec<Constraint>,
}

impl Default for MainState {
    fn default() -> Self {
        let mut arena = Vec::new();
        let mid = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        for i in 0..20 {
            arena.push(Node::with_pos(mid + Vec2::new(0.0, 10.0 * i as f32)));
        }
        arena[0].fixed = true;

        let mut constraints = Vec::new();
        for i in 0..19 {
            constraints.push(Constraint { a: i, b: i + 1 });
        }

        Self { arena, constraints }
    }
}

impl MainState {
    pub fn update(&mut self) -> Result<(), SimError> {
        self.arena.iter_mut().for_each(Node::apply_gravity);
        self.arena.iter_mut().for_each(Node::integrate);

        for constraint in self.constraints.iter() {
            constraint.solve(&mut self.arena);
        }

        self.arena.iter_mut().for_each(Node::differentiate);

        if is_key_down(KeyCode::LeftShift) {
            self.arena[0].pos = mouse_position().into();
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), SimError> {
        for constraint in self.constraints.iter() {
            let a = self.arena[constraint.a];
            let b = self.arena[constraint.b];
            draw_line(a.pos.x, a.pos.y, b.pos.x, b.pos.y, ROPE_WIDTH, WHITE);
        }

        for node in self.arena.iter() {
            let c = if node.fixed { RED } else { WHITE };
            draw_circle(node.pos.x, node.pos.y, NODE_RADIUS, c);
        }

        Ok(())
    }
}
