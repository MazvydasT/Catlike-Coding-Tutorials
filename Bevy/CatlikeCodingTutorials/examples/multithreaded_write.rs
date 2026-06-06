use std::time::Instant;

use bevy::prelude::*;

const COUNT: usize = 100_000_000;

#[derive(Component, Clone)]
struct SomeComponent(f32, usize);

#[derive(Resource)]
//struct SomeVec([f32; COUNT]);
struct SomeVec(Vec<f32>);

fn main() -> AppExit {
    App::new()
        .insert_resource(SomeVec(vec![0.; COUNT]))
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (run_single_threaded, run_parallel).chain().run_if(run_once),
        )
        .run()
}

fn startup(mut commands: Commands) {
    commands.spawn_batch((0..COUNT).map(|index| SomeComponent(2., index)));
}

fn run_single_threaded(components: Query<&SomeComponent>, mut vector: ResMut<SomeVec>) {
    let start = Instant::now();

    components.iter().for_each(|component| {
        vector.0[component.1] = (component.0 * 10. + (component.1 as i32 as f32)).sqrt()
    });

    let elapsed = start.elapsed();

    println!("Single thread: {:?}", elapsed);
}

fn run_parallel(components: Query<&SomeComponent>, mut vector: ResMut<SomeVec>) {
    let start = Instant::now();

    let reference_to_vector = &mut vector.0;
    let pointer_to_vector = reference_to_vector.as_mut_ptr() as usize;

    components.par_iter().for_each(|component| {
        let local_pointer_to_vector = pointer_to_vector as *mut f32;

        unsafe {
            *local_pointer_to_vector.add(component.1) =
                (component.0 * 100. + (component.1 as i32 as f32)).sqrt();
        }
    });

    let elapsed = start.elapsed();

    println!("Multiple threads: {:?}", elapsed);
}
