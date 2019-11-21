use aravis::prelude::*;
use glib::{Cast, IsA};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(setting = structopt::clap::AppSettings::UnifiedHelpMessage)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
#[structopt(group = structopt::clap::ArgGroup::with_name("selector").required(true))]
struct Options {
	/// The IP address of the camera to connecto to.
	id: String,

	/// Show all options.
	#[structopt(long, short)]
	#[structopt(group = "selector")]
	all: bool,

	/// The option to get or set.
	#[structopt(long, short)]
	#[structopt(value_name = "NAME")]
	#[structopt(group = "selector")]
	feature: Option<String>,

	/// Set the value of the selected option.
	#[structopt(long, short)]
	#[structopt(value_name = "VALUE")]
	#[structopt(requires = "feature")]
	set: Option<String>,
}

fn init_logging() {
	env_logger::Builder::new().format(|buffer, record: &log::Record| {
		use std::io::Write;
		use env_logger::fmt::Color;

		let mut prefix_style = buffer.style();
		let prefix;

		match record.level() {
			log::Level::Trace => {
				prefix = "Trace: ";
				prefix_style.set_bold(true);
			}
			log::Level::Debug => {
				prefix = "";
			}
			log::Level::Info => {
				prefix = "";
			}
			log::Level::Warn => {
				prefix = "Warning: ";
				prefix_style.set_color(Color::Yellow).set_bold(true);
			}
			log::Level::Error => {
				prefix = "Error: ";
				prefix_style.set_color(Color::Red).set_bold(true);
			}
		};

		writeln!(
			buffer,
			"{}{}",
			prefix_style.value(prefix),
			record.args()
		)
	}).init();
}

fn main() {
	init_logging();
	if let Err(e) = do_main() {
		log::error!("{}", e);
		std::process::exit(1);
	}
}

fn do_main() -> Result<(), String> {
	let options = Options::from_args();

	log::info!("Connecting to camera {}.", options.id);
	let camera = aravis::Camera::new(Some(&options.id))
		.ok_or("Failed to connecto to camera.")?;

	let genicam = camera.get_device().unwrap().get_genicam().unwrap();

	if options.all {
		walk_genicam(&genicam, "Root", "").map_err(|e| format!("{}", e))?;
	} else if let Some(feature) = options.feature {
		if let Some(_set_value) = options.set {
			return Err(String::from("Setting feature values is not yet implemented."));
		} else {
			walk_genicam(&genicam, &feature, "").map_err(|e| format!("{}", e))?;
		}
	} else {
		unreachable!();
	}

	Ok(())
}

fn walk_genicam<T: IsA<aravis::Gc>>(genicam: &T, feature: &str, indent: &str) -> Result<(), glib::Error> {
	let node = genicam.get_node(feature).unwrap();

	if let Some(node) = node.dynamic_cast_ref::<aravis::GcBoolean>() {
		// TODO: fix GcBoolean::get_value().
		println!("{}{}: boolean {:?}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcCategory>() {
		if feature == "Root" {
			for feature in node.get_features() {
				walk_genicam(genicam, &feature, indent)?;
			}
		} else {
			println!("{}{}: category", indent, feature);
			for feature in node.get_features() {
				walk_genicam(genicam, &feature, &format!("  {}", indent))?;
			}
		}
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcCommand>() {
		println!("{}{}: command", indent, feature);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatRegNode>() {
		println!("{}{}: float {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcFloatNode>() {
		println!("{}{}: float {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntRegNode>() {
		println!("{}{}: integer {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcStringRegNode>() {
		println!("{}{}: string {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcIntegerNode>() {
		println!("{}{}: integer {}", indent, feature, node.get_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcEnumeration>() {
		println!("{}{}: enumeration {}", indent, feature, node.get_string_value()?);
	} else if let Some(node) = node.dynamic_cast_ref::<aravis::GcRegisterNode>() {
		println!("{}{}: register (0x{:02X}, {})", indent, feature, node.get_address()?, node.get_length()?);
	} else if let Some(_) = node.dynamic_cast_ref::<aravis::GcSwissKnife>() {
		println!("{}{}: swiss-knife", indent, feature);
	} else {
		println!("{:?}", node);
		unimplemented!();
	}

	Ok(())
}