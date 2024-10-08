use crate::grpc::methods::*;
use crate::proto;
use crate::proto::windexer_server::{Windexer, WindexerServer};
use crate::proto::*;
use anyhow::Result;
use solana_client::rpc_client::RpcClient as SolanaRpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature,
    transaction::Transaction,
};
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedConfirmedTransaction, EncodedTransaction, ParsedInstruction,
    TransactionStatusMeta, UiConfirmedBlock, UiMessage, UiParsedInstruction, UiTransaction,
    UiTransactionStatusMeta,
};
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedConfirmedTransaction, UiTransactionEncoding,
};
use tonic::{transport::Server, Request, Response, Status};

pub struct GrpcServer {
    solana_rpc: SolanaRpcClient,
}

impl GrpcServer {
    pub fn new(solana_rpc_url: &str) -> Self {
        Self {
            solana_rpc: SolanaRpcClient::new_with_commitment(
                solana_rpc_url.to_string(),
                CommitmentConfig::confirmed(),
            ),
        }
    }

    pub async fn run(self, addr: &str) -> Result<()> {
        let addr = addr.parse()?;
        Server::builder()
            .add_service(WindexerServer::new(self))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl Windexer for GrpcServer {
    async fn get_slot(
        &self,
        _request: Request<GetSlotRequest>,
    ) -> Result<Response<GetSlotResponse>, Status> {
        let slot = self
            .solana_rpc
            .get_slot()
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetSlotResponse { slot }))
    }

    async fn get_block(
        &self,
        request: Request<GetBlockRequest>,
    ) -> Result<Response<GetBlockResponse>, Status> {
        let slot = request.into_inner().slot;
        let block = self
            .solana_rpc
            .get_block_with_encoding(slot, UiTransactionEncoding::Json)
            .map_err(|e| Status::internal(e.to_string()))?;

        let proto_block = convert_encoded_confirmed_block(block);

        Ok(Response::new(GetBlockResponse {
            block: Some(proto_block),
        }))
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        let pubkey = Pubkey::from_str(&request.into_inner().pubkey)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let account = self
            .solana_rpc
            .get_account(&pubkey)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetAccountResponse {
            lamports: account.lamports,
            owner: account.owner.to_string(),
            data: account.data,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        }))
    }

    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status> {
        let signature = Signature::from_str(&request.into_inner().signature)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let transaction = self
            .solana_rpc
            .get_transaction(&signature, UiTransactionEncoding::Json)
            .map_err(|e| Status::internal(e.to_string()))?;

        let proto_transaction = convert_encoded_confirmed_transaction(transaction);

        Ok(Response::new(GetTransactionResponse {
            transaction: Some(proto_transaction),
        }))
    }

    async fn send_transaction(
        &self,
        request: Request<SendTransactionRequest>,
    ) -> Result<Response<SendTransactionResponse>, Status> {
        let transaction_data = request.into_inner().transaction;
        let transaction = Transaction::from_bytes(&transaction_data)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let signature = self
            .solana_rpc
            .send_transaction(&transaction)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendTransactionResponse {
            signature: signature.to_string(),
        }))
    }

    async fn get_compressed_account(
        &self,
        request: Request<GetCompressedAccountRequest>,
    ) -> Result<Response<GetCompressedAccountResponse>, Status> {
        let pubkey = Pubkey::from_str(&request.into_inner().pubkey)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let compressed_account = get_compressed_account(&self.solana_rpc, &pubkey)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetCompressedAccountResponse {
            pubkey: compressed_account.pubkey.to_string(),
            lamports: compressed_account.lamports,
            data: compressed_account.data,
            owner: compressed_account.owner.to_string(),
            executable: compressed_account.executable,
            rent_epoch: compressed_account.rent_epoch,
        }))
    }

    async fn get_compressed_balance(
        &self,
        request: Request<GetCompressedBalanceRequest>,
    ) -> Result<Response<GetCompressedBalanceResponse>, Status> {
        let pubkey = Pubkey::from_str(&request.into_inner().pubkey)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let balance = get_compressed_balance(&self.solana_rpc, &pubkey)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetCompressedBalanceResponse { balance }))
    }

    async fn get_compressed_token_account_balance(
        &self,
        request: Request<GetCompressedTokenAccountBalanceRequest>,
    ) -> Result<Response<GetCompressedTokenAccountBalanceResponse>, Status> {
        let pubkey = Pubkey::from_str(&request.into_inner().pubkey)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let balance = get_compressed_token_account_balance(&self.solana_rpc, &pubkey)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetCompressedTokenAccountBalanceResponse {
            amount: balance.amount,
            decimals: balance.decimals as u32,
        }))
    }

    async fn get_compressed_token_accounts_by_owner(
        &self,
        request: Request<GetCompressedTokenAccountsByOwnerRequest>,
    ) -> Result<Response<GetCompressedTokenAccountsByOwnerResponse>, Status> {
        let request = request.into_inner();
        let owner = Pubkey::from_str(&request.owner)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let filter = request.filter.map(|f| match f.filter_type.unwrap() {
            proto::rpc_token_accounts_filter::FilterType::Mint(pubkey) => {
                RpcTokenAccountsFilter::Mint(Pubkey::new_from_array(
                    pubkey.data.try_into().unwrap(),
                ))
            }
            proto::rpc_token_accounts_filter::FilterType::ProgramId(pubkey) => {
                RpcTokenAccountsFilter::ProgramId(Pubkey::new_from_array(
                    pubkey.data.try_into().unwrap(),
                ))
            }
        });

        let accounts = get_compressed_token_accounts_by_owner(&self.solana_rpc, &owner, filter)
            .map_err(|e| Status::internal(e.to_string()))?;

        let proto_accounts: Vec<proto::CompressedTokenAccount> = accounts
            .into_iter()
            .map(|a| proto::CompressedTokenAccount {
                pubkey: a.pubkey.to_string(),
                account: Some(proto::CompressedAccount {
                    pubkey: a.account.pubkey.to_string(),
                    lamports: a.account.lamports,
                    data: a.account.data,
                    owner: a.account.owner.to_string(),
                    executable: a.account.executable,
                    rent_epoch: a.account.rent_epoch,
                }),
                amount: a.amount,
                decimals: a.decimals as u32,
            })
            .collect();

        Ok(Response::new(GetCompressedTokenAccountsByOwnerResponse {
            accounts: proto_accounts,
        }))
    }

    async fn get_transaction_with_compression_info(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionWithCompressionInfoResponse>, Status> {
        let signature = Signature::from_str(&request.into_inner().signature)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let tx_with_compression =
            get_transaction_with_compression_info(&self.solana_rpc, &signature)
                .map_err(|e| Status::internal(e.to_string()))?;

        let proto_tx_with_compression = proto::TransactionWithCompressionInfo {
            transaction: Some(convert_encoded_confirmed_transaction(
                tx_with_compression.transaction,
            )),
            compression_info: Some(proto::CompressionInfo {
                compressed_accounts: tx_with_compression
                    .compression_info
                    .compressed_accounts
                    .into_iter()
                    .map(convert_compressed_account)
                    .collect(),
                decompressed_accounts: tx_with_compression
                    .compression_info
                    .decompressed_accounts
                    .into_iter()
                    .map(convert_compressed_account)
                    .collect(),
            }),
        };

        Ok(Response::new(GetTransactionWithCompressionInfoResponse {
            transaction_with_compression_info: Some(proto_tx_with_compression),
        }))
    }
}

