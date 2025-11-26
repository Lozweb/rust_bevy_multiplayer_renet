use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Serveur de jeu multijoueur (console uniquement)"
)]
pub struct ServerArgs {
    #[arg(long, default_value_t = 5000)]
    pub port: u16,
}
