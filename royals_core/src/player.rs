use crate::{card::Card, play::Action, Event};

pub type PlayerId = usize;

pub struct PlayerData {
    id: PlayerId,
    name: String,
    protected: bool,
    hand: Vec<Card>,
}

impl PlayerData {
    pub fn new(id: PlayerId, name: String) -> Self {
        PlayerData {
            id,
            name,
            protected: false,
            hand: vec![],
        }
    }
}

pub trait Player {
    fn data(&self) -> &PlayerData;

    fn data_mut(&mut self) -> &mut PlayerData;

    fn id(&self) -> PlayerId {
        self.data().id
    }

    fn name(&self) -> &String {
        &self.data().name
    }

    fn protected(&self) -> bool {
        self.data().protected
    }

    fn set_protected(&mut self, value: bool) {
        self.data_mut().protected = value;
    }

    fn hand(&self) -> &Vec<Card> {
        &self.data().hand
    }

    fn hand_mut(&mut self) -> &mut Vec<Card> {
        &mut self.data_mut().hand
    }

    fn is_active(&self) -> bool {
        !&self.hand().is_empty()
    }

    fn notify(&self, game_log: &[Event], players: &[&String]);

    fn obtain_action(
        &self,
        players: &[&String],
        game_log: &[Event],
        valid_actions: &[Action],
    ) -> usize;
}
