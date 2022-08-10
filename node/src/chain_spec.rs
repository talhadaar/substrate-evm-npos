use nfid_runtime::{
	AccountId, BabeConfig, Balance, BalancesConfig, GenesisConfig, GrandpaConfig,
	NodeAuthorizationConfig, SessionConfig, SessionKeys, Signature, StakerStatus, StakingConfig,
	SudoConfig, SystemConfig, DOLLARS, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, OpaquePeerId, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, BabeId, GrandpaId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"NFID [Dev]",
		// ID
		"nfid_test_dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial BABE & GRANDPA validators
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("nfid_test"),
		//Fork ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"NFID Testnet",
		// ID
		"nfid_test",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial BABE & GRANDPA validators
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("nfid_test"),
		//Fork ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), STASH, StakerStatus::Validator))
		.collect::<Vec<_>>();
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(nfid_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						SessionKeys { babe: x.1.clone(), grandpa: x.2.clone() },
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: sp_runtime::Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		evm: Default::default(),
		// evm: EVMConfig {
		// 	accounts: {
		// 		let mut map = BTreeMap::new();
		// 		map.insert(
		// 			// H160 address of Alice dev account
		// 			// Derived from SS58 (42 prefix) address
		// 			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
		// 			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		// 			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
		// 			// chars)
		// 			H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
		// 				.expect("internal H160 is valid; qed"),
		// 			fp_evm::GenesisAccount {
		// 				balance: U256::from(u128::max_value()),
		// 				code: Default::default(),
		// 				nonce: Default::default(),
		// 				storage: Default::default(),
		// 			},
		// 		);
		// 		map.insert(
		// 			// H160 address of Heath dev account
		// 			// Derived from SS58 (42 prefix) address
		// 			// Public Address: 0x931f3600a299fd9B24cEfB3BfF79388D19804BeA
		// 			// Private Key:
		// 			// 0x0d6dcaaef49272a5411896be8ad16c01c35d6f8c18873387b71fbc734759b0ab Using the
		// 			// full hex key, truncating to the first 20 bytes (the first 40 hex chars)
		// 			H160::from_str("931f3600a299fd9B24cEfB3BfF79388D19804BeA")
		// 				.expect("internal H160 is valid; qed"),
		// 			fp_evm::GenesisAccount {
		// 				balance: U256::from(u128::max_value()),
		// 				code: Default::default(),
		// 				nonce: Default::default(),
		// 				storage: Default::default(),
		// 			},
		// 		);
		// 		map.insert(
		// 			// H160 address of Ethen dev account
		// 			// Derived from SS58 (42 prefix) address
		// 			// Public Address: 0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB
		// 			// Private Key:
		// 			// 0x7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4 Using the
		// 			// full hex key, truncating to the first 20 bytes (the first 40 hex chars)
		// 			H160::from_str("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")
		// 				.expect("internal H160 is valid; qed"),
		// 			fp_evm::GenesisAccount {
		// 				balance: U256::default(),
		// 				code: Default::default(),
		// 				nonce: Default::default(),
		// 				storage: Default::default(),
		// 			},
		// 		);
		// 		map.insert(
		// 			// H160 address of CI test runner account
		// 			H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
		// 				.expect("internal H160 is valid; qed"),
		// 			fp_evm::GenesisAccount {
		// 				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
		// 					.expect("internal U256 is valid; qed"),
		// 				code: Default::default(),
		// 				nonce: Default::default(),
		// 				storage: Default::default(),
		// 			},
		// 		);
		// 		map
		// 	},
		// },
		ethereum: Default::default(),
		base_fee: Default::default(),
		dynamic_fee: Default::default(),
		node_authorization: NodeAuthorizationConfig {
			nodes: vec![
				(
					OpaquePeerId(
						bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2")
							.into_vec()
							.unwrap(),
					),
					endowed_accounts[0].clone(), // Alice
				),
				(
					OpaquePeerId(
						bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust")
							.into_vec()
							.unwrap(),
					),
					endowed_accounts[1].clone(), // Bob
				),
			],
		},
	}
}
