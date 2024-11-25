#[test]
fn usage_idea_test() {
    let botifactory_api = Botifactory::new("https://botifactory.izzys.place", "bb-bot").build();

    let project_json = botifactory_api
        .get_project()
        .expect("expected to get a project");
    println!("project json: {project_json}");

    let new_project_json = botifactory_api
        .create_new_project("bb-bot2")
        .expect("Expected to create a new project");
    println!("new project json: {new_project_json}");

    let stable_channel = botifactory_api.channel("stable");
    let channel_json = stable_channel
        .get_channel()
        .expect("expected to successfully get a channel");
    println!("stable channel json: {channel_json}");

    let new_channel_json = botifactory_api
        .create_new_channel("unstable")
        .expect("Expected to create new channel");
    println!("new channel json: {new_project_json}");

    let release = stable_channel.release("current");
}
