//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use fc_db::Backend as FrontierBackend;
use fc_rpc::{EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride};
use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit};
use jsonrpsee::RpcModule;
use kories_runtime::{opaque::Block, AccountId, Balance, Hash, Index};
use sc_client_api::backend::{Backend, StateBackend, StorageProvider};
use sc_network::NetworkService;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::BlakeTwo256;
use std::collections::BTreeMap;

// TODO revisit to implement is_dev_signer, filter_pool, max_past_logs
/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// The Node authority flag
	pub is_authority: bool,
	/// Backend.
	pub backend: Arc<FrontierBackend<Block>>,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
}

// TODO revisit C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore, for AuxStore
pub fn overrides_handle<C, BE>(client: Arc<C>) -> Arc<OverrideHandle<Block>>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
	C: Send + Sync + 'static,
	C::Api: sp_api::ApiExt<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	// let mut overrides_map = BTreeMap::new();
	// overrides_map.insert(
	// 	EthereumStorageSchema::V1,
	// 	Box::new(SchemaV1Override::new(client.clone()))
	// 		as Box<dyn StorageOverride<_> + Send + Sync>,
	// );
	// overrides_map.insert(
	// 	EthereumStorageSchema::V2,
	// 	Box::new(SchemaV2Override::new(client.clone()))
	// 		as Box<dyn StorageOverride<_> + Send + Sync>,
	// );
	// overrides_map.insert(
	// 	EthereumStorageSchema::V3,
	// 	Box::new(SchemaV3Override::new(client.clone()))
	// 		as Box<dyn StorageOverride<_> + Send + Sync>,
	// );

	Arc::new(OverrideHandle {
		schemas: BTreeMap::new(),
		fallback: Box::new(RuntimeApiStorageOverride::new(client)),
	})
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, BE, A>(
	deps: FullDeps<C, P, A>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C: StorageProvider<Block, BE>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	P: TransactionPool<Block = Block> + 'static,
	A: ChainApi<Block = Block> + 'static,
{
	use fc_rpc::{Eth, EthApiServer, Net, NetApiServer};
	use pallet_transaction_payment_rpc::{TransactionPaymentApiServer, TransactionPaymentRpc};
	use substrate_frame_rpc_system::{SystemApiServer, SystemRpc};

	let mut module = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		deny_unsafe,
		network,
		graph,
		is_authority,
		backend,
		fee_history_cache,
		fee_history_cache_limit,
		block_data_cache,
		overrides,
	} = deps;

	module.merge(SystemRpc::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPaymentRpc::new(client.clone()).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	module.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph,
			Some(kories_runtime::TransactionConverter),
			network.clone(),
			vec![],
			overrides.clone(),
			backend.clone(),
			// Is authority.
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_cache_limit,
		)
		.into_rpc(),
	)?;

	module.merge(
		Net::new(
			client.clone(),
			network,
			// Whether to format the `peer_count` response as Hex (default) or not.
			true,
		)
		.into_rpc(),
	)?;

	Ok(module)
}
