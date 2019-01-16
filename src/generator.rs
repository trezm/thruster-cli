use std::process::Command;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use chrono::Utc;
use fuel_line::Render;

use crate::utils::*;

static TIMESTAMP_FORMAT: &str = "%Y-%m-%d-%H%M%S";

#[derive(Render)]
#[TemplateName = "./src/util.template.rs"]
struct UtilTemplate {}

pub fn migrate() {
  Command::new("diesel")
    .arg("migration")
    .arg("run")
    .output()
    .expect("failed to run migrations");

  Command::new("sh")
    .arg("-c")
    .arg("diesel print-schema > src/schema.rs")
    .output()
    .expect("failed to create schema");
}

pub fn create_component(raw_name: &str) {
  let mut chars = raw_name.chars();
  let name = match chars.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
  };

  create_dir(format!("src/{}s", &name.to_snek_case()))
    .expect("failed to create component directory");

  Command::new("mkdir")
    .arg("-p")
    .arg("src/models")
    .output()
    .expect("failed to create models directory");

  Command::new("sh")
    .arg("-c")
    .arg(format!("echo 'pub mod {}s;\n' >> src/models/mod.rs", name.to_snek_case()))
    .output()
    .expect("failed to create models directory");

  #[derive(Render)]
  #[TemplateName = "./src/controller.template.rs"]
  struct ControllerTemplate {
    snek_case: String,
    name: String,
    ctx: String
  }
  let mut controller_file = File::create(format!("src/{}s/{}_controller.rs", &name.to_snek_case(), &name.to_snek_case()))
    .expect("Could not create controller");
  controller_file.write_all((ControllerTemplate {
    snek_case: SnekCase::to_snek_case(&name),
    name: name.to_owned(),
    ctx: "Ctx".to_owned()
  }).render().as_bytes())
    .expect("Could not write controller to file");

  #[derive(Render)]
  #[TemplateName = "./src/service.template.rs"]
  struct ServiceTemplate {
    snek_case: String,
    name: String
  }
  let mut service_file = File::create(format!("src/{}s/{}_service.rs", &name.to_snek_case(), &name.to_snek_case()))
    .expect("Could not create service");
  service_file.write_all((ServiceTemplate {
    snek_case: SnekCase::to_snek_case(&name),
    name: name.to_owned()
  }).render().as_bytes())
    .expect("Could not write service to file");

  #[derive(Render)]
  #[TemplateName = "./src/mod.template.rs"]
  struct ModTemplate {
    snek_case: String,
    ctx: String
  }
  let mut mod_file = File::create(format!("src/{}s/mod.rs", &name.to_snek_case()))
    .expect("Could not create mod file");
  mod_file.write_all((ModTemplate {
    snek_case: SnekCase::to_snek_case(&name),
    ctx: "Ctx".to_owned()
  }).render().as_bytes())
    .expect("Could not write mod to file");

  #[derive(Render)]
  #[TemplateName = "./src/model.template.rs"]
  struct ModelTemplate {
    snek_case: String,
    name: String
  }
  let mut model_file = File::create(format!("src/models/{}s.rs", name.to_snek_case()))
    .expect("Could not create model file");
  model_file.write_all((ModelTemplate {
    snek_case: SnekCase::to_snek_case(&name),
    name: name.to_owned()
  }).render().as_bytes())
    .expect("Could not write model file");

  let migration_folder = format!("migrations/{}_create_{}", Utc::now().format(TIMESTAMP_FORMAT), name.to_snek_case());
  create_dir(&migration_folder)
    .expect("failed to create migration folder");

  let mut up_file = File::create(format!("{}/up.sql", migration_folder))
    .expect("Could not create up migration file");
  up_file.write_all(format!("CREATE TABLE {}s (
  id SERIAL PRIMARY KEY,
  some_field TEXT
);
", name.to_snek_case()).as_bytes())
    .expect("Could not create up migration file");

  let mut down_file = File::create(format!("{}/down.sql", migration_folder))
    .expect("Could not create up migration file");
  down_file.write_all(format!("DROP TABLE {}s
", name.to_snek_case()).as_bytes())
    .expect("Could not create up migration file");

  let message = templatify! { "Almost there! Your new component isn't linked up to a route, so just add something like the following to your App:

mod "; &name.to_snek_case() ;"s;

...

use crate::"; &name.to_snek_case() ;"s::{init as "; &name.to_snek_case() ;"_routes};

...

  let mut _app = App::<Ctx>::create(generate_context);

  ....

  _app.use_sub_app(\"/"; &name.to_snek_case() ;"s\", "; &name.to_snek_case() ;"_routes());
}
" };

  println!("{}", message);
}

pub fn init(name: &str) {
  Command::new("mkdir")
    .arg(name)
    .output()
    .expect("failed to create project directory");

  Command::new("cargo")
    .arg("init")
    .arg("--bin")
    .current_dir(name)
    .output()
    .expect("failed to initialize rust");

  let dependencies = "'diesel = { version = \"1.0.0\", features = [\"postgres\", \"r2d2\", \"uuid\"] }
dotenv = \"0.9.0\"
futures = \"0.1\"
lazy_static = \"0.2\"
r2d2 = \"0.8.3\"
serde = \"1.0.24\"
serde_json = \"1.0.8\"
serde_derive = \"1.0.24\"
thruster = \"0.6.3\"
time = \"0.1.38\"
tokio = \"0.1.3\"
tokio-proto = \"0.1\"
tokio-service = \"0.1\"
env_logger = { version = \"0.3.4\", default-features = false }
'";

  Command::new("sh")
    .arg("-c")
    .arg(format!("echo {} >> Cargo.toml", dependencies))
    .current_dir(name)
    .output()
    .expect("failed to add dependencies");

  let database_file = format!("DATABASE_URL=postgres://postgres@localhost/{}", name);
  Command::new("sh")
    .arg("-c")
    .arg(format!("echo {} > .env", database_file))
    .current_dir(name)
    .output()
    .expect("failed to create databse file");

  Command::new("diesel")
    .arg("setup")
    .current_dir(name)
    .output()
    .expect("failed to setup diesel");

  #[derive(Render)]
  #[TemplateName = "./src/main.template.rs"]
  struct MainTemplate {}
  let mut main_file = File::create(format!("{}/src/main.rs", name))
    .expect("Could not create main file");
  main_file.write_all((MainTemplate {}).render().as_bytes())
    .expect("Could not write main file");

  let mut context_file = File::create(format!("{}/src/util.rs", name))
    .expect("Could not create util file");
  context_file.write_all((UtilTemplate {}).render().as_bytes())
    .expect("Could not write util file");

  Command::new("mkdir")
    .arg(format!("{}/src/models", name))
    .output()
    .expect("failed to create models directory");

  let mut models_mod_file = File::create(format!("{}/src/models/mod.rs", name))
    .expect("Could not create models/mod file");
  models_mod_file.write_all("// Models
".as_bytes())
    .expect("Could not write models/mod file");

  #[derive(Render)]
  #[TemplateName = "./src/context.template.rs"]
  struct ContextTemplate {}
  let mut context_file = File::create(format!("{}/src/context.rs", name))
    .expect("Could not create context file");
  context_file.write_all((ContextTemplate {}).render().as_bytes())
    .expect("Could not write context file");

  let mut context_file = File::create(format!("{}/src/util.rs", name))
    .expect("Could not create util file");
  context_file.write_all((UtilTemplate {}).render().as_bytes())
    .expect("Could not write util file");

  #[derive(Render)]
  #[TemplateName = "./src/Dockerfile.template"]
  struct DockerfileTemplate {}
  let mut docker_file = File::create(format!("{}/Dockerfile", name))
    .expect("Could not create Dockerfile");
  docker_file.write_all((DockerfileTemplate {}).render().as_bytes())
    .expect("Could not write Dockerfile");

  #[derive(Render)]
  #[TemplateName = "./src/docker-compose.yml.template"]
  struct ComposeTemplate {
    db_name: String
  }
  let mut docker_file = File::create(format!("{}/docker-compose.yml", name))
    .expect("Could not create Dockerfile");
  docker_file.write_all((ComposeTemplate {
    db_name: name.to_owned()
  }).render().as_bytes())
    .expect("Could not write Dockerfile");

  #[derive(Render)]
  #[TemplateName = "./src/env.template"]
  struct EnvTemplate {
    db_name: String
  }
  let mut docker_file = File::create(format!("{}/.env", name))
    .expect("Could not create .env");
  docker_file.write_all((EnvTemplate {
    db_name: name.to_owned()
  }).render().as_bytes())
    .expect("Could not write .env");

  Command::new("mkdir")
    .arg(format!("{}/examples", name))
    .output()
    .expect("failed to create examples directory");

  #[derive(Render)]
  #[TemplateName = "./src/ping.template.rs"]
  struct PingExampleTemplate {}
  let mut docker_file = File::create(format!("{}/examples/ping.rs", name))
    .expect("Could not create ping example");
  docker_file.write_all((PingExampleTemplate {}).render().as_bytes())
    .expect("Could not write ping example");

  #[derive(Render)]
  #[TemplateName = "./src/schema.template.rs"]
  struct SchemaTemplate {}
  let mut docker_file = File::create(format!("{}/src/schema.rs", name))
    .expect("Could not create schema");
  docker_file.write_all((SchemaTemplate {}).render().as_bytes())
    .expect("Could not write schema");
}
