use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(name = "ps_cli")]
#[command(author = "Giacomo Pantalone")]
#[command(version = "1.0")]
#[command(about = "A CLI for quick point clouds manipulations")]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

// The commands structure: 
#[derive(Parser, Debug)]
pub enum CliCommand {
    #[command(subcommand)]
    Create(CreateCommand),


    // Without subcommands
    Clear(ClearCommand),
    Corrode(CorrodeCommand),
    Relax(RelaxCommand),
}


#[derive(Subcommand, Debug)]   
pub enum CreateCommand {
    Cube {
        #[arg(long, default_value = "1")]
        side: f32,
    
        #[arg(long, default_value = "0.02")]
        step: f32,
    },
    Sphere {
        #[arg(long, default_value = "2")]
        radius: f32,
    
        #[arg(long, default_value = "0.03")]
        step: f32,
    }
}


#[derive(Parser, Debug)]
pub struct ClearCommand {

}


#[derive(Parser, Debug)]
pub struct CorrodeCommand {
    #[arg(long, default_value = "100")]
    iterations: usize,
}


#[derive(Parser, Debug)]
pub struct RelaxCommand {
    #[arg(long, default_value = "3")]
    iterations: usize,
}
