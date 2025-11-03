//! Module de gestion de l'état de debug et du journal de messages pour le serveur.
//!
//! Ce module fournit :
//! - Un enum `DebugMode` pour représenter le mode de debug courant.
//! - Une ressource `MessageLog` pour stocker un journal circulaire des messages échangés.
//! - Une structure `LogEntry` pour représenter chaque message loggé, avec horodatage, canal, direction et contenu.
//! - Un enum `MessageDirection` pour indiquer le sens du message (envoyé ou reçu).

use bevy::prelude::*;
use chrono::{DateTime, Local};
use std::time::SystemTime;

/// Modes de debug disponibles pour l'application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum DebugMode {
    /// Mode console (par défaut).
    #[default]
    Console,
    /// Mode caméra.
    Camera,
}

/// Ressource contenant le journal des messages.
///
/// `entries` contient les entrées du log.
/// `max_entries` limite le nombre d'entrées conservées (FIFO).
#[derive(Resource, Default)]
pub struct MessageLog {
    pub entries: Vec<LogEntry>,
    pub max_entries: usize,
}

/// Représente une entrée dans le journal des messages.
///
/// - `timestamp` : horodatage système du message.
/// - `channel` : nom du canal.
/// - `direction` : sens du message (envoyé/reçu).
/// - `content` : contenu du message.
#[derive(Clone)]
pub struct LogEntry {
    /// Horodatage système du message
    pub timestamp: SystemTime,
    pub channel: String,
    pub direction: MessageDirection,
    pub content: String,
}

impl LogEntry {
    /// Formate l'horodatage en date courte et heure.
    ///
    /// Format : `JJ/MM HH:MM:SS`
    pub fn formatted_timestamp(&self) -> String {
        let datetime: DateTime<Local> = self.timestamp.into();
        datetime.format("%d/%m %H:%M:%S").to_string()
    }
}

/// Sens du message dans le journal.
#[derive(Clone, Copy, PartialEq)]
pub enum MessageDirection {
    /// Message envoyé.
    Sent,
    /// Message reçu.
    Received,
}

impl MessageLog {
    /// Crée un nouveau journal de messages avec une capacité maximale.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Ajoute une entrée au log avec l'heure système actuelle.
    ///
    /// Si le nombre d'entrées dépasse la capacité maximale, la plus ancienne est supprimée.
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

    /// Vide le journal des messages.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
