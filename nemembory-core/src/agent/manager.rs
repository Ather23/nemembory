// use crate::NememboryAgent;
// use tokio::sync::{ mpsc, broadcast };

// pub struct RemoteAgent {
//     pub agent: NememboryAgent,
//     pub input: mpsc::Receiver<String>,
//     pub input_tx: mpsc::Sender<String>,
//     pub output: broadcast::Sender<String>,
// }

// impl RemoteAgent {
//     pub fn new(agent: NememboryAgent) -> Self {
//         let (input_tx, input) = mpsc::channel::<String>(100);
//         let (output, _) = broadcast::channel::<String>(100);

//         Self {
//             agent,
//             input,
//             input_tx,
//             output,
//         }
//     }

//     pub async fn run(&mut self) -> Result<(), std::io::Error> {
//         while let Some(message) = self.input.recv().await {
//             println!("Received: {}", message);
//             let resp = &self.agent.run(&message, 2).await?;
//             if let Err(e) = self.output.send(resp.to_string()) {
//                 eprintln!("Failed to send response: {}", e);
//             }
//         }

//         Ok(())
//     }
// }
