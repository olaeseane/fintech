mod errors;
mod paths;

use errors::CliError;
use octopus_common::{
    core::types::{
        AccountBalanceRequest, AccountUpdateRequest, Order, PartialOrder, Receipt, SendRequest,
        Side,
    },
    tx::Tx,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{env, io, num::ParseIntError};

enum Operation {
    Deposit,
    Withdraw,
}

struct OctopusClient {
    base_url: Url,
    client: reqwest::Client,
}

impl OctopusClient {
    fn new(base_url: &str) -> Result<Self, CliError> {
        let base_url = reqwest::Url::parse(base_url)?;

        Ok(Self {
            base_url,
            client: reqwest::Client::new(),
        })
    }

    async fn deposit_withdraw(&self, op_type: Operation) -> Result<Tx, CliError> {
        let account = read_from_stdin("Account:");

        let amount = read_from_stdin("Amount:")
            .parse::<u64>()
            .map_err(|_| CliError::InvalidNumber())?;

        let op = AccountUpdateRequest {
            signer: account,
            amount,
        };

        let path = match op_type {
            Operation::Deposit => self.base_url.join(paths::DEPOSIT)?,
            Operation::Withdraw => self.base_url.join(paths::WITHDRAW)?,
        };

        send_request(&self.client, path, Some(op)).await
    }

    async fn send(&self) -> Result<(Tx, Tx), CliError> {
        let sender = read_from_stdin("Sender Account:");
        let recipient = read_from_stdin("Recipient Account:");
        let amount = read_from_stdin("Amount:")
            .parse::<u64>()
            .map_err(|_| CliError::InvalidNumber())?;

        let op = SendRequest {
            sender,
            recipient,
            amount,
        };

        send_request(&self.client, self.base_url.join(paths::SEND)?, Some(op)).await
    }

    async fn order(&self) -> Result<Receipt, CliError> {
        let order = read_order_parameters().map_err(CliError::InvalidOrderParameters)?;

        send_request(&self.client, self.base_url.join(paths::ORDER)?, Some(order)).await
    }

    async fn balance(&self) -> Result<u64, CliError> {
        let account = read_from_stdin("Account:");

        let op = AccountBalanceRequest { signer: account };

        send_request(&self.client, self.base_url.join(paths::BALANCE)?, Some(op)).await
    }

    async fn accounts(&self) -> Result<Vec<(String, u64)>, CliError> {
        send_request::<(), Vec<(String, u64)>>(
            &self.client,
            self.base_url.join(paths::ACCOUNTS)?,
            None,
        )
        .await
    }

    async fn orderbook(&self) -> Result<Vec<PartialOrder>, CliError> {
        send_request::<(), Vec<PartialOrder>>(
            &self.client,
            self.base_url.join(paths::ORDERBOOK)?,
            None,
        )
        .await
    }

    async fn txlog(&self) -> Result<Vec<Tx>, CliError> {
        send_request::<(), Vec<Tx>>(&self.client, self.base_url.join(paths::TXLOG)?, None).await
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("USAGE octopus-cli <SERVER_URL>");
    }

    let cli = OctopusClient::new(args[1].as_str()).expect("failed to parse SERVER_URL");

    println!("Hello, accounting world!");

    loop {
        let input = read_from_stdin(
            "Choose operation [deposit(d), withdraw(w), send(s), balance(b), accounts(a), txlog(tx), order(o), orderbook(ob), quit(q)], confirm with return:",
        );
        match input.as_str() {
            "deposit" | "d" => handle_command(cli.deposit_withdraw(Operation::Deposit).await),

            "withdraw" | "w" => handle_command(cli.deposit_withdraw(Operation::Withdraw).await),

            "send" | "s" => handle_command(cli.send().await),

            "order" | "o" => handle_command(cli.order().await),

            "balance" | "b" => handle_command(cli.balance().await),

            "accounts" | "a" => handle_command(cli.accounts().await),

            "orderbook" | "ob" => handle_command(cli.orderbook().await),

            "txlog" | "tx" => handle_command(cli.txlog().await),

            "quit" | "q" => {
                println!("Quitting...");
                break;
            }
            _ => {
                eprintln!("Invalid option: '{}'", input);
            }
        }
    }
}

fn read_order_parameters() -> Result<Order, String> {
    let account = read_from_stdin("Account:");
    let side = match read_from_stdin("Buy or Sell?:").to_lowercase().as_ref() {
        "buy" => Ok(Side::Buy),
        "sell" => Ok(Side::Sell),
        _ => Err("Unsupported order side"),
    }?;

    let amount = read_from_stdin("Amount:")
        .parse()
        .map_err(|e: ParseIntError| e.to_string())?;
    let price = read_from_stdin("Price:")
        .parse()
        .map_err(|e: ParseIntError| e.to_string())?;

    Ok(Order {
        price,
        amount,
        side,
        signer: account,
    })
}

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}

async fn send_request<T, U>(
    client: &reqwest::Client,
    path: Url,
    body: Option<T>,
) -> Result<U, CliError>
where
    T: Serialize,
    U: for<'de> Deserialize<'de>,
{
    let response = match body {
        Some(body) => client.post(path).json(&body).send().await?,
        None => client.get(path).send().await?,
    };

    if response.status() == 200 {
        Ok(response.json().await?)
    } else {
        Err(CliError::LogicError(response.text().await?))
    }
}

fn handle_command<T>(res: Result<T, CliError>)
where
    T: std::fmt::Debug,
{
    match res {
        Ok(res) => println!("Operation succesfull:\n{:#?}", res),
        Err(e) => eprintln!("Operation failed: '{:?}'", e),
    }
}
