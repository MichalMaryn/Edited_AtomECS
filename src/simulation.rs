//! Plugin functionality for AtomECS
//! 
//! Allows a simulation to be created in a flexible manner by combining different plugins.

use std::any::{Any, type_name};
use specs::prelude::*;

use crate::{magnetic::MagneticsPlugin, atom::{AtomPlugin, ClearForceSystem}, sim_region::SimulationRegionPlugin, integrator::{VelocityVerletIntegratePositionSystem, INTEGRATE_POSITION_SYSTEM_NAME, INTEGRATE_VELOCITY_SYSTEM_NAME, VelocityVerletIntegrateVelocitySystem, Step}, gravity::GravityPlugin, destructor::DestroyAtomsPlugin, output::console_output::ConsoleOutputSystem};

/// A simulation in AtomECS.
pub struct Simulation {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>
}
impl Simulation {
    pub fn step(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

/// Used to construct a simulation in AtomECS.
pub struct SimulationBuilder {
    pub world: World,
    pub dispatcher_builder: DispatcherBuilder<'static, 'static>,
    end_frame_systems_added: bool,
    plugins: Vec<Box<dyn Plugin>>
}
impl SimulationBuilder {
    pub fn new() -> Self {
        let mut dispatcher_builder = DispatcherBuilder::default();

        dispatcher_builder.add(
            VelocityVerletIntegratePositionSystem,
            INTEGRATE_POSITION_SYSTEM_NAME,
            &[],
        );
        dispatcher_builder
            .add(ClearForceSystem, "clear", &[INTEGRATE_POSITION_SYSTEM_NAME]);

        SimulationBuilder {
            world: World::new(),
            dispatcher_builder,
            end_frame_systems_added: false,
            plugins: Vec::new()
        }
    }

    /// Add a [Plugin] to the [SimulationBuilder]
    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        self.check_plugin_dependencies(&plugin);
        plugin.build(self);
        self.plugins.push(Box::new(plugin));
    }

    fn check_plugin_dependencies(&self, plugin: &impl Plugin) {
        for dep in plugin.deps() {
            if !self.plugins.iter().map(|p| p.name()).any(|n| n == dep.name()) {
                panic!("Cannot add plugin {}: it requires a {} plugin, which has not yet been added.", plugin.name(), dep.name());
            }
        }
    }

    /// Builds a [Simulation] from the [SimulationBuilder].
    pub fn build(mut self) -> Simulation {

        if !self.end_frame_systems_added {
            self.add_end_frame_systems();
        }

        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut self.world);

        self.world.insert(Step { n: 0 });

        Simulation {
            world: self.world,
            dispatcher
        }
    }

    pub fn add_end_frame_systems(&mut self) {
        self.dispatcher_builder.add_barrier();
        self.dispatcher_builder.add(
            VelocityVerletIntegrateVelocitySystem,
            INTEGRATE_VELOCITY_SYSTEM_NAME,
            &[
                // No deps specified now - implicit in the barrier.
            ],
        );
        self.dispatcher_builder.add(ConsoleOutputSystem, "", &[INTEGRATE_VELOCITY_SYSTEM_NAME]);
        self.end_frame_systems_added = true;
    }
}
impl Default for SimulationBuilder {
    fn default() -> Self
    {
        let mut builder = Self::new();
        builder.add_plugin(AtomPlugin);
        builder.add_plugin(MagneticsPlugin);
        builder.add_plugin(SimulationRegionPlugin);
        builder.add_plugin(GravityPlugin);
        builder.add_plugin(DestroyAtomsPlugin);
        builder
    }
}

pub const DEFAULT_BEAM_NUMBER : usize = 8;

pub trait Plugin : Any + Send + Sync {
    fn build(&self, builder: &mut SimulationBuilder);
    fn name(&self) -> &str { type_name::<Self>() }
    fn deps(&self) -> Vec::<Box<dyn Plugin>>;
}