fn convert_encoded_confirmed_block(block: EncodedConfirmedBlock) -> proto::EncodedConfirmedBlock {
    proto::EncodedConfirmedBlock {
        previous_blockhash: block.previous_blockhash,
        blockhash: block.blockhash,
        parent_slot: block.parent_slot,
        transactions: block
            .transactions
            .into_iter()
            .map(convert_encoded_confirmed_transaction)
            .collect(),
        rewards: block
            .rewards
            .unwrap_or_default()
            .into_iter()
            .map(|reward| proto::Reward {
                pubkey: reward.pubkey,
                lamports: reward.lamports,
                post_balance: reward.post_balance,
                reward_type: reward.reward_type.map(|t| t as i32).unwrap_or_default(),
                commission: reward.commission.map(|c| c as u32),
            })
            .collect(),
        block_time: block.block_time,
        block_height: block.block_height,
    }
}

fn convert_encoded_confirmed_transaction(
    transaction: EncodedConfirmedTransaction,
) -> proto::EncodedConfirmedTransaction {
    proto::EncodedConfirmedTransaction {
        transaction: Some(match transaction.transaction {
            EncodedTransaction::Json(ui_transaction) => convert_ui_transaction(ui_transaction),
            EncodedTransaction::Binary(data, encoding) => proto::EncodedTransaction {
                encoded_transaction: Some(proto::encoded_transaction::EncodedTransaction::Binary(
                    proto::BinaryEncodedTransaction {
                        data,
                        encoding: encoding as i32,
                    },
                )),
            },
            EncodedTransaction::LegacyBinary(data) => proto::EncodedTransaction {
                encoded_transaction: Some(
                    proto::encoded_transaction::EncodedTransaction::LegacyBinary(data),
                ),
            },
            EncodedTransaction::Accounts(account_keys) => proto::EncodedTransaction {
                encoded_transaction: Some(
                    proto::encoded_transaction::EncodedTransaction::Accounts(proto::AccountKeys {
                        account_keys,
                    }),
                ),
            },
        }),
        meta: transaction.meta.map(convert_transaction_status_meta),
    }
}

