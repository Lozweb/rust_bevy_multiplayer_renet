use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Serveur de jeu multijoueur")]
pub struct ServerArgs {
    /// lance le serveur en mode headless (sans interface graphique)
    #[arg(long, default_value_t = false)]
    pub headless: bool,

    /// Port d'écoute du serveur
    #[arg(long, default_value_t = 5000)]
    pub port: u16,
}
