use std::process::Command;
use std::str;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sh");
    let topic_list = cmd.arg("-c").arg("ros2 topic list").output()?;
    let topic_list = str::from_utf8(&topic_list.stdout)?.split("\n").collect::<Vec<&str>>();
    let mut no_pub = Vec::new();
    let mut no_sub = Vec::new();
    cmd = Command::new("sh");
    for topic in topic_list {
        let output = cmd.arg("-c").arg(format!("ros2 topic info {topic}")).output()?;
        let output = str::from_utf8(&output.stdout)?;
        let output: Vec<&str>= output.split("\n").collect();
        let pub_count:Vec<&str> = output[1].split(" ").collect();
        let pub_count = pub_count[2].parse::<i32>()?;
        let sub_count:Vec<&str> = output[2].split(" ").collect();
        let sub_count = sub_count[2].parse::<i32>()?;
        if pub_count == 0 {
            no_pub.push(topic);
        }
        if sub_count == 0 {
            no_sub.push(topic);
        }
    }
    println!("Topics with no publishers: {:?}", no_pub);
    println!("Topics with no subscribers: {:?}", no_sub);
    Ok(())
}