fn convert_ui_transaction(transaction: UiTransaction) -> proto::EncodedTransaction {
    proto::EncodedTransaction {
        encoded_transaction: Some(proto::encoded_transaction::EncodedTransaction::Json(
            proto::JsonEncodedTransaction {
                signatures: transaction.signatures,
                message: Some(convert_ui_message(transaction.message)),
            },
        )),
    }
}

fn convert_ui_message(message: UiMessage) -> proto::UiMessage {
    proto::UiMessage {
        header: Some(proto::MessageHeader {
            num_required_signatures: message.header.num_required_signatures as u32,
            num_readonly_signed_accounts: message.header.num_readonly_signed_accounts as u32,
            num_readonly_unsigned_accounts: message.header.num_readonly_unsigned_accounts as u32,
        }),
        account_keys: message.account_keys,
        recent_blockhash: message.recent_blockhash,
        instructions: message
            .instructions
            .into_iter()
            .map(convert_ui_parsed_instruction)
            .collect(),
    }
}

fn convert_ui_parsed_instruction(instruction: UiParsedInstruction) -> proto::UiInstruction {
    proto::UiInstruction {
        instruction: Some(match instruction {
            UiParsedInstruction::Parsed(parsed) => {
                proto::ui_instruction::Instruction::Parsed(proto::ParsedInstruction {
                    program: parsed.program,
                    program_id: parsed.program_id,
                    parsed: serde_json::to_string(&parsed.parsed).unwrap_or_default(),
                })
            }
            UiParsedInstruction::Compiled(compiled) => {
                proto::ui_instruction::Instruction::Compiled(proto::CompiledInstruction {
                    program_id_index: compiled.program_id_index as u32,
                    accounts: compiled.accounts,
                    data: compiled.data,
                })
            }
        }),
    }
}

fn convert_transaction_status_meta(meta: TransactionStatusMeta) -> proto::TransactionStatusMeta {
    proto::TransactionStatusMeta {
        status: meta.status.map(|status| proto::TransactionStatus {
            status: Some(match status {
                solana_sdk::transaction::Result::Ok(()) => {
                    proto::transaction_status::Status::Ok(())
                }
                solana_sdk::transaction::Result::Err(err) => {
                    proto::transaction_status::Status::Err(err.to_string())
                }
            }),
        }),
        fee: meta.fee,
        pre_balances: meta.pre_balances,
        post_balances: meta.post_balances,
        inner_instructions: meta
            .inner_instructions
            .map(|instructions| proto::InnerInstructions {
                instructions: instructions
                    .into_iter()
                    .map(|inner| proto::InnerInstruction {
                        index: inner.index as u32,
                        instructions: inner
                            .instructions
                            .into_iter()
                            .map(|inst| proto::CompiledInstruction {
                                program_id_index: inst.program_id_index as u32,
                                accounts: inst.accounts,
                                data: inst.data,
                            })
                            .collect(),
                    })
                    .collect(),
            }),
        log_messages: meta.log_messages,
        pre_token_balances: meta
            .pre_token_balances
            .map(|balances| balances.into_iter().map(convert_ui_token_balance).collect()),
        post_token_balances: meta
            .post_token_balances
            .map(|balances| balances.into_iter().map(convert_ui_token_balance).collect()),
        rewards: meta.rewards.map(|rewards| {
            rewards
                .into_iter()
                .map(|reward| proto::Reward {
                    pubkey: reward.pubkey,
                    lamports: reward.lamports,
                    post_balance: reward.post_balance,
                    reward_type: reward.reward_type.map(|t| t as i32).unwrap_or_default(),
                    commission: reward.commission.map(|c| c as u32),
                })
                .collect()
        }),
    }
}

fn convert_ui_token_balance(balance: UiTokenBalance) -> proto::UiTokenBalance {
    proto::UiTokenBalance {
        account_index: balance.account_index as u32,
        mint: balance.mint,
        owner: balance.owner,
        ui_token_amount: Some(proto::UiTokenAmount {
            ui_amount: balance.ui_token_amount.ui_amount,
            decimals: balance.ui_token_amount.decimals as u32,
            amount: balance.ui_token_amount.amount,
            ui_amount_string: balance.ui_token_amount.ui_amount_string,
        }),
    }
}

fn convert_compressed_account(account: CompressedAccount) -> proto::CompressedAccount {
    proto::CompressedAccount {
        pubkey: account.pubkey.to_string(),
        lamports: account.lamports,
        data: account.data,
        owner: account.owner.to_string(),
        executable: account.executable,
        rent_epoch: account.rent_epoch,
    }
}
