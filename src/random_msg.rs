use rand::prelude::SliceRandom;

pub fn get_random_message() -> String {
    let mut items = Vec::new();

    items.push("The PC that run's the server and this website is called apollo");
    items.push("secret message");
    items.push("How are things going?");
    items.push("I'm still waiting for dying light 2");
    items.push("I'm working on a voxel game as well right now");
    items.push("I made this website in two days (almost one)");
    items.push("I want to make improvements to this site over time");
    items.push("Yeah that map thing really does suck doesn't it");
    items.push("Lorem ipsum");
    items.push("I miss cometbot");
    items.push("Thank you for checking out the website");
    items.push("<3");
    items.push("Website by codedcosmos");
    items.push("Random message 1");
    items.push("Random message 2");
    items.push("Random message 4");
    items.push("Random message 8");
    items.push("Random message 16");
    items.push("This site isn't 16 bit");
    items.push("This is a random message");
    items.push("This message is secret");

    if let Some(item) = items.choose(&mut rand::thread_rng()) {
        return String::from(*item);
    }

    String::from("Couldn't find a random message :(")
}