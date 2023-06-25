// project code to promote creativity and innovation
// file: creativity_and_innovation.rs

// imports
use std::collections::HashMap;

// global constants
const SUPPORT_TIMES: [u8; 3] = [30, 60, 120];

// struct definition for resources
struct Resources {
	name: String,
	file_linked: String,
	description: String,
}

// struct definition for makers and inventors
struct MakerInventor {
	name: String,
	age: u8,
	resources: Vec<Resources>,
}

// function to add resources
fn add_resource(name: String, file_linked: String, description: String) -> Resources {
	Resources { name, file_linked, description }
}

// function to add makers and inventors
fn add_maker_inventor(name: String, age: u8) -> MakerInventor {
	let resources = vec![
		add_resource(String::from("guide"), String::from("guide.pdf"), String::from("guide for makers and inventors")),
		add_resource(String::from("FAQ"), String::from("FAQ.pdf"), String::from("frequently asked questions")),
		add_resource(String::from("Blueprint"), String::from("blueprint.pdf"), String::from("blueprint for product")),
	];
	MakerInventor { name, age, resources }
}

// function to get the resource associated with a maker or inventor
fn get_resource(maker_inventor: &MakerInventor, resource_name: &str) -> Option<&Resources> {
	maker_inventor.resources.iter().find(|resource| resource.name == resource_name)
}

// main function implementing the project
fn main() {
	// create a new mutable map to store makers and inventors
	let mut makers_and_inventors: HashMap<String, MakerInventor> = HashMap::new();

	// add makers and inventors to the hashmap
	makers_and_inventors.insert(String::from("Albert Einstein"), add_maker_inventor(String::from("Albert Einstein"), 122));
	makers_and_inventors.insert(String::from("Steve Jobs"), add_maker_inventor(String::from("Steve Jobs"), 79));
	makers_and_inventors.insert(String::from("Thomas Edison"), add_maker_inventor(String::from("Thomas Edison"), 86));

	// loop through the makers and inventors and provide resources
	for (maker_inventor_name, maker_inventor) in &makers_and_inventors {
		println!("Providing resources for {}:", maker_inventor_name);

		// loop through the support times
		for &support_time in SUPPORT_TIMES.iter() {
			println!("- Providing {} minutes of support", support_time);

			// get the resource associated with the maker or inventor
			let resource = get_resource(&maker_inventor, "guide");
			if let Some(resource) = resource {
				// print the resource information
				println!("  - Resource: {}", resource.name);
				println!("  - File Linked: {}", resource.file_linked);
				println!("  - Description: {}", resource.description);
			}
		}
	}
}