#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec,
};

// Struktur data Note
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    id: u64,
    title: String,
    content: String,
    created_at: u64,
    updated_at: u64,
}

// Storage key
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // Ambil semua notes
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah note baru
    pub fn create_note(
        env: Env,
        title: String,
        content: String,
    ) -> String {

        // Ambil notes lama
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        // Ambil timestamp sekarang
        let now = env.ledger().timestamp();

        // Buat note baru
        let note = Note {
            id: env.prng().gen::<u64>(),
            title,
            content,
            created_at: now,
            updated_at: now,
        };

        // Simpan note
        notes.push_back(note);

        env.storage()
            .instance()
            .set(&NOTE_DATA, &notes);

        String::from_str(&env, "Note berhasil ditambahkan")
    }

    // Edit note berdasarkan id
    pub fn edit_note(
        env: Env,
        id: u64,
        new_title: String,
        new_content: String,
    ) -> String {

        // Ambil data notes
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari note
        for i in 0..notes.len() {

            let mut note = notes.get(i).unwrap();

            if note.id == id {

                // Update data
                note.title = new_title;
                note.content = new_content;
                note.updated_at = env.ledger().timestamp();

                // Simpan kembali note
                notes.set(i, note);

                // Update storage
                env.storage()
                    .instance()
                    .set(&NOTE_DATA, &notes);

                return String::from_str(&env, "Note berhasil diupdate");
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }

    // Hapus note berdasarkan id
    pub fn delete_note(env: Env, id: u64) -> String {

        // Ambil data notes
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari note
        for i in 0..notes.len() {

            if notes.get(i).unwrap().id == id {

                // Hapus note
                notes.remove(i);

                // Simpan ulang
                env.storage()
                    .instance()
                    .set(&NOTE_DATA, &notes);

                return String::from_str(&env, "Berhasil hapus note");
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }
}

mod test;