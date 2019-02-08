use crate::util::*;

pub struct Game {
    pub arrows: u8,
    pub current_room: u8,
    pub messages: Vec<String>,
    pub wumpus: u8,
    bats: [u8; 2],
    pits: [u8; 2],
}

impl Game {
    fn configure_cave(&mut self) {
        self.messages.push(
            "voce entrou em uma caverna escura, armado com cinco flechas.  Você está com muito frio."
                .to_string(),
        );
        self.wumpus = js_rand(1, 20);
        self.bats[0] = self.get_empty_room();
        self.bats[1] = self.get_empty_room();
        self.pits[0] = self.get_empty_room();
        self.pits[1] = self.get_empty_room();
        self.warning_messages();
    }

    fn get_empty_room(&self) -> u8 {
        gen_range_avoiding(
            0,
            20,
            vec![
                self.current_room,
                self.wumpus,
                self.bats[0],
                self.bats[1],
                self.pits[0],
                self.pits[1],
            ],
        )
    }

    pub fn move_effects(&mut self) -> Option<String> {
        self.warning_messages();
        if self.current_room == self.wumpus {
            Some("Voce foi comido lenta e dolorasamente pelo wumpus".into())
        } else if self.pits.contains(&self.current_room) {
            Some(
        "Voce caiu em um poço sem fundo, espere pela morte de sede , depois de cair durante 3 dias"
          .into(),
      )
        } else if self.bats.contains(&self.current_room) {
            // Switch us to a random room
            let current = self.current_room;
            let next = self.get_empty_room();
            self.messages.push(format!(
                "Um morcego gigante te exotou da sala  {} para a sala {} antes de voce conseguir piscar",
                current, next
            ));
            self.current_room = next;
            self.warning_messages();
            None
        } else {
            None
        }
    }

    pub fn warning_messages(&mut self) {
        for adj in &room_exits(self.current_room).unwrap() {
            let t = *adj;
            if self.wumpus == t {
                self.messages
                    .push("Voce sente um cheiro de algo horrivel, nojento.".into());
            } else if self.pits.contains(&t) {
                self.messages
                    .push("Voce sente uma brisa muito fria vindo da caverna.".into());
            } else if self.bats.contains(&t) {
                self.messages
                    .push("Voce escunta um leve barulho de bater de asas.".into());
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut ret = Self {
            arrows: 5,
            current_room: 1,
            messages: Vec::new(),
            wumpus: 0,
            bats: [0, 0],
            pits: [0, 0],
        };
        ret.configure_cave();
        ret
    }
}
