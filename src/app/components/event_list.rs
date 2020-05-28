use linked_hash_set::LinkedHashSet;
use log::*;
use matrix_sdk::identifiers::RoomId;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::app::matrix::types::MessageWrapper;
use crate::app::matrix::{MatrixAgent, Request, Response};

pub struct EventList {
    link: ComponentLink<Self>,
    state: State,
    matrix_agent: Box<dyn Bridge<MatrixAgent>>,
    props: Props,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    // TODO handle all events
    pub events: LinkedHashSet<MessageWrapper>,
}

pub enum Msg {
    NewMessage(Response),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub current_room: Option<RoomId>,
    #[prop_or_default]
    pub displayname: String,
}

impl Component for EventList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let matrix_callback = link.callback(Msg::NewMessage);
        let mut matrix_agent = MatrixAgent::bridge(matrix_callback);

        let state = State {
            events: Default::default(),
        };

        if props.clone().current_room.is_some() {
            let room_id = props.clone().current_room.clone().unwrap();
            if state
                .events
                .iter()
                .filter(|x| x.room_id.clone().unwrap() == room_id)
                .collect::<LinkedHashSet<&MessageWrapper>>()
                .is_empty()
            {
                matrix_agent.send(Request::GetOldMessages((room_id.clone(), None)));
            }
        }

        EventList {
            props,
            link,
            matrix_agent,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::NewMessage(response) => {
                match response {
                    Response::Sync(msg) => {
                        // TODO handle all events
                        self.state.events.insert(msg);
                        true
                    }
                    Response::OldMessages(messages) => {
                        // TODO this doesn't seem smart
                        let mut new_events_map = LinkedHashSet::new();
                        for event in self.state.events.clone().into_iter() {
                            new_events_map.insert(event);
                        }
                        for event in messages.into_iter() {
                            new_events_map.insert(event);
                        }
                        self.state.events = new_events_map;
                        true
                    }

                    _ => false,
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        return html! {
            <div class="container uk-flex uk-flex-column uk-width-5-6 uk-padding uk-padding-remove-bottom" style="height: 100%">
                <h1>{ self.props.displayname.clone() }</h1>
                <div class="scrollable" style="height: 100%">
                    { self.state.events.iter().filter(|x| x.room_id.clone().unwrap() == self.props.current_room.clone().unwrap()).map(|event| self.get_event(event.clone())).collect::<Html>() }
                    <div id="anchor"></div>
                </div>
            </div>
        };
    }
}

impl EventList {
    fn get_event(&self, event: MessageWrapper) -> Html {
        html! {
            <p>{event.sender_displayname.unwrap_or(event.sender.to_string()).clone()}{": "}{event.content.clone()}</p>
        }
    }
}