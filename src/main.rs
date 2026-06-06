use bevy::prelude::*;

// defining components and resources

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // resources are global data, useful for things like config, score tracking, etc
        // adding the greet timer resource to the app, setting it to 2 seconds long and repeating
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

        // systems are functions that iterate over entities
        app.add_systems(Startup, add_people); // registering startup systems
        app.add_systems(Update,(greet_people, update_people).chain()); // registering update systems (every frame?)
    }
}

// Default Plugins adds core game engine functionality, including:
// binding Update system to an event loop
// creating a window with WinitPlugin
fn main() {
    App::new() // new app
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

// spawning new entities, with person component and name component
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Superman".to_string()) ) );
    commands.spawn((Person, Name("Madison A".to_string())) );
    commands.spawn((Person, Name("Elliot B".to_string()) ) );
}

// take in a query requesting entities' name component, if they also have the person component
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() { // if the timer has just finished on the last frame

        // loop through all the names returned by the query and greet them
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

// take in a mutable query that requests entities' name component in a mutable context, only if they also have the person component
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query { // go through all of the name components returned
        if name.0 == "Madison A" {
            name.0 = "Madison B".to_string(); // change madison a to madison b
            break;
        }
    }
}

// very interesting; uses an ECS system rather than an OO system