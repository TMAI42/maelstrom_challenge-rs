use std::io::{BufRead, Write};
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitData {
    Init { node_id: String, node_ids: Vec<String> },
    InitOk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub
enum EchoData {
    Echo { echo: String },
    EchoOk { echo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload<TypedData> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub typed_data: TypedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<TypedData>
    where TypedData: Serialize {
    pub src: String,
    pub dest: String,
    pub body: Payload<TypedData>,
}

impl<TypedData : Serialize> Message<TypedData> {
    pub fn send(&self, output: &mut impl Write) -> anyhow::Result<()>{
        serde_json::to_writer(&mut *output, self).context("Serialize message")?;

        output.write_all(b"\n").context("Writing message separator")?;
        Ok(())
    }
}

pub trait Node<T: Serialize> {
    fn init(msg: Message<InitData>)
        -> anyhow::Result<(Self, Message<InitData>)> where Self: Sized;
    fn handle(&mut self, msg: Message<T>) -> anyhow::Result<Message<T>>;
}

struct EchoNode {
    id: usize,
}

impl Node<EchoData> for EchoNode {
    fn init(msg: Message<InitData>)
        -> anyhow::Result<(Self, Message<InitData>)> where Self: Sized {
        Ok((EchoNode {id : 0}, match msg.body.typed_data {
            InitData::Init {..} => Message {
                src: msg.dest,
                dest: msg.src,
                body: Payload {
                    msg_id : Some(0),
                    in_reply_to: msg.body.msg_id,
                    typed_data: InitData::InitOk,
                }
            },
            InitData::InitOk  => panic!("recive InitOk")
        }))
    }

    fn handle(&mut self, msg: Message<EchoData>) -> anyhow::Result<Message<EchoData>> {
        match msg.body.typed_data {
            EchoData::Echo { echo} => Ok(Message{
                src: msg.dest,
                dest: msg.src,
                body: Payload {
                    msg_id : Some({
                        self.id = self.id + 1;
                        self.id
                    }),
                    in_reply_to: msg.body.msg_id,
                    typed_data: EchoData::EchoOk { echo },
                }
            }),
            EchoData::EchoOk {..} => panic!()
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock().lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg : Message<InitData> = serde_json::from_str(
        &stdin
            .next()
            .expect("No init message")
            .context("failed to read init message from stdin")?
    )?;

    let (mut state, message) = EchoNode::init(init_msg)?;

    message.send(&mut stdout)?;

    for line in stdin {
        let mut message : Message<EchoData> = serde_json::from_str(&line?).context("deserialize message")?;
        message = state.handle(message)?;
        message.send(&mut stdout)?;
    }

    Ok(())
}


