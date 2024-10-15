        use anki_bridge::{prelude::{CardsInfoRequest, CardsInfoResponse, DeckNamesRequest, FindCardsRequest}, AnkiRequestable};
        use anyhow::Result;

        use crate::AppState;

        pub struct AnkiClient<'a> {
            communication_client: anki_bridge::AnkiClient<'a>
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum AnkiConnectionStatus {
            Connected,
            Connecting,
            Disconnected,
            CouldNotConnect,
        }

        impl<'a> AnkiClient<'a> {
            pub fn new() -> Self {
                let communication_client = anki_bridge::AnkiClient::new("http://localhost:8765");
                let anki_client = AnkiClient {
                    communication_client
                };
                anki_client
            }

            pub async fn get_decks(&self) -> Vec<String> {
                let decks = self.communication_client.request(DeckNamesRequest).await.unwrap();
                decks
            }

            pub async fn check_connection(&self) -> Result<()> {
                let _res = self.communication_client.request(DeckNamesRequest).await?;
                Ok(())
            }

            pub async fn add_card_to_deck(&self, deck_name: &str, front: &str, back: &str) -> Result<()> {
                //println!("Front: {}", front);
                //println!("Back: {}", back);
                let request = reqwest::Client::new()
                    .post("http://localhost:8765")
                    .json(&serde_json::json!({
                        "action": "addNote",
                        "version": 6,
                        "params": {
                            "note": {
                                "deckName": deck_name,
                                "modelName": "Basic",
                                "fields": {
                                    "Front": front,
                                    "Back": back
                                },
                                "options": {
                                    "allowDuplicate": false
                                },
                                "tags": []
                            }
                        }
                    }))
                    .send()
                    .await?;
                let response = request.text().await?;
                if response.contains("error\": null") {
                    Ok(())
                } else if response.contains("cannot create note because it is a duplicate") {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Anki Connect returned : {:?}", response))
                }

            }

            pub fn check_connection_non_blocking(app_state: &mut AppState) -> Result<AnkiConnectionStatus> {
                if app_state.anki_connection_status == AnkiConnectionStatus::Disconnected  || app_state.anki_connection_status == AnkiConnectionStatus::CouldNotConnect {
                    app_state.anki_connection_status = AnkiConnectionStatus::Connecting;
                    let (tx, rx) = tokio::sync::mpsc::channel(1);
                    app_state.channels.anki_connection_status_rc = Some(rx);
                    app_state.async_rt.spawn(async move {
                        let anki_client = AnkiClient::new();
                        let res = anki_client.check_connection().await;
                        match res {
                            Ok(_) => {
                                tx.send(AnkiConnectionStatus::Connected).await.unwrap();
                            }
                            Err(err) => {
                                eprintln!("Error: {:?}", err);
                                tx.send(AnkiConnectionStatus::CouldNotConnect).await.unwrap();
                            }
                        }
                    });
                }

                if let Some(rx) = &mut app_state.channels.anki_connection_status_rc {
                    if let Ok(status) = rx.try_recv() {
                        app_state.anki_connection_status = status;
                        app_state.first_attempt_at_connecting_to_anki = false;
                    }
                }

                Ok(app_state.anki_connection_status)
            }

            pub async fn get_cards_from_anki_deck(&self, deck_name: &str) -> Vec<CardsInfoResponse> {
                let card_ids = self.communication_client.request(
                    FindCardsRequest{
                        query: format!("deck:{}", deck_name)
                    }
                ).await.unwrap();
                let cards = self.communication_client.request(
                    CardsInfoRequest{
                        cards: card_ids
                    }
                ).await.unwrap();

                return cards;
            }

        }