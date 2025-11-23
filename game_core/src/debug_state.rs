//! Gestion de l'état de debug et du journal de messages pour le serveur.

use bevy::prelude::*;
use chrono::{DateTime, Local};
use std::time::SystemTime;

/// Modes de debug disponibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum DebugMode {
    /// Mode console
    #[default]
    Console,
    /// Mode caméra
    Camera,
}

/// Ressource contenant le journal des messages réseau.
#[derive(Resource, Default)]
pub struct Log {
    /// Entrées du log
    pub entries: Vec<LogEntry>,
    /// Nombre maximum d'entrées conservées (FIFO)
    pub max_entries: usize,
}

/// Entrée dans le journal des messages.
#[derive(Clone)]
pub struct LogEntry {
    /// Horodatage système du message
    pub timestamp: SystemTime,
    /// Nom du canal réseau
    pub channel: String,
    /// Direction du message (envoyé/reçu)
    pub direction: MessageDirection,
    /// Contenu du message
    pub content: String,
}

impl LogEntry {
    /// Formate l'horodatage au format `DD/MM HH:MM:SS`.
    pub fn formatted_timestamp(&self) -> String {
        let datetime: DateTime<Local> = self.timestamp.into();
        datetime.format("%d/%m %H:%M:%S").to_string()
    }
}

/// Direction d'un message dans le journal.
#[derive(Clone, Copy, PartialEq)]
pub enum MessageDirection {
    /// Message envoyé
    Sent,
    /// Message reçu
    Received,
}

impl Log {
    /// Crée un nouveau journal avec une capacité maximale.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Ajoute une entrée au log.
    ///
    /// Supprime la plus ancienne entrée si la capacité est dépassée.
    pub fn add(&mut self, channel: String, direction: MessageDirection, content: String) {
        self.entries.push(LogEntry {
            timestamp: SystemTime::now(),
            channel,
            direction,
            content,
        });

        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Vide le journal.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
