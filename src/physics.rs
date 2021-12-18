//! An easy to use wrapper over Rapier.

use rapier2d::prelude::{
   BroadPhase, CCDSolver, ColliderSet, IntegrationParameters, IslandManager, JointSet, NarrowPhase,
   PhysicsPipeline, QueryPipeline, RigidBodySet,
};
use tetra::math::Vec2;

use crate::common::ToNalgebraVector2;

pub struct Physics {
   pub gravity: Vec2<f32>,

   pub rigid_bodies: RigidBodySet,
   pub colliders: ColliderSet,
   pub joints: JointSet,

   pub parameters: IntegrationParameters,
   pub pipeline: PhysicsPipeline,
   pub query: QueryPipeline,

   pub island_manager: IslandManager,
   pub broad_phase: BroadPhase,
   pub narrow_phase: NarrowPhase,
   pub ccd_solver: CCDSolver,
}

impl Physics {
   /// Creates a new bundle of physics state, with the specified gravational force vector.
   pub fn new(gravity: Vec2<f32>) -> Self {
      Self {
         gravity,
         rigid_bodies: RigidBodySet::new(),
         colliders: ColliderSet::new(),
         joints: JointSet::new(),

         parameters: IntegrationParameters {
            dt: (60.0 / 2.0),
            erp: 1.0,
            ..IntegrationParameters::default()
         },
         pipeline: PhysicsPipeline::new(),
         query: QueryPipeline::new(),

         island_manager: IslandManager::new(),
         broad_phase: BroadPhase::new(),
         narrow_phase: NarrowPhase::new(),
         ccd_solver: CCDSolver::new(),
      }
   }

   /// Steps the physics state.
   pub fn step(&mut self) {
      // Perform two steps to hopefully make penetrations less obvious.
      for _ in 0..2 {
         // self.pipeline.step(
         //    &self.gravity.nalgebra(),
         //    &self.parameters,
         //    &mut self.island_manager,
         //    &mut self.broad_phase,
         //    &mut self.narrow_phase,
         //    &mut self.rigid_bodies,
         //    &mut self.colliders,
         //    &mut self.joints,
         //    &mut self.ccd_solver,
         //    &(),
         //    &(),
         // )
      }
      self.query.update(&self.island_manager, &self.rigid_bodies, &self.colliders);
   }
}

/// Collision group bits.
pub struct CollisionGroups;

impl CollisionGroups {
   pub const PLAYER: u32 = 0b0000_0001;
   pub const SOLIDS: u32 = 0b0001_0000;
   pub const ALL: u32 = Self::PLAYER | Self::SOLIDS;
}
