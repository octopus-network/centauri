#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 39usize] = [
		"System",
		"Timestamp",
		"Sudo",
		"RandomnessCollectiveFlip",
		"TransactionPayment",
		"Indices",
		"Balances",
		"ParachainSystem",
		"ParachainInfo",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"Council",
		"CouncilMembership",
		"Treasury",
		"Democracy",
		"TechnicalCommittee",
		"TechnicalCommitteeMembership",
		"ReleaseCommittee",
		"ReleaseMembership",
		"Scheduler",
		"Utility",
		"Preimage",
		"Proxy",
		"XcmpQueue",
		"RelayerXcm",
		"CumulusXcm",
		"DmpQueue",
		"XTokens",
		"UnknownTokens",
		"Tokens",
		"CurrencyFactory",
		"CrowdloanRewards",
		"Assets",
		"GovernanceRegistry",
		"CallFilter",
		"Ibc",
	];
	#[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 2)]
		Sudo(sudo::Event),
		#[codec(index = 4)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 5)]
		Indices(indices::Event),
		#[codec(index = 6)]
		Balances(balances::Event),
		#[codec(index = 10)]
		ParachainSystem(parachain_system::Event),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Event),
		#[codec(index = 22)]
		Session(session::Event),
		#[codec(index = 30)]
		Council(council::Event),
		#[codec(index = 31)]
		CouncilMembership(council_membership::Event),
		#[codec(index = 32)]
		Treasury(treasury::Event),
		#[codec(index = 33)]
		Democracy(democracy::Event),
		#[codec(index = 72)]
		TechnicalCommittee(technical_committee::Event),
		#[codec(index = 73)]
		TechnicalCommitteeMembership(technical_committee_membership::Event),
		#[codec(index = 74)]
		ReleaseCommittee(release_committee::Event),
		#[codec(index = 75)]
		ReleaseMembership(release_membership::Event),
		#[codec(index = 34)]
		Scheduler(scheduler::Event),
		#[codec(index = 35)]
		Utility(utility::Event),
		#[codec(index = 36)]
		Preimage(preimage::Event),
		#[codec(index = 37)]
		Proxy(proxy::Event),
		#[codec(index = 40)]
		XcmpQueue(xcmp_queue::Event),
		#[codec(index = 41)]
		RelayerXcm(relayer_xcm::Event),
		#[codec(index = 42)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 43)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 44)]
		XTokens(x_tokens::Event),
		#[codec(index = 45)]
		UnknownTokens(unknown_tokens::Event),
		#[codec(index = 52)]
		Tokens(tokens::Event),
		#[codec(index = 53)]
		CurrencyFactory(currency_factory::Event),
		#[codec(index = 56)]
		CrowdloanRewards(crowdloan_rewards::Event),
		#[codec(index = 58)]
		GovernanceRegistry(governance_registry::Event),
		#[codec(index = 100)]
		CallFilter(call_filter::Event),
		#[codec(index = 190)]
		Ibc(ibc::Event),
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`"]
				#[doc = "# </weight>"]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<Remark> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"remark",
						Remark { remark },
						[
							101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
							147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
							146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
							160u8,
						],
					)
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_heap_pages",
						SetHeapPages { pages },
						[
							43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
							86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
							125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
						],
					)
				}
				#[doc = "Set the new runtime code."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
				#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
				#[doc = "  expensive)."]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
				#[doc = "expensive. We will treat this as a full block."]
				#[doc = "# </weight>"]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCode> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_code",
						SetCode { code },
						[
							27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
							244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
							4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
						],
					)
				}
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C)` where `C` length of `code`"]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
				#[doc = "block. # </weight>"]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_code_without_checks",
						SetCodeWithoutChecks { code },
						[
							102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
							159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
							40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
						],
					)
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_storage",
						SetStorage { items },
						[
							74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
							45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
							140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
						],
					)
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::StaticTxPayload<KillStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_storage",
						KillStorage { keys },
						[
							174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
							85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
							67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
							15u8,
						],
					)
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_prefix",
						KillPrefix { prefix, subkeys },
						[
							203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
							193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
							80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
							48u8,
						],
					)
				}
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"remark_with_event",
						RemarkWithEvent { remark },
						[
							123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
							134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
							136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
							46u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: ::subxt::utils::AccountId32,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						Vec::new(),
						[
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
							222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
							41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::dispatch::PerDispatchClass<
							runtime_types::sp_weights::weight_v2::Weight,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockWeight",
						vec![],
						[
							120u8, 67u8, 71u8, 163u8, 36u8, 202u8, 52u8, 106u8, 143u8, 155u8,
							144u8, 87u8, 142u8, 241u8, 232u8, 183u8, 56u8, 235u8, 27u8, 237u8,
							20u8, 202u8, 33u8, 85u8, 189u8, 0u8, 28u8, 52u8, 198u8, 40u8, 219u8,
							54u8,
						],
					)
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
							254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
							219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
							134u8, 60u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						Vec::new(),
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Number",
						vec![],
						[
							228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
							246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
							36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
							164u8, 191u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ParentHash",
						vec![],
						[
							232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
							169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
							15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
							36u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::generic::digest::Digest,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Digest",
						vec![],
						[
							83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
							31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
							216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_system::EventRecord<
								runtime_types::composable_runtime::RuntimeEvent,
								::subxt::utils::H256,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Events",
						vec![],
						[
							115u8, 90u8, 169u8, 203u8, 173u8, 150u8, 219u8, 69u8, 185u8, 175u8,
							18u8, 4u8, 43u8, 90u8, 81u8, 51u8, 84u8, 187u8, 147u8, 238u8, 11u8,
							110u8, 174u8, 101u8, 0u8, 64u8, 34u8, 113u8, 157u8, 35u8, 130u8, 168u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventCount",
						vec![],
						[
							236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
							203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
							161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
							112u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						Vec::new(),
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::LastRuntimeUpgradeInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
							126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
							185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
							178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
							83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
							210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
							155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
							185u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExecutionPhase",
						vec![],
						[
							230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
							103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
							201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
							172u8, 93u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockWeights,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockWeights",
						[
							118u8, 253u8, 239u8, 217u8, 145u8, 115u8, 85u8, 86u8, 172u8, 248u8,
							139u8, 32u8, 158u8, 126u8, 172u8, 188u8, 197u8, 105u8, 145u8, 235u8,
							171u8, 50u8, 31u8, 225u8, 167u8, 187u8, 241u8, 87u8, 6u8, 17u8, 234u8,
							185u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockLength,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockLength",
						[
							116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
							150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
							12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
							236u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_weights::RuntimeDbWeight>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"DbWeight",
						[
							124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
							199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
							20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
							118u8,
						],
					)
				}
				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"Version",
						[
							93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
							47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
							165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "`MinimumPeriod`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				#[doc = "# </weight>"]
				pub fn set(
					&self,
					now: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<Set> {
					::subxt::tx::StaticTxPayload::new(
						"Timestamp",
						"set",
						Set { now },
						[
							6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
							102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
							214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"Now",
						vec![],
						[
							148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
							221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
							185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
						],
					)
				}
				#[doc = " Did the timestamp get updated in this block?"]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
							13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
							27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKey {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoAs {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Sudo> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo",
						Sudo { call: ::std::boxed::Box::new(call) },
						[
							175u8, 208u8, 38u8, 246u8, 0u8, 174u8, 212u8, 170u8, 151u8, 12u8, 84u8,
							58u8, 66u8, 205u8, 247u8, 136u8, 208u8, 131u8, 115u8, 63u8, 204u8,
							224u8, 27u8, 20u8, 118u8, 27u8, 65u8, 89u8, 157u8, 72u8, 181u8, 173u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- The weight of this call is defined by the caller."]
				#[doc = "# </weight>"]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							223u8, 255u8, 213u8, 245u8, 151u8, 142u8, 196u8, 240u8, 83u8, 84u8,
							45u8, 210u8, 204u8, 233u8, 164u8, 159u8, 15u8, 228u8, 9u8, 72u8, 245u8,
							93u8, 25u8, 166u8, 20u8, 98u8, 2u8, 104u8, 200u8, 199u8, 27u8, 246u8,
						],
					)
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB change."]
				#[doc = "# </weight>"]
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetKey> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"set_key",
						SetKey { new },
						[
							34u8, 116u8, 170u8, 18u8, 106u8, 17u8, 231u8, 159u8, 110u8, 246u8, 2u8,
							27u8, 161u8, 155u8, 163u8, 41u8, 138u8, 7u8, 81u8, 98u8, 230u8, 182u8,
							23u8, 222u8, 240u8, 117u8, 173u8, 232u8, 192u8, 55u8, 92u8, 208u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<SudoAs> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_as",
						SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							185u8, 120u8, 119u8, 120u8, 200u8, 93u8, 167u8, 50u8, 116u8, 228u8,
							187u8, 221u8, 45u8, 101u8, 75u8, 158u8, 13u8, 125u8, 112u8, 175u8,
							187u8, 26u8, 67u8, 111u8, 193u8, 30u8, 148u8, 197u8, 231u8, 43u8,
							122u8, 78u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Sudo",
						"Key",
						vec![],
						[
							244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8,
							34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8,
							80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
						],
					)
				}
			}
		}
	}
	pub mod randomness_collective_flip {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
				#[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
				#[doc = " the oldest hash."]
				pub fn random_material(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RandomnessCollectiveFlip",
						"RandomMaterial",
						vec![],
						[
							152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8, 214u8, 40u8, 159u8,
							30u8, 236u8, 61u8, 240u8, 65u8, 178u8, 94u8, 146u8, 152u8, 135u8,
							252u8, 160u8, 86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8, 217u8,
							242u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::utils::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::fixed_point::FixedU128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
							197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
							237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
							180u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_transaction_payment::Releases,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
							200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
							58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod indices {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Assign an previously unassigned index."]
				#[doc = ""]
				#[doc = "Payment: `Deposit` is reserved from the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be claimed. This must not be in use."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Claim> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"claim",
						Claim { index },
						[
							5u8, 24u8, 11u8, 173u8, 226u8, 170u8, 0u8, 30u8, 193u8, 102u8, 214u8,
							59u8, 252u8, 32u8, 221u8, 88u8, 196u8, 189u8, 244u8, 18u8, 233u8, 37u8,
							228u8, 248u8, 76u8, 175u8, 212u8, 233u8, 238u8, 203u8, 162u8, 68u8,
						],
					)
				}
				#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
				#[doc = "is effectively transferred to the new account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One transfer operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
				#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"transfer",
						Transfer { new, index },
						[
							1u8, 83u8, 197u8, 184u8, 8u8, 96u8, 48u8, 146u8, 116u8, 76u8, 229u8,
							115u8, 226u8, 215u8, 41u8, 154u8, 27u8, 34u8, 205u8, 188u8, 10u8,
							169u8, 203u8, 39u8, 2u8, 236u8, 181u8, 162u8, 115u8, 254u8, 42u8, 28u8,
						],
					)
				}
				#[doc = "Free up an index owned by the sender."]
				#[doc = ""]
				#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
				#[doc = ""]
				#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
				#[doc = ""]
				#[doc = "Emits `IndexFreed` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Free> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"free",
						Free { index },
						[
							133u8, 202u8, 225u8, 127u8, 69u8, 145u8, 43u8, 13u8, 160u8, 248u8,
							215u8, 243u8, 232u8, 166u8, 74u8, 203u8, 235u8, 138u8, 255u8, 27u8,
							163u8, 71u8, 254u8, 217u8, 6u8, 208u8, 202u8, 204u8, 238u8, 70u8,
							126u8, 252u8,
						],
					)
				}
				#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
				#[doc = "held, then any deposit is reimbursed to its current owner."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be (re-)assigned."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
				#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"force_transfer",
						ForceTransfer { new, index, freeze },
						[
							126u8, 8u8, 145u8, 175u8, 177u8, 153u8, 131u8, 123u8, 184u8, 53u8,
							72u8, 207u8, 21u8, 140u8, 87u8, 181u8, 172u8, 64u8, 37u8, 165u8, 121u8,
							111u8, 173u8, 224u8, 181u8, 79u8, 76u8, 134u8, 93u8, 169u8, 65u8,
							131u8,
						],
					)
				}
				#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
				#[doc = "deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
				#[doc = "non-frozen account `index`."]
				#[doc = ""]
				#[doc = "- `index`: the index to be frozen in place."]
				#[doc = ""]
				#[doc = "Emits `IndexFrozen` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one slash operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Freeze> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"freeze",
						Freeze { index },
						[
							121u8, 45u8, 118u8, 2u8, 72u8, 48u8, 38u8, 7u8, 234u8, 204u8, 68u8,
							20u8, 76u8, 251u8, 205u8, 246u8, 149u8, 31u8, 168u8, 186u8, 208u8,
							90u8, 40u8, 47u8, 100u8, 228u8, 188u8, 33u8, 79u8, 220u8, 105u8, 209u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A account index was assigned."]
			pub struct IndexAssigned {
				pub who: ::subxt::utils::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A account index has been freed up (unassigned)."]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A account index has been frozen to its current account ID."]
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IndexFrozen {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFrozen";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The lookup from index to account."]
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
				#[doc = " The lookup from index to account."]
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The deposit needed for reserving an index."]
				pub fn deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Indices",
						"Deposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetBalance {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAll {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnreserve {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
				#[doc = "  types. See related functions below."]
				#[doc = "- It contains a limited number of reads and writes internally and no complex"]
				#[doc = "  computation."]
				#[doc = ""]
				#[doc = "Related functions:"]
				#[doc = ""]
				#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
				#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
				#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
				#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
				#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
				#[doc = "    that the transfer will not kill the origin account."]
				#[doc = "---------------------------------"]
				#[doc = "- Origin account is already in memory, so no DB operations for them."]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer",
						Transfer { dest, value },
						[
							255u8, 181u8, 144u8, 248u8, 64u8, 167u8, 5u8, 134u8, 208u8, 20u8,
							223u8, 103u8, 235u8, 35u8, 66u8, 184u8, 27u8, 94u8, 176u8, 60u8, 233u8,
							236u8, 145u8, 218u8, 44u8, 138u8, 240u8, 224u8, 16u8, 193u8, 220u8,
							95u8,
						],
					)
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
				#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
				#[doc = "If the new free or reserved balance is below the existential deposit,"]
				#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"set_balance",
						SetBalance { who, new_free, new_reserved },
						[
							174u8, 34u8, 80u8, 252u8, 193u8, 51u8, 228u8, 236u8, 234u8, 16u8,
							173u8, 214u8, 122u8, 21u8, 254u8, 7u8, 49u8, 176u8, 18u8, 128u8, 122u8,
							68u8, 72u8, 181u8, 119u8, 90u8, 167u8, 46u8, 203u8, 220u8, 109u8,
							110u8,
						],
					)
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
				#[doc = "specified."]
				#[doc = "# <weight>"]
				#[doc = "- Same as transfer, but additional read and write because the source account is not"]
				#[doc = "  assumed to be in the overlay."]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_transfer",
						ForceTransfer { source, dest, value },
						[
							56u8, 80u8, 186u8, 45u8, 134u8, 147u8, 200u8, 19u8, 53u8, 221u8, 213u8,
							32u8, 13u8, 51u8, 130u8, 42u8, 244u8, 85u8, 50u8, 246u8, 189u8, 51u8,
							93u8, 1u8, 108u8, 142u8, 112u8, 245u8, 104u8, 255u8, 15u8, 62u8,
						],
					)
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
				#[doc = "origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_keep_alive",
						TransferKeepAlive { dest, value },
						[
							202u8, 239u8, 204u8, 0u8, 52u8, 57u8, 158u8, 8u8, 252u8, 178u8, 91u8,
							197u8, 238u8, 186u8, 205u8, 56u8, 217u8, 250u8, 21u8, 44u8, 239u8,
							66u8, 79u8, 99u8, 25u8, 106u8, 70u8, 226u8, 50u8, 255u8, 176u8, 71u8,
						],
					)
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true). # <weight>"]
				#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
				#[doc = "  #</weight>"]
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_all",
						TransferAll { dest, keep_alive },
						[
							118u8, 215u8, 198u8, 243u8, 4u8, 173u8, 108u8, 224u8, 113u8, 203u8,
							149u8, 23u8, 130u8, 176u8, 53u8, 205u8, 112u8, 147u8, 88u8, 167u8,
							197u8, 32u8, 104u8, 117u8, 201u8, 168u8, 144u8, 230u8, 120u8, 29u8,
							122u8, 159u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_unreserve",
						ForceUnreserve { who, amount },
						[
							39u8, 229u8, 111u8, 44u8, 147u8, 80u8, 7u8, 26u8, 185u8, 121u8, 149u8,
							25u8, 151u8, 37u8, 124u8, 46u8, 108u8, 136u8, 167u8, 145u8, 103u8,
							65u8, 33u8, 168u8, 36u8, 214u8, 126u8, 237u8, 180u8, 61u8, 108u8,
							110u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8, 51u8, 226u8, 12u8,
							85u8, 135u8, 41u8, 206u8, 177u8, 238u8, 94u8, 246u8, 184u8, 250u8,
							140u8, 213u8, 91u8, 118u8, 163u8, 111u8, 211u8, 46u8, 204u8, 160u8,
							154u8, 21u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						Vec::new(),
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						Vec::new(),
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open."]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_system {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetValidationData {
				pub data:
					runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoSendUpwardMessage {
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AuthorizeUpgrade {
				pub code_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EnactAuthorizedUpgrade {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::tx::StaticTxPayload<SetValidationData> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"set_validation_data",
						SetValidationData { data },
						[
							200u8, 80u8, 163u8, 177u8, 184u8, 117u8, 61u8, 203u8, 244u8, 214u8,
							106u8, 151u8, 128u8, 131u8, 254u8, 120u8, 254u8, 76u8, 104u8, 39u8,
							215u8, 227u8, 233u8, 254u8, 26u8, 62u8, 17u8, 42u8, 19u8, 127u8, 108u8,
							242u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SudoSendUpwardMessage> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"sudo_send_upward_message",
						SudoSendUpwardMessage { message },
						[
							127u8, 79u8, 45u8, 183u8, 190u8, 205u8, 184u8, 169u8, 255u8, 191u8,
							86u8, 154u8, 134u8, 25u8, 249u8, 63u8, 47u8, 194u8, 108u8, 62u8, 60u8,
							170u8, 81u8, 240u8, 113u8, 48u8, 181u8, 171u8, 95u8, 63u8, 26u8, 222u8,
						],
					)
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<AuthorizeUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"authorize_upgrade",
						AuthorizeUpgrade { code_hash },
						[
							52u8, 152u8, 69u8, 207u8, 143u8, 113u8, 163u8, 11u8, 181u8, 182u8,
							124u8, 101u8, 207u8, 19u8, 59u8, 81u8, 129u8, 29u8, 79u8, 115u8, 90u8,
							83u8, 225u8, 124u8, 21u8, 108u8, 99u8, 194u8, 78u8, 83u8, 252u8, 163u8,
						],
					)
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<EnactAuthorizedUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParachainSystem",
						"enact_authorized_upgrade",
						EnactAuthorizedUpgrade { code },
						[
							43u8, 157u8, 1u8, 230u8, 134u8, 72u8, 230u8, 35u8, 159u8, 13u8, 201u8,
							134u8, 184u8, 94u8, 167u8, 13u8, 108u8, 157u8, 145u8, 166u8, 119u8,
							37u8, 51u8, 121u8, 252u8, 255u8, 48u8, 251u8, 126u8, 152u8, 247u8, 5u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The validation function has been scheduled to apply."]
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "The validation function was applied as of the contained relay chain block number."]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The relay-chain aborted the upgrade process."]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An upgrade has been authorized."]
			pub struct UpgradeAuthorized {
				pub code_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some downward messages have been received and will be processed."]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward messages were processed using the given weight."]
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be applied."]
				#[doc = ""]
				#[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]"]
				#[doc = " which will result the next block process with the new validation code. This concludes the upgrade process."]
				#[doc = ""]
				#[doc = " [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE"]
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"PendingValidationCode",
						vec![],
						[
							162u8, 35u8, 108u8, 76u8, 160u8, 93u8, 215u8, 84u8, 20u8, 249u8, 57u8,
							187u8, 88u8, 161u8, 15u8, 131u8, 213u8, 89u8, 140u8, 20u8, 227u8,
							204u8, 79u8, 176u8, 114u8, 119u8, 8u8, 7u8, 64u8, 15u8, 90u8, 92u8,
						],
					)
				}
				#[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
				#[doc = " consequently the relay-chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
				#[doc = " the value."]
				pub fn new_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"NewValidationCode",
						vec![],
						[
							224u8, 174u8, 53u8, 106u8, 240u8, 49u8, 48u8, 79u8, 219u8, 74u8, 142u8,
							166u8, 92u8, 204u8, 244u8, 200u8, 43u8, 169u8, 177u8, 207u8, 190u8,
							106u8, 180u8, 65u8, 245u8, 131u8, 134u8, 4u8, 53u8, 45u8, 76u8, 3u8,
						],
					)
				}
				#[doc = " The [`PersistedValidationData`] set for this block."]
				#[doc = " This value is expected to be set only once per block and it's never stored"]
				#[doc = " in the trie."]
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::PersistedValidationData<
							::subxt::utils::H256,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ValidationData",
						vec![],
						[
							112u8, 58u8, 240u8, 81u8, 219u8, 110u8, 244u8, 186u8, 251u8, 90u8,
							195u8, 217u8, 229u8, 102u8, 233u8, 24u8, 109u8, 96u8, 219u8, 72u8,
							139u8, 93u8, 58u8, 140u8, 40u8, 110u8, 167u8, 98u8, 199u8, 12u8, 138u8,
							131u8,
						],
					)
				}
				#[doc = " Were the validation data set to notify the relay chain?"]
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"DidSetValidationCode",
						vec![],
						[
							89u8, 83u8, 74u8, 174u8, 234u8, 188u8, 149u8, 78u8, 140u8, 17u8, 92u8,
							165u8, 243u8, 87u8, 59u8, 97u8, 135u8, 81u8, 192u8, 86u8, 193u8, 187u8,
							113u8, 22u8, 108u8, 83u8, 242u8, 208u8, 174u8, 40u8, 49u8, 245u8,
						],
					)
				}
				#[doc = " The relay chain block number associated with the last parachain block."]
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						vec![],
						[
							68u8, 121u8, 6u8, 159u8, 181u8, 94u8, 151u8, 215u8, 225u8, 244u8, 4u8,
							158u8, 216u8, 85u8, 55u8, 228u8, 197u8, 35u8, 200u8, 33u8, 29u8, 182u8,
							17u8, 83u8, 59u8, 63u8, 25u8, 180u8, 132u8, 23u8, 97u8, 252u8,
						],
					)
				}
				#[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
				#[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
				#[doc = " candidate will be invalid."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<
							runtime_types::polkadot_primitives::v2::UpgradeRestriction,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						vec![],
						[
							61u8, 3u8, 26u8, 6u8, 88u8, 114u8, 109u8, 63u8, 7u8, 115u8, 245u8,
							198u8, 73u8, 234u8, 28u8, 228u8, 126u8, 27u8, 151u8, 18u8, 133u8, 54u8,
							144u8, 149u8, 246u8, 43u8, 83u8, 47u8, 77u8, 238u8, 10u8, 196u8,
						],
					)
				}
				#[doc = " The state proof for the last relay parent block."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_trie::storage_proof::StorageProof,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"RelayStateProof",
						vec![],
						[
							35u8, 124u8, 167u8, 221u8, 162u8, 145u8, 158u8, 186u8, 57u8, 154u8,
							225u8, 6u8, 176u8, 13u8, 178u8, 195u8, 209u8, 122u8, 221u8, 26u8,
							155u8, 126u8, 153u8, 246u8, 101u8, 221u8, 61u8, 145u8, 211u8, 236u8,
							48u8, 130u8,
						],
					)
				}
				#[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
				#[doc = " the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"RelevantMessagingState",
						vec![],
						[
							68u8, 241u8, 114u8, 83u8, 200u8, 99u8, 8u8, 244u8, 110u8, 134u8, 106u8,
							153u8, 17u8, 90u8, 184u8, 157u8, 100u8, 140u8, 157u8, 83u8, 25u8,
							166u8, 173u8, 31u8, 221u8, 24u8, 236u8, 85u8, 176u8, 223u8, 237u8,
							65u8,
						],
					)
				}
				#[doc = " The parachain host configuration that was obtained from the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn host_configuration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::AbridgedHostConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HostConfiguration",
						vec![],
						[
							104u8, 200u8, 30u8, 202u8, 119u8, 204u8, 233u8, 20u8, 67u8, 199u8,
							47u8, 166u8, 254u8, 152u8, 10u8, 187u8, 240u8, 255u8, 148u8, 201u8,
							134u8, 41u8, 130u8, 201u8, 112u8, 65u8, 68u8, 103u8, 56u8, 123u8,
							178u8, 113u8,
						],
					)
				}
				#[doc = " The last downward message queue chain head we have observed."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastDmqMqcHead",
						vec![],
						[
							176u8, 255u8, 246u8, 125u8, 36u8, 120u8, 24u8, 44u8, 26u8, 64u8, 236u8,
							210u8, 189u8, 237u8, 50u8, 78u8, 45u8, 139u8, 58u8, 141u8, 112u8,
							253u8, 178u8, 198u8, 87u8, 71u8, 77u8, 248u8, 21u8, 145u8, 187u8, 52u8,
						],
					)
				}
				#[doc = " The message queue chain heads we have observed per each channel incoming channel."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::subxt::utils::KeyedVec<
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						vec![],
						[
							55u8, 179u8, 35u8, 16u8, 173u8, 0u8, 122u8, 179u8, 236u8, 98u8, 9u8,
							112u8, 11u8, 219u8, 241u8, 89u8, 131u8, 198u8, 64u8, 139u8, 103u8,
							158u8, 77u8, 107u8, 83u8, 236u8, 255u8, 208u8, 47u8, 61u8, 219u8,
							240u8,
						],
					)
				}
				#[doc = " Number of downward messages processed in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						vec![],
						[
							48u8, 177u8, 84u8, 228u8, 101u8, 235u8, 181u8, 27u8, 66u8, 55u8, 50u8,
							146u8, 245u8, 223u8, 77u8, 132u8, 178u8, 80u8, 74u8, 90u8, 166u8, 81u8,
							109u8, 25u8, 91u8, 69u8, 5u8, 69u8, 123u8, 197u8, 160u8, 146u8,
						],
					)
				}
				#[doc = " HRMP watermark that was set in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HrmpWatermark",
						vec![],
						[
							189u8, 59u8, 183u8, 195u8, 69u8, 185u8, 241u8, 226u8, 62u8, 204u8,
							230u8, 77u8, 102u8, 75u8, 86u8, 157u8, 249u8, 140u8, 219u8, 72u8, 94u8,
							64u8, 176u8, 72u8, 34u8, 205u8, 114u8, 103u8, 231u8, 233u8, 206u8,
							111u8,
						],
					)
				}
				#[doc = " HRMP messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
								runtime_types::polkadot_parachain::primitives::Id,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"HrmpOutboundMessages",
						vec![],
						[
							74u8, 86u8, 173u8, 248u8, 90u8, 230u8, 71u8, 225u8, 127u8, 164u8,
							221u8, 62u8, 146u8, 13u8, 73u8, 9u8, 98u8, 168u8, 6u8, 14u8, 97u8,
							166u8, 45u8, 70u8, 62u8, 210u8, 9u8, 32u8, 83u8, 18u8, 4u8, 201u8,
						],
					)
				}
				#[doc = " Upward messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn upward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"UpwardMessages",
						vec![],
						[
							129u8, 208u8, 187u8, 36u8, 48u8, 108u8, 135u8, 56u8, 204u8, 60u8,
							100u8, 158u8, 113u8, 238u8, 46u8, 92u8, 228u8, 41u8, 178u8, 177u8,
							208u8, 195u8, 148u8, 149u8, 127u8, 21u8, 93u8, 92u8, 29u8, 115u8, 10u8,
							248u8,
						],
					)
				}
				#[doc = " Upward messages that are still pending and not yet send to the relay chain."]
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"PendingUpwardMessages",
						vec![],
						[
							223u8, 46u8, 224u8, 227u8, 222u8, 119u8, 225u8, 244u8, 59u8, 87u8,
							127u8, 19u8, 217u8, 237u8, 103u8, 61u8, 6u8, 210u8, 107u8, 201u8,
							117u8, 25u8, 85u8, 248u8, 36u8, 231u8, 28u8, 202u8, 41u8, 140u8, 208u8,
							254u8,
						],
					)
				}
				#[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
				#[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						vec![],
						[
							132u8, 61u8, 162u8, 129u8, 251u8, 243u8, 20u8, 144u8, 162u8, 73u8,
							237u8, 51u8, 248u8, 41u8, 127u8, 171u8, 180u8, 79u8, 137u8, 23u8, 66u8,
							134u8, 106u8, 222u8, 182u8, 154u8, 0u8, 145u8, 184u8, 156u8, 36u8,
							97u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						vec![],
						[
							180u8, 90u8, 34u8, 178u8, 1u8, 242u8, 211u8, 97u8, 100u8, 34u8, 39u8,
							42u8, 142u8, 249u8, 236u8, 194u8, 244u8, 164u8, 96u8, 54u8, 98u8, 46u8,
							92u8, 196u8, 185u8, 51u8, 231u8, 234u8, 249u8, 143u8, 244u8, 64u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						vec![],
						[
							90u8, 122u8, 168u8, 240u8, 95u8, 195u8, 160u8, 109u8, 175u8, 170u8,
							227u8, 44u8, 139u8, 176u8, 32u8, 161u8, 57u8, 233u8, 56u8, 55u8, 123u8,
							168u8, 174u8, 96u8, 159u8, 62u8, 186u8, 186u8, 17u8, 70u8, 57u8, 246u8,
						],
					)
				}
				#[doc = " The next authorized upgrade, if there is one."]
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"AuthorizedUpgrade",
						vec![],
						[
							136u8, 238u8, 241u8, 144u8, 252u8, 61u8, 101u8, 171u8, 234u8, 160u8,
							145u8, 210u8, 69u8, 29u8, 204u8, 166u8, 250u8, 101u8, 254u8, 32u8,
							96u8, 197u8, 222u8, 212u8, 50u8, 189u8, 25u8, 7u8, 48u8, 183u8, 234u8,
							95u8,
						],
					)
				}
				#[doc = " A custom head data that should be returned as result of `validate_block`."]
				#[doc = ""]
				#[doc = " See [`Pallet::set_custom_validation_head_data`] for more information."]
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainSystem",
						"CustomValidationHeadData",
						vec![],
						[
							189u8, 150u8, 234u8, 128u8, 111u8, 27u8, 173u8, 92u8, 109u8, 4u8, 98u8,
							103u8, 158u8, 19u8, 16u8, 5u8, 107u8, 135u8, 126u8, 170u8, 62u8, 64u8,
							149u8, 80u8, 33u8, 17u8, 83u8, 22u8, 176u8, 118u8, 26u8, 223u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParachainInfo",
						"ParachainId",
						vec![],
						[
							151u8, 191u8, 241u8, 118u8, 192u8, 47u8, 166u8, 151u8, 217u8, 240u8,
							165u8, 232u8, 51u8, 113u8, 243u8, 1u8, 89u8, 240u8, 11u8, 1u8, 77u8,
							104u8, 12u8, 56u8, 17u8, 135u8, 214u8, 19u8, 114u8, 135u8, 66u8, 76u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetUncles {
				pub new_uncles: ::std::vec::Vec<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Provide a set of uncles."]
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetUncles> {
					::subxt::tx::StaticTxPayload::new(
						"Authorship",
						"set_uncles",
						SetUncles { new_uncles },
						[
							181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8,
							115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8,
							223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8,
							64u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Uncles"]
				pub fn uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_authorship::UncleEntryItem<
								::core::primitive::u32,
								::subxt::utils::H256,
								::subxt::utils::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Uncles",
						vec![],
						[
							193u8, 226u8, 196u8, 151u8, 233u8, 82u8, 60u8, 164u8, 27u8, 156u8,
							231u8, 51u8, 79u8, 134u8, 170u8, 166u8, 71u8, 120u8, 250u8, 255u8,
							52u8, 168u8, 74u8, 199u8, 122u8, 253u8, 248u8, 178u8, 39u8, 233u8,
							132u8, 67u8,
						],
					)
				}
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Author",
						vec![],
						[
							149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
							25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
							181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
							113u8,
						],
					)
				}
				#[doc = " Whether uncles were already set in this block."]
				pub fn did_set_uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"DidSetUncles",
						vec![],
						[
							64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
							126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
							83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks back we should accept uncles."]
				#[doc = " This means that we will deal with uncle-parents that are"]
				#[doc = " `UncleGenerations + 1` before `now`."]
				pub fn uncle_generations(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Authorship",
						"UncleGenerations",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod collator_selection {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetInvulnerables {
				pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDesiredCandidates {
				pub max: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetCandidacyBond {
				pub bond: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RegisterAsCandidate;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LeaveIntent;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the list of invulnerable (fixed) collators."]
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<SetInvulnerables> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_invulnerables",
						SetInvulnerables { new },
						[
							120u8, 177u8, 166u8, 239u8, 2u8, 102u8, 76u8, 143u8, 218u8, 130u8,
							168u8, 152u8, 200u8, 107u8, 221u8, 30u8, 252u8, 18u8, 108u8, 147u8,
							81u8, 251u8, 183u8, 185u8, 0u8, 184u8, 100u8, 251u8, 95u8, 168u8, 26u8,
							142u8,
						],
					)
				}
				#[doc = "Set the ideal number of collators (not including the invulnerables)."]
				#[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
				#[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDesiredCandidates> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_desired_candidates",
						SetDesiredCandidates { max },
						[
							181u8, 32u8, 138u8, 37u8, 254u8, 213u8, 197u8, 224u8, 82u8, 26u8, 3u8,
							113u8, 11u8, 146u8, 251u8, 35u8, 250u8, 202u8, 209u8, 2u8, 231u8,
							176u8, 216u8, 124u8, 125u8, 43u8, 52u8, 126u8, 150u8, 140u8, 20u8,
							113u8,
						],
					)
				}
				#[doc = "Set the candidacy bond amount."]
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetCandidacyBond> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"set_candidacy_bond",
						SetCandidacyBond { bond },
						[
							42u8, 173u8, 79u8, 226u8, 224u8, 202u8, 70u8, 185u8, 125u8, 17u8,
							123u8, 99u8, 107u8, 163u8, 67u8, 75u8, 110u8, 65u8, 248u8, 179u8, 39u8,
							177u8, 135u8, 186u8, 66u8, 237u8, 30u8, 73u8, 163u8, 98u8, 81u8, 152u8,
						],
					)
				}
				#[doc = "Register this account as a collator candidate. The account must (a) already have"]
				#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::tx::StaticTxPayload<RegisterAsCandidate> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"register_as_candidate",
						RegisterAsCandidate {},
						[
							63u8, 11u8, 114u8, 142u8, 89u8, 78u8, 120u8, 214u8, 22u8, 215u8, 125u8,
							60u8, 203u8, 89u8, 141u8, 126u8, 124u8, 167u8, 70u8, 240u8, 85u8,
							253u8, 34u8, 245u8, 67u8, 46u8, 240u8, 195u8, 57u8, 81u8, 138u8, 69u8,
						],
					)
				}
				#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
				#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
				#[doc = ""]
				#[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn leave_intent(&self) -> ::subxt::tx::StaticTxPayload<LeaveIntent> {
					::subxt::tx::StaticTxPayload::new(
						"CollatorSelection",
						"leave_intent",
						LeaveIntent {},
						[
							217u8, 3u8, 35u8, 71u8, 152u8, 203u8, 203u8, 212u8, 25u8, 113u8, 158u8,
							124u8, 161u8, 154u8, 32u8, 47u8, 116u8, 134u8, 11u8, 201u8, 154u8,
							40u8, 138u8, 163u8, 184u8, 188u8, 33u8, 237u8, 219u8, 40u8, 63u8,
							221u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct NewInvulnerables {
				pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct NewDesiredCandidates {
				pub desired_candidates: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CandidateAdded {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CandidateRemoved {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The invulnerable, fixed collators."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"Invulnerables",
						vec![],
						[
							215u8, 62u8, 140u8, 81u8, 0u8, 189u8, 182u8, 139u8, 32u8, 42u8, 20u8,
							223u8, 81u8, 212u8, 100u8, 97u8, 146u8, 253u8, 75u8, 123u8, 240u8,
							125u8, 249u8, 62u8, 226u8, 70u8, 57u8, 206u8, 16u8, 74u8, 52u8, 72u8,
						],
					)
				}
				#[doc = " The (community, limited) collation candidates."]
				pub fn candidates(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_collator_selection::pallet::CandidateInfo<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"Candidates",
						vec![],
						[
							28u8, 116u8, 232u8, 94u8, 147u8, 216u8, 214u8, 30u8, 26u8, 241u8, 68u8,
							108u8, 165u8, 107u8, 89u8, 136u8, 111u8, 239u8, 150u8, 42u8, 210u8,
							214u8, 192u8, 234u8, 29u8, 41u8, 157u8, 169u8, 120u8, 126u8, 192u8,
							32u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"LastAuthoredBlock",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"LastAuthoredBlock",
						Vec::new(),
						[
							53u8, 30u8, 243u8, 31u8, 228u8, 231u8, 175u8, 153u8, 204u8, 241u8,
							76u8, 147u8, 6u8, 202u8, 255u8, 89u8, 30u8, 129u8, 85u8, 92u8, 10u8,
							97u8, 177u8, 129u8, 88u8, 196u8, 7u8, 255u8, 74u8, 52u8, 28u8, 0u8,
						],
					)
				}
				#[doc = " Desired number of candidates."]
				#[doc = ""]
				#[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
				pub fn desired_candidates(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"DesiredCandidates",
						vec![],
						[
							161u8, 170u8, 254u8, 76u8, 112u8, 146u8, 144u8, 7u8, 177u8, 152u8,
							146u8, 60u8, 143u8, 237u8, 1u8, 168u8, 176u8, 33u8, 103u8, 35u8, 39u8,
							233u8, 107u8, 253u8, 47u8, 183u8, 11u8, 86u8, 230u8, 13u8, 127u8,
							133u8,
						],
					)
				}
				#[doc = " Fixed amount to deposit to become a collator."]
				#[doc = ""]
				#[doc = " When a collator calls `leave_intent` they immediately receive the deposit back."]
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CollatorSelection",
						"CandidacyBond",
						vec![],
						[
							1u8, 153u8, 211u8, 74u8, 138u8, 178u8, 81u8, 9u8, 205u8, 117u8, 102u8,
							182u8, 56u8, 184u8, 56u8, 62u8, 193u8, 82u8, 224u8, 218u8, 253u8,
							194u8, 250u8, 55u8, 220u8, 107u8, 157u8, 175u8, 62u8, 35u8, 224u8,
							183u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKeys {
				pub keys: runtime_types::composable_runtime::opaque::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PurgeKeys;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
				#[doc = "- DbWrites: `origin account`, `NextKeys`"]
				#[doc = "- DbReads per key id: `KeyOwner`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn set_keys(
					&self,
					keys: runtime_types::composable_runtime::opaque::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"set_keys",
						SetKeys { keys, proof },
						[
							199u8, 56u8, 39u8, 236u8, 44u8, 88u8, 207u8, 0u8, 187u8, 195u8, 218u8,
							94u8, 126u8, 128u8, 37u8, 162u8, 216u8, 223u8, 36u8, 165u8, 18u8, 37u8,
							16u8, 72u8, 136u8, 28u8, 134u8, 230u8, 231u8, 48u8, 230u8, 122u8,
						],
					)
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
				#[doc = "  of `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
				#[doc = "- DbWrites: `NextKeys`, `origin account`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"purge_keys",
						PurgeKeys {},
						[
							200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
							35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
							71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"Validators",
						vec![],
						[
							144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
							242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
							32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
							68u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"CurrentIndex",
						vec![],
						[
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedChanged",
						vec![],
						[
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::composable_runtime::opaque::SessionKeys,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedKeys",
						vec![],
						[
							42u8, 134u8, 252u8, 233u8, 29u8, 69u8, 168u8, 107u8, 77u8, 70u8, 80u8,
							189u8, 149u8, 227u8, 77u8, 74u8, 100u8, 175u8, 10u8, 162u8, 145u8,
							105u8, 85u8, 196u8, 169u8, 195u8, 116u8, 255u8, 112u8, 122u8, 112u8,
							133u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"DisabledValidators",
						vec![],
						[
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::opaque::SessionKeys,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							21u8, 0u8, 237u8, 42u8, 156u8, 77u8, 229u8, 211u8, 105u8, 8u8, 231u8,
							5u8, 246u8, 188u8, 69u8, 143u8, 202u8, 240u8, 252u8, 253u8, 106u8,
							37u8, 51u8, 244u8, 206u8, 199u8, 249u8, 37u8, 17u8, 102u8, 20u8, 246u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::opaque::SessionKeys,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							21u8, 0u8, 237u8, 42u8, 156u8, 77u8, 229u8, 211u8, 105u8, 8u8, 231u8,
							5u8, 246u8, 188u8, 69u8, 143u8, 202u8, 240u8, 252u8, 253u8, 106u8,
							37u8, 51u8, 244u8, 206u8, 199u8, 249u8, 37u8, 17u8, 102u8, 20u8, 246u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod aura {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current authority set."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Aura",
						"Authorities",
						vec![],
						[
							199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
							85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
							217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
							41u8,
						],
					)
				}
				#[doc = " The current slot of this block."]
				#[doc = ""]
				#[doc = " This will be set in `on_initialize`."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Aura",
						"CurrentSlot",
						vec![],
						[
							139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
							204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
							208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
							108u8,
						],
					)
				}
			}
		}
	}
	pub mod aura_ext {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Serves as cache for the authorities."]
				#[doc = ""]
				#[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
				#[doc = " but we require the old authorities to verify the seal when validating a PoV. This will always"]
				#[doc = " be updated to the latest AuRa authorities in `on_finalize`."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuraExt",
						"Authorities",
						vec![],
						[
							199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
							85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
							217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
							41u8,
						],
					)
				}
			}
		}
	}
	pub mod council {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							4u8, 170u8, 24u8, 7u8, 213u8, 181u8, 38u8, 176u8, 151u8, 116u8, 198u8,
							178u8, 209u8, 162u8, 117u8, 8u8, 7u8, 119u8, 246u8, 124u8, 101u8, 55u8,
							113u8, 66u8, 184u8, 168u8, 91u8, 110u8, 34u8, 141u8, 14u8, 93u8,
						],
					)
				}
				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							39u8, 80u8, 18u8, 245u8, 43u8, 154u8, 56u8, 17u8, 5u8, 187u8, 243u8,
							133u8, 153u8, 4u8, 36u8, 125u8, 137u8, 173u8, 122u8, 123u8, 210u8,
							154u8, 235u8, 254u8, 117u8, 59u8, 144u8, 46u8, 149u8, 32u8, 255u8,
							163u8,
						],
					)
				}
				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CloseOldWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"Council",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"Proposals",
						vec![],
						[
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"ProposalOf",
						Vec::new(),
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"Voting",
						Vec::new(),
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Council",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod council_membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChangeKey {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
					::subxt::tx::StaticTxPayload::new(
						"CouncilMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CouncilMembership",
						"Members",
						vec![],
						[
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CouncilMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod treasury {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ProposeSpend {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RejectProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ApproveProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Spend {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveApproval {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
				#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
				#[doc = "proposal is awarded."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `ProposalCount`, `origin account`"]
				#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
				#[doc = "# </weight>"]
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ProposeSpend> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"propose_spend",
						ProposeSpend { value, beneficiary },
						[
							124u8, 32u8, 83u8, 127u8, 240u8, 169u8, 3u8, 190u8, 235u8, 163u8, 23u8,
							29u8, 88u8, 242u8, 238u8, 187u8, 136u8, 75u8, 193u8, 192u8, 239u8, 2u8,
							54u8, 238u8, 147u8, 42u8, 91u8, 14u8, 244u8, 175u8, 41u8, 14u8,
						],
					)
				}
				#[doc = "Reject a proposed spend. The original deposit will be slashed."]
				#[doc = ""]
				#[doc = "May only be called from `T::RejectOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
				#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
				#[doc = "# </weight>"]
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RejectProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"reject_proposal",
						RejectProposal { proposal_id },
						[
							106u8, 223u8, 97u8, 22u8, 111u8, 208u8, 128u8, 26u8, 198u8, 140u8,
							118u8, 126u8, 187u8, 51u8, 193u8, 50u8, 193u8, 68u8, 143u8, 144u8,
							34u8, 132u8, 44u8, 244u8, 105u8, 186u8, 223u8, 234u8, 17u8, 145u8,
							209u8, 145u8,
						],
					)
				}
				#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
				#[doc = "and the original deposit will be returned."]
				#[doc = ""]
				#[doc = "May only be called from `T::ApproveOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)."]
				#[doc = "- DbReads: `Proposals`, `Approvals`"]
				#[doc = "- DbWrite: `Approvals`"]
				#[doc = "# </weight>"]
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ApproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"approve_proposal",
						ApproveProposal { proposal_id },
						[
							164u8, 229u8, 172u8, 98u8, 129u8, 62u8, 84u8, 128u8, 47u8, 108u8, 33u8,
							120u8, 89u8, 79u8, 57u8, 121u8, 4u8, 197u8, 170u8, 153u8, 156u8, 17u8,
							59u8, 164u8, 123u8, 227u8, 175u8, 195u8, 220u8, 160u8, 60u8, 186u8,
						],
					)
				}
				#[doc = "Propose and approve a spend of treasury funds."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`."]
				#[doc = "- `amount`: The amount to be transferred from the treasury to the `beneficiary`."]
				#[doc = "- `beneficiary`: The destination account for the transfer."]
				#[doc = ""]
				#[doc = "NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the"]
				#[doc = "beneficiary."]
				pub fn spend(
					&self,
					amount: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Spend> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"spend",
						Spend { amount, beneficiary },
						[
							208u8, 79u8, 96u8, 218u8, 205u8, 209u8, 165u8, 119u8, 92u8, 208u8,
							54u8, 168u8, 83u8, 190u8, 98u8, 97u8, 6u8, 2u8, 35u8, 249u8, 18u8,
							88u8, 193u8, 51u8, 130u8, 33u8, 28u8, 99u8, 49u8, 194u8, 34u8, 77u8,
						],
					)
				}
				#[doc = "Force a previously approved proposal to be removed from the approval queue."]
				#[doc = "The original deposit will no longer be returned."]
				#[doc = ""]
				#[doc = "May only be called from `T::RejectOrigin`."]
				#[doc = "- `proposal_id`: The index of a proposal"]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(A) where `A` is the number of approvals"]
				#[doc = "- Db reads and writes: `Approvals`"]
				#[doc = "# </weight>"]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,"]
				#[doc = "i.e., the proposal has not been approved. This could also mean the proposal does not"]
				#[doc = "exist altogether, thus there is no way it would have been approved in the first place."]
				pub fn remove_approval(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveApproval> {
					::subxt::tx::StaticTxPayload::new(
						"Treasury",
						"remove_approval",
						RemoveApproval { proposal_id },
						[
							133u8, 126u8, 181u8, 47u8, 196u8, 243u8, 7u8, 46u8, 25u8, 251u8, 154u8,
							125u8, 217u8, 77u8, 54u8, 245u8, 240u8, 180u8, 97u8, 34u8, 186u8, 53u8,
							225u8, 144u8, 155u8, 107u8, 172u8, 54u8, 250u8, 184u8, 178u8, 86u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "New proposal."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "We have ended a spend period and will now allocate funds."]
			pub struct Spending {
				pub budget_remaining: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some funds have been allocated."]
			pub struct Awarded {
				pub proposal_index: ::core::primitive::u32,
				pub award: ::core::primitive::u128,
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal was rejected; funds were slashed."]
			pub struct Rejected {
				pub proposal_index: ::core::primitive::u32,
				pub slashed: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some of our funds have been burnt."]
			pub struct Burnt {
				pub burnt_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
			pub struct Rollover {
				pub rollover_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some funds have been deposited."]
			pub struct Deposit {
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new spend proposal has been approved."]
			pub struct SpendApproved {
				pub proposal_index: ::core::primitive::u32,
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for SpendApproved {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "SpendApproved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of proposals that have been made."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				#[doc = " Proposals that have been made."]
				pub fn proposals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_treasury::Proposal<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Proposals",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
						],
					)
				}
				#[doc = " Proposals that have been made."]
				pub fn proposals_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_treasury::Proposal<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Proposals",
						Vec::new(),
						[
							62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
							113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
							49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
						],
					)
				}
				#[doc = " The amount which has been reported as inactive to Currency."]
				pub fn inactive(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Inactive",
						vec![],
						[
							240u8, 100u8, 242u8, 40u8, 169u8, 252u8, 255u8, 248u8, 66u8, 157u8,
							165u8, 206u8, 229u8, 145u8, 80u8, 73u8, 237u8, 44u8, 72u8, 76u8, 101u8,
							215u8, 87u8, 33u8, 252u8, 224u8, 54u8, 138u8, 79u8, 99u8, 225u8, 34u8,
						],
					)
				}
				#[doc = " Proposal indices that have been approved but not yet awarded."]
				pub fn approvals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Treasury",
						"Approvals",
						vec![],
						[
							202u8, 106u8, 189u8, 40u8, 127u8, 172u8, 108u8, 50u8, 193u8, 4u8,
							248u8, 226u8, 176u8, 101u8, 212u8, 222u8, 64u8, 206u8, 244u8, 175u8,
							111u8, 106u8, 86u8, 96u8, 19u8, 109u8, 218u8, 152u8, 30u8, 59u8, 96u8,
							1u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Fraction of a proposal's value that should be bonded in order to place the proposal."]
				#[doc = " An accepted proposal gets these back. A rejected proposal does not."]
				pub fn proposal_bond(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"ProposalBond",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}
				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_minimum(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"ProposalBondMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_maximum(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<::core::primitive::u128>,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"ProposalBondMaximum",
						[
							84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
							205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
							204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
							216u8,
						],
					)
				}
				#[doc = " Period between successive spends."]
				pub fn spend_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"SpendPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Percentage of spare funds (if any) that are burnt per spend period."]
				pub fn burn(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Permill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"Burn",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}
				#[doc = " The treasury's pallet id, used for deriving its sovereign account ID."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"PalletId",
						[
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
						],
					)
				}
				#[doc = " The maximum number of approvals that can wait in the spending queue."]
				#[doc = ""]
				#[doc = " NOTE: This parameter is also used within the Bounties Pallet extension if enabled."]
				pub fn max_approvals(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Treasury",
						"MaxApprovals",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod democracy {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Propose {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Second {
				#[codec(compact)]
				pub proposal: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Vote {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct EmergencyCancel {
				pub ref_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ExternalPropose {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ExternalProposeMajority {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ExternalProposeDefault {
				pub proposal: runtime_types::frame_support::traits::preimages::Bounded<
					runtime_types::composable_runtime::RuntimeCall,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FastTrack {
				pub proposal_hash: ::subxt::utils::H256,
				pub voting_period: ::core::primitive::u32,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct VetoExternal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelReferendum {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Delegate {
				pub to: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
				pub balance: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Undelegate;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearPublicProposals;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Unlock {
				pub target: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct RemoveVote {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveOtherVote {
				pub target: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Blacklist {
				pub proposal_hash: ::subxt::utils::H256,
				pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelProposal {
				#[codec(compact)]
				pub prop_index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Propose a sensitive action to be taken."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
				#[doc = "have funds to cover the deposit."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
				#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
				#[doc = ""]
				#[doc = "Emits `Proposed`."]
				pub fn propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"propose",
						Propose { proposal, value },
						[
							123u8, 3u8, 204u8, 140u8, 194u8, 195u8, 214u8, 39u8, 167u8, 126u8,
							45u8, 4u8, 219u8, 17u8, 143u8, 185u8, 29u8, 224u8, 230u8, 68u8, 253u8,
							15u8, 170u8, 90u8, 232u8, 123u8, 46u8, 255u8, 168u8, 39u8, 204u8, 63u8,
						],
					)
				}
				#[doc = "Signals agreement with a particular proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
				#[doc = "must have funds to cover the deposit, equal to the original deposit."]
				#[doc = ""]
				#[doc = "- `proposal`: The index of the proposal to second."]
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Second> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"second",
						Second { proposal },
						[
							59u8, 240u8, 183u8, 218u8, 61u8, 93u8, 184u8, 67u8, 10u8, 4u8, 138u8,
							196u8, 168u8, 49u8, 42u8, 69u8, 154u8, 42u8, 90u8, 112u8, 179u8, 69u8,
							51u8, 148u8, 159u8, 212u8, 221u8, 226u8, 132u8, 228u8, 51u8, 83u8,
						],
					)
				}
				#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
				#[doc = "otherwise it is a vote to keep the status quo."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to vote for."]
				#[doc = "- `vote`: The vote configuration."]
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"vote",
						Vote { ref_index, vote },
						[
							138u8, 213u8, 229u8, 111u8, 1u8, 191u8, 73u8, 3u8, 145u8, 28u8, 44u8,
							88u8, 163u8, 188u8, 129u8, 188u8, 64u8, 15u8, 64u8, 103u8, 250u8, 97u8,
							234u8, 188u8, 29u8, 205u8, 51u8, 6u8, 116u8, 58u8, 156u8, 201u8,
						],
					)
				}
				#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
				#[doc = ""]
				#[doc = "-`ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<EmergencyCancel> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"emergency_cancel",
						EmergencyCancel { ref_index },
						[
							139u8, 213u8, 133u8, 75u8, 34u8, 206u8, 124u8, 245u8, 35u8, 237u8,
							132u8, 92u8, 49u8, 167u8, 117u8, 80u8, 188u8, 93u8, 198u8, 237u8,
							132u8, 77u8, 195u8, 65u8, 29u8, 37u8, 86u8, 74u8, 214u8, 119u8, 71u8,
							204u8,
						],
					)
				}
				#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				pub fn external_propose(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::StaticTxPayload<ExternalPropose> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose",
						ExternalPropose { proposal },
						[
							164u8, 193u8, 14u8, 122u8, 105u8, 232u8, 20u8, 194u8, 99u8, 227u8,
							36u8, 105u8, 218u8, 146u8, 16u8, 208u8, 56u8, 62u8, 100u8, 65u8, 35u8,
							33u8, 51u8, 208u8, 17u8, 43u8, 223u8, 198u8, 202u8, 16u8, 56u8, 75u8,
						],
					)
				}
				#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
				#[doc = "an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_majority(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::StaticTxPayload<ExternalProposeMajority> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose_majority",
						ExternalProposeMajority { proposal },
						[
							129u8, 124u8, 147u8, 253u8, 69u8, 115u8, 230u8, 186u8, 155u8, 4u8,
							220u8, 103u8, 28u8, 132u8, 115u8, 153u8, 196u8, 88u8, 9u8, 130u8,
							129u8, 234u8, 75u8, 96u8, 202u8, 216u8, 145u8, 189u8, 231u8, 101u8,
							127u8, 11u8,
						],
					)
				}
				#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
				#[doc = "schedule an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_default(
					&self,
					proposal: runtime_types::frame_support::traits::preimages::Bounded<
						runtime_types::composable_runtime::RuntimeCall,
					>,
				) -> ::subxt::tx::StaticTxPayload<ExternalProposeDefault> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"external_propose_default",
						ExternalProposeDefault { proposal },
						[
							96u8, 15u8, 108u8, 208u8, 141u8, 247u8, 4u8, 73u8, 2u8, 30u8, 231u8,
							40u8, 184u8, 250u8, 42u8, 161u8, 248u8, 45u8, 217u8, 50u8, 53u8, 13u8,
							181u8, 214u8, 136u8, 51u8, 93u8, 95u8, 165u8, 3u8, 83u8, 190u8,
						],
					)
				}
				#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
				#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
				#[doc = "but it is not a majority-carries referendum then it fails."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the current external proposal."]
				#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
				#[doc = "\tMust be always greater than zero."]
				#[doc = "\tFor `FastTrackOrigin` must be equal or greater than `FastTrackVotingPeriod`."]
				#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
				#[doc = "  enacted. This doesn't have a minimum amount."]
				#[doc = ""]
				#[doc = "Emits `Started`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn fast_track(
					&self,
					proposal_hash: ::subxt::utils::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<FastTrack> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"fast_track",
						FastTrack { proposal_hash, voting_period, delay },
						[
							125u8, 209u8, 107u8, 120u8, 93u8, 205u8, 129u8, 147u8, 254u8, 126u8,
							45u8, 126u8, 39u8, 0u8, 56u8, 14u8, 233u8, 49u8, 245u8, 220u8, 156u8,
							10u8, 252u8, 31u8, 102u8, 90u8, 163u8, 236u8, 178u8, 85u8, 13u8, 24u8,
						],
					)
				}
				#[doc = "Veto and blacklist the external proposal hash."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
				#[doc = ""]
				#[doc = "Emits `Vetoed`."]
				#[doc = ""]
				#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
				pub fn veto_external(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<VetoExternal> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"veto_external",
						VetoExternal { proposal_hash },
						[
							209u8, 18u8, 18u8, 103u8, 186u8, 160u8, 214u8, 124u8, 150u8, 207u8,
							112u8, 90u8, 84u8, 197u8, 95u8, 157u8, 165u8, 65u8, 109u8, 101u8, 75u8,
							201u8, 41u8, 149u8, 75u8, 154u8, 37u8, 178u8, 239u8, 121u8, 124u8,
							23u8,
						],
					)
				}
				#[doc = "Remove a referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "# Weight: `O(1)`."]
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelReferendum> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"cancel_referendum",
						CancelReferendum { ref_index },
						[
							51u8, 25u8, 25u8, 251u8, 236u8, 115u8, 130u8, 230u8, 72u8, 186u8,
							119u8, 71u8, 165u8, 137u8, 55u8, 167u8, 187u8, 128u8, 55u8, 8u8, 212u8,
							139u8, 245u8, 232u8, 103u8, 136u8, 229u8, 113u8, 125u8, 36u8, 1u8,
							149u8,
						],
					)
				}
				#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
				#[doc = ""]
				#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
				#[doc = "time appropriate for the conviction's lock period."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
				#[doc = "  - be delegating already; or"]
				#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
				#[doc = "    through `reap_vote` or `unvote`)."]
				#[doc = ""]
				#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
				#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
				#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
				#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
				#[doc = "  be more than the account's current balance."]
				#[doc = ""]
				#[doc = "Emits `Delegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn delegate(
					&self,
					to: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Delegate> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"delegate",
						Delegate { to, conviction, balance },
						[
							247u8, 226u8, 242u8, 221u8, 47u8, 161u8, 91u8, 223u8, 6u8, 79u8, 238u8,
							205u8, 41u8, 188u8, 140u8, 56u8, 181u8, 248u8, 102u8, 10u8, 127u8,
							166u8, 90u8, 187u8, 13u8, 124u8, 209u8, 117u8, 16u8, 209u8, 74u8, 29u8,
						],
					)
				}
				#[doc = "Undelegate the voting power of the sending account."]
				#[doc = ""]
				#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
				#[doc = "of the conviction with which the delegation was issued."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
				#[doc = "currently delegating."]
				#[doc = ""]
				#[doc = "Emits `Undelegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn undelegate(&self) -> ::subxt::tx::StaticTxPayload<Undelegate> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"undelegate",
						Undelegate {},
						[
							165u8, 40u8, 183u8, 209u8, 57u8, 153u8, 111u8, 29u8, 114u8, 109u8,
							107u8, 235u8, 97u8, 61u8, 53u8, 155u8, 44u8, 245u8, 28u8, 220u8, 56u8,
							134u8, 43u8, 122u8, 248u8, 156u8, 191u8, 154u8, 4u8, 121u8, 152u8,
							153u8,
						],
					)
				}
				#[doc = "Clears all public proposals."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn clear_public_proposals(
					&self,
				) -> ::subxt::tx::StaticTxPayload<ClearPublicProposals> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"clear_public_proposals",
						ClearPublicProposals {},
						[
							59u8, 126u8, 254u8, 223u8, 252u8, 225u8, 75u8, 185u8, 188u8, 181u8,
							42u8, 179u8, 211u8, 73u8, 12u8, 141u8, 243u8, 197u8, 46u8, 130u8,
							215u8, 196u8, 225u8, 88u8, 48u8, 199u8, 231u8, 249u8, 195u8, 53u8,
							184u8, 204u8,
						],
					)
				}
				#[doc = "Unlock tokens that have an expired lock."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account to remove the lock on."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` with R number of vote of target."]
				pub fn unlock(
					&self,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<Unlock> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"unlock",
						Unlock { target },
						[
							227u8, 6u8, 154u8, 150u8, 253u8, 167u8, 142u8, 6u8, 147u8, 24u8, 124u8,
							51u8, 101u8, 185u8, 184u8, 170u8, 6u8, 223u8, 29u8, 167u8, 73u8, 31u8,
							179u8, 60u8, 156u8, 244u8, 192u8, 233u8, 79u8, 99u8, 248u8, 126u8,
						],
					)
				}
				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If:"]
				#[doc = "- the referendum was cancelled, or"]
				#[doc = "- the referendum is ongoing, or"]
				#[doc = "- the referendum has ended such that"]
				#[doc = "  - the vote of the account was in opposition to the result; or"]
				#[doc = "  - there was no conviction to the account's vote; or"]
				#[doc = "  - the account made a split vote"]
				#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
				#[doc = "funds being available."]
				#[doc = ""]
				#[doc = "If, however, the referendum has ended and:"]
				#[doc = "- it finished corresponding to the vote of the account, and"]
				#[doc = "- the account made a standard vote with conviction, and"]
				#[doc = "- the lock period of the conviction is not over"]
				#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
				#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
				#[doc = "of both the amount locked and the time is it locked for)."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
				#[doc = "registered for referendum `index`."]
				#[doc = ""]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveVote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"remove_vote",
						RemoveVote { index },
						[
							148u8, 120u8, 14u8, 172u8, 81u8, 152u8, 159u8, 178u8, 106u8, 244u8,
							36u8, 98u8, 120u8, 189u8, 213u8, 93u8, 119u8, 156u8, 112u8, 34u8,
							241u8, 72u8, 206u8, 113u8, 212u8, 161u8, 164u8, 126u8, 122u8, 82u8,
							160u8, 74u8,
						],
					)
				}
				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
				#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
				#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
				#[doc = "because the conviction period is over."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
				#[doc = "  referendum `index`."]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_other_vote(
					&self,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveOtherVote> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"remove_other_vote",
						RemoveOtherVote { target, index },
						[
							251u8, 245u8, 79u8, 229u8, 3u8, 107u8, 66u8, 202u8, 148u8, 31u8, 6u8,
							236u8, 156u8, 202u8, 197u8, 107u8, 100u8, 60u8, 255u8, 213u8, 222u8,
							209u8, 249u8, 61u8, 209u8, 215u8, 82u8, 73u8, 25u8, 73u8, 161u8, 24u8,
						],
					)
				}
				#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
				#[doc = "proposed again."]
				#[doc = ""]
				#[doc = "If called on a queued public or external proposal, then this will result in it being"]
				#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
				#[doc = "then it will be cancelled."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
				#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
				#[doc = "cancelled."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
				#[doc = "  reasonable value)."]
				pub fn blacklist(
					&self,
					proposal_hash: ::subxt::utils::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<Blacklist> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"blacklist",
						Blacklist { proposal_hash, maybe_ref_index },
						[
							48u8, 144u8, 81u8, 164u8, 54u8, 111u8, 197u8, 134u8, 6u8, 98u8, 121u8,
							179u8, 254u8, 191u8, 204u8, 212u8, 84u8, 255u8, 86u8, 110u8, 225u8,
							130u8, 26u8, 65u8, 133u8, 56u8, 231u8, 15u8, 245u8, 137u8, 146u8,
							242u8,
						],
					)
				}
				#[doc = "Remove a proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
				#[doc = ""]
				#[doc = "- `prop_index`: The index of the proposal to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CancelProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Democracy",
						"cancel_proposal",
						CancelProposal { prop_index },
						[
							179u8, 3u8, 198u8, 244u8, 241u8, 124u8, 205u8, 58u8, 100u8, 80u8,
							177u8, 254u8, 98u8, 220u8, 189u8, 63u8, 229u8, 60u8, 157u8, 83u8,
							142u8, 6u8, 236u8, 183u8, 193u8, 235u8, 253u8, 126u8, 153u8, 185u8,
							74u8, 117u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion has been proposed by a public account."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A public proposal has been tabled for referendum vote."]
			pub struct Tabled {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Tabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Tabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An external proposal has been tabled."]
			pub struct ExternalTabled;
			impl ::subxt::events::StaticEvent for ExternalTabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ExternalTabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A referendum has begun."]
			pub struct Started {
				pub ref_index: ::core::primitive::u32,
				pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			}
			impl ::subxt::events::StaticEvent for Started {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Started";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A proposal has been approved by referendum."]
			pub struct Passed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Passed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Passed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A proposal has been rejected by referendum."]
			pub struct NotPassed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NotPassed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "NotPassed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A referendum has been cancelled."]
			pub struct Cancelled {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Cancelled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Cancelled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account has delegated their vote to another account."]
			pub struct Delegated {
				pub who: ::subxt::utils::AccountId32,
				pub target: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Delegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Delegated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account has cancelled a previous delegation operation."]
			pub struct Undelegated {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Undelegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Undelegated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An external proposal has been vetoed."]
			pub struct Vetoed {
				pub who: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub until: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Vetoed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Vetoed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal_hash has been blacklisted permanently."]
			pub struct Blacklisted {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Blacklisted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Blacklisted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account has voted in a referendum"]
			pub struct Voted {
				pub voter: ::subxt::utils::AccountId32,
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account has secconded a proposal"]
			pub struct Seconded {
				pub seconder: ::subxt::utils::AccountId32,
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Seconded {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Seconded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A proposal got canceled."]
			pub struct ProposalCanceled {
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProposalCanceled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ProposalCanceled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The number of (public) proposals that have been made so far."]
				pub fn public_prop_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"PublicPropCount",
						vec![],
						[
							91u8, 14u8, 171u8, 94u8, 37u8, 157u8, 46u8, 157u8, 254u8, 13u8, 68u8,
							144u8, 23u8, 146u8, 128u8, 159u8, 9u8, 174u8, 74u8, 174u8, 218u8,
							197u8, 23u8, 235u8, 152u8, 226u8, 216u8, 4u8, 120u8, 121u8, 27u8,
							138u8,
						],
					)
				}
				#[doc = " The public proposals. Unsorted. The second item is the proposal."]
				pub fn public_props(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<(
							::core::primitive::u32,
							runtime_types::frame_support::traits::preimages::Bounded<
								runtime_types::composable_runtime::RuntimeCall,
							>,
							::subxt::utils::AccountId32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"PublicProps",
						vec![],
						[
							63u8, 172u8, 211u8, 85u8, 27u8, 14u8, 86u8, 49u8, 133u8, 5u8, 132u8,
							189u8, 138u8, 137u8, 219u8, 37u8, 209u8, 49u8, 172u8, 86u8, 240u8,
							235u8, 42u8, 201u8, 203u8, 12u8, 122u8, 225u8, 0u8, 109u8, 205u8,
							103u8,
						],
					)
				}
				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub fn deposit_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"DepositOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							9u8, 219u8, 11u8, 58u8, 17u8, 194u8, 248u8, 154u8, 135u8, 119u8, 123u8,
							235u8, 252u8, 176u8, 190u8, 162u8, 236u8, 45u8, 237u8, 125u8, 98u8,
							176u8, 184u8, 160u8, 8u8, 181u8, 213u8, 65u8, 14u8, 84u8, 200u8, 64u8,
						],
					)
				}
				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub fn deposit_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
						::core::primitive::u128,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"DepositOf",
						Vec::new(),
						[
							9u8, 219u8, 11u8, 58u8, 17u8, 194u8, 248u8, 154u8, 135u8, 119u8, 123u8,
							235u8, 252u8, 176u8, 190u8, 162u8, 236u8, 45u8, 237u8, 125u8, 98u8,
							176u8, 184u8, 160u8, 8u8, 181u8, 213u8, 65u8, 14u8, 84u8, 200u8, 64u8,
						],
					)
				}
				#[doc = " The next free referendum index, aka the number of referenda started so far."]
				pub fn referendum_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumCount",
						vec![],
						[
							153u8, 210u8, 106u8, 244u8, 156u8, 70u8, 124u8, 251u8, 123u8, 75u8,
							7u8, 189u8, 199u8, 145u8, 95u8, 119u8, 137u8, 11u8, 240u8, 160u8,
							151u8, 248u8, 229u8, 231u8, 89u8, 222u8, 18u8, 237u8, 144u8, 78u8,
							99u8, 58u8,
						],
					)
				}
				#[doc = " The lowest referendum index representing an unbaked referendum. Equal to"]
				#[doc = " `ReferendumCount` if there isn't a unbaked referendum."]
				pub fn lowest_unbaked(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"LowestUnbaked",
						vec![],
						[
							4u8, 51u8, 108u8, 11u8, 48u8, 165u8, 19u8, 251u8, 182u8, 76u8, 163u8,
							73u8, 227u8, 2u8, 212u8, 74u8, 128u8, 27u8, 165u8, 164u8, 111u8, 22u8,
							209u8, 190u8, 103u8, 7u8, 116u8, 16u8, 160u8, 144u8, 123u8, 64u8,
						],
					)
				}
				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub fn referendum_info_of(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							runtime_types::frame_support::traits::preimages::Bounded<
								runtime_types::composable_runtime::RuntimeCall,
							>,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumInfoOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							167u8, 58u8, 230u8, 197u8, 185u8, 56u8, 181u8, 32u8, 81u8, 150u8, 29u8,
							138u8, 142u8, 38u8, 255u8, 216u8, 139u8, 93u8, 56u8, 148u8, 196u8,
							169u8, 168u8, 144u8, 193u8, 200u8, 187u8, 5u8, 141u8, 201u8, 254u8,
							248u8,
						],
					)
				}
				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub fn referendum_info_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							runtime_types::frame_support::traits::preimages::Bounded<
								runtime_types::composable_runtime::RuntimeCall,
							>,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"ReferendumInfoOf",
						Vec::new(),
						[
							167u8, 58u8, 230u8, 197u8, 185u8, 56u8, 181u8, 32u8, 81u8, 150u8, 29u8,
							138u8, 142u8, 38u8, 255u8, 216u8, 139u8, 93u8, 56u8, 148u8, 196u8,
							169u8, 168u8, 144u8, 193u8, 200u8, 187u8, 5u8, 141u8, 201u8, 254u8,
							248u8,
						],
					)
				}
				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub fn voting_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::vote::Voting<
							::core::primitive::u128,
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"VotingOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							125u8, 121u8, 167u8, 170u8, 18u8, 194u8, 183u8, 38u8, 176u8, 48u8,
							30u8, 88u8, 233u8, 196u8, 33u8, 119u8, 160u8, 201u8, 29u8, 183u8, 88u8,
							67u8, 219u8, 137u8, 6u8, 195u8, 11u8, 63u8, 162u8, 181u8, 82u8, 243u8,
						],
					)
				}
				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub fn voting_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_democracy::vote::Voting<
							::core::primitive::u128,
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"VotingOf",
						Vec::new(),
						[
							125u8, 121u8, 167u8, 170u8, 18u8, 194u8, 183u8, 38u8, 176u8, 48u8,
							30u8, 88u8, 233u8, 196u8, 33u8, 119u8, 160u8, 201u8, 29u8, 183u8, 88u8,
							67u8, 219u8, 137u8, 6u8, 195u8, 11u8, 63u8, 162u8, 181u8, 82u8, 243u8,
						],
					)
				}
				#[doc = " True if the last referendum tabled was submitted externally. False if it was a public"]
				#[doc = " proposal."]
				pub fn last_tabled_was_external(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"LastTabledWasExternal",
						vec![],
						[
							3u8, 67u8, 106u8, 1u8, 89u8, 204u8, 4u8, 145u8, 121u8, 44u8, 34u8,
							76u8, 18u8, 206u8, 65u8, 214u8, 222u8, 82u8, 31u8, 223u8, 144u8, 169u8,
							17u8, 6u8, 138u8, 36u8, 113u8, 155u8, 241u8, 106u8, 189u8, 218u8,
						],
					)
				}
				#[doc = " The referendum to be tabled whenever it would be valid to table an external proposal."]
				#[doc = " This happens when a referendum needs to be tabled and one of two conditions are met:"]
				#[doc = " - `LastTabledWasExternal` is `false`; or"]
				#[doc = " - `PublicProps` is empty."]
				pub fn next_external(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"NextExternal",
						vec![],
						[
							213u8, 36u8, 235u8, 75u8, 153u8, 33u8, 140u8, 121u8, 191u8, 197u8,
							17u8, 57u8, 234u8, 67u8, 81u8, 55u8, 123u8, 179u8, 207u8, 124u8, 238u8,
							147u8, 243u8, 126u8, 200u8, 2u8, 16u8, 143u8, 165u8, 143u8, 159u8,
							93u8,
						],
					)
				}
				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub fn blacklist(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Blacklist",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							8u8, 227u8, 185u8, 179u8, 192u8, 92u8, 171u8, 125u8, 237u8, 224u8,
							109u8, 207u8, 44u8, 181u8, 78u8, 17u8, 254u8, 183u8, 199u8, 241u8,
							49u8, 90u8, 101u8, 168u8, 46u8, 89u8, 253u8, 155u8, 38u8, 183u8, 112u8,
							35u8,
						],
					)
				}
				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub fn blacklist_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Blacklist",
						Vec::new(),
						[
							8u8, 227u8, 185u8, 179u8, 192u8, 92u8, 171u8, 125u8, 237u8, 224u8,
							109u8, 207u8, 44u8, 181u8, 78u8, 17u8, 254u8, 183u8, 199u8, 241u8,
							49u8, 90u8, 101u8, 168u8, 46u8, 89u8, 253u8, 155u8, 38u8, 183u8, 112u8,
							35u8,
						],
					)
				}
				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub fn cancellations(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Cancellations",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
						],
					)
				}
				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub fn cancellations_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Democracy",
						"Cancellations",
						Vec::new(),
						[
							154u8, 36u8, 172u8, 46u8, 65u8, 218u8, 30u8, 151u8, 173u8, 186u8,
							166u8, 79u8, 35u8, 226u8, 94u8, 200u8, 67u8, 44u8, 47u8, 7u8, 17u8,
							89u8, 169u8, 166u8, 236u8, 101u8, 68u8, 54u8, 114u8, 141u8, 177u8,
							135u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The period between a proposal being approved and enacted."]
				#[doc = ""]
				#[doc = " It should generally be a little more than the unstake period to ensure that"]
				#[doc = " voting stakers have an opportunity to remove themselves from the system in the case"]
				#[doc = " where they are on the losing side of a vote."]
				pub fn enactment_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"EnactmentPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How often (in blocks) new public referenda are launched."]
				pub fn launch_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"LaunchPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " How often (in blocks) to check for new votes."]
				pub fn voting_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"VotingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The minimum period of vote locking."]
				#[doc = ""]
				#[doc = " It should be no shorter than enactment period to ensure that in the case of an approval,"]
				#[doc = " those successful voters are locked into the consequences that their votes entail."]
				pub fn vote_locking_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"VoteLockingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The minimum amount to be used as a deposit for a public referendum proposal."]
				pub fn minimum_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"MinimumDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Indicator for whether an emergency origin is even allowed to happen. Some chains may"]
				#[doc = " want to set this permanently to `false`, others may want to condition it on things such"]
				#[doc = " as an upgrade having happened recently."]
				pub fn instant_allowed(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"InstantAllowed",
						[
							165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
							252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
							100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
						],
					)
				}
				#[doc = " Minimum voting period allowed for a fast-track referendum."]
				pub fn fast_track_voting_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"FastTrackVotingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Period in blocks where an external proposal may not be re-submitted after being vetoed."]
				pub fn cooloff_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"CooloffPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of votes for an account."]
				#[doc = ""]
				#[doc = " Also used to compute weight, an overly big value can"]
				#[doc = " lead to extrinsic with very big weight: see `delegate` for instance."]
				pub fn max_votes(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"MaxVotes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of public proposals that can exist at any time."]
				pub fn max_proposals(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"MaxProposals",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of deposits a public proposal may have at any time."]
				pub fn max_deposits(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"MaxDeposits",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of items which can be blacklisted."]
				pub fn max_blacklisted(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Democracy",
						"MaxBlacklisted",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							4u8, 170u8, 24u8, 7u8, 213u8, 181u8, 38u8, 176u8, 151u8, 116u8, 198u8,
							178u8, 209u8, 162u8, 117u8, 8u8, 7u8, 119u8, 246u8, 124u8, 101u8, 55u8,
							113u8, 66u8, 184u8, 168u8, 91u8, 110u8, 34u8, 141u8, 14u8, 93u8,
						],
					)
				}
				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							39u8, 80u8, 18u8, 245u8, 43u8, 154u8, 56u8, 17u8, 5u8, 187u8, 243u8,
							133u8, 153u8, 4u8, 36u8, 125u8, 137u8, 173u8, 122u8, 123u8, 210u8,
							154u8, 235u8, 254u8, 117u8, 59u8, 144u8, 46u8, 149u8, 32u8, 255u8,
							163u8,
						],
					)
				}
				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CloseOldWeight> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommittee",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Proposals",
						vec![],
						[
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Voting",
						Vec::new(),
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommittee",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee_membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChangeKey {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
					::subxt::tx::StaticTxPayload::new(
						"TechnicalCommitteeMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "TechnicalCommitteeMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommitteeMembership",
						"Members",
						vec![],
						[
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TechnicalCommitteeMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod release_committee {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Vote {
				pub proposal: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CloseOldWeight {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Close {
				pub proposal_hash: ::subxt::utils::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
				}
				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							4u8, 170u8, 24u8, 7u8, 213u8, 181u8, 38u8, 176u8, 151u8, 116u8, 198u8,
							178u8, 209u8, 162u8, 117u8, 8u8, 7u8, 119u8, 246u8, 124u8, 101u8, 55u8,
							113u8, 66u8, 184u8, 168u8, 91u8, 110u8, 34u8, 141u8, 14u8, 93u8,
						],
					)
				}
				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							39u8, 80u8, 18u8, 245u8, 43u8, 154u8, 56u8, 17u8, 5u8, 187u8, 243u8,
							133u8, 153u8, 4u8, 36u8, 125u8, 137u8, 173u8, 122u8, 123u8, 210u8,
							154u8, 235u8, 254u8, 117u8, 59u8, 144u8, 46u8, 149u8, 32u8, 255u8,
							163u8,
						],
					)
				}
				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close_old_weight(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::OldWeight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<CloseOldWeight> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"close_old_weight",
						CloseOldWeight {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
							17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
							186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
						],
					)
				}
				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseCommittee",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
							225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
							116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "ReleaseCommittee";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"Proposals",
						vec![],
						[
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_runtime::RuntimeCall,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"ProposalOf",
						Vec::new(),
						[
							51u8, 86u8, 136u8, 151u8, 55u8, 26u8, 67u8, 38u8, 58u8, 176u8, 78u8,
							73u8, 33u8, 232u8, 115u8, 99u8, 147u8, 210u8, 31u8, 143u8, 44u8, 79u8,
							216u8, 14u8, 96u8, 177u8, 195u8, 109u8, 221u8, 196u8, 126u8, 184u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"Voting",
						Vec::new(),
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseCommittee",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod release_membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveMember {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SwapMember {
				pub remove: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub add: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChangeKey {
				pub new: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetPrime {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"add_member",
						AddMember { who },
						[
							168u8, 166u8, 6u8, 167u8, 12u8, 109u8, 99u8, 96u8, 240u8, 57u8, 60u8,
							174u8, 57u8, 52u8, 131u8, 16u8, 230u8, 172u8, 23u8, 140u8, 48u8, 131u8,
							73u8, 131u8, 133u8, 217u8, 137u8, 50u8, 165u8, 149u8, 174u8, 188u8,
						],
					)
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"remove_member",
						RemoveMember { who },
						[
							33u8, 178u8, 96u8, 158u8, 126u8, 172u8, 0u8, 207u8, 143u8, 144u8,
							219u8, 28u8, 205u8, 197u8, 192u8, 195u8, 141u8, 26u8, 39u8, 101u8,
							140u8, 88u8, 212u8, 26u8, 221u8, 29u8, 187u8, 160u8, 119u8, 101u8,
							45u8, 162u8,
						],
					)
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"swap_member",
						SwapMember { remove, add },
						[
							52u8, 10u8, 13u8, 175u8, 35u8, 141u8, 159u8, 135u8, 34u8, 235u8, 117u8,
							146u8, 134u8, 49u8, 76u8, 116u8, 93u8, 209u8, 24u8, 242u8, 123u8, 82u8,
							34u8, 192u8, 147u8, 237u8, 163u8, 167u8, 18u8, 64u8, 196u8, 132u8,
						],
					)
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"change_key",
						ChangeKey { new },
						[
							202u8, 114u8, 208u8, 33u8, 254u8, 51u8, 31u8, 220u8, 229u8, 251u8,
							167u8, 149u8, 139u8, 131u8, 252u8, 100u8, 32u8, 20u8, 72u8, 97u8, 5u8,
							8u8, 25u8, 198u8, 95u8, 154u8, 73u8, 220u8, 46u8, 85u8, 162u8, 40u8,
						],
					)
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"set_prime",
						SetPrime { who },
						[
							109u8, 16u8, 35u8, 72u8, 169u8, 141u8, 101u8, 209u8, 241u8, 218u8,
							170u8, 180u8, 37u8, 223u8, 249u8, 37u8, 168u8, 20u8, 130u8, 30u8,
							191u8, 157u8, 230u8, 156u8, 135u8, 73u8, 96u8, 98u8, 193u8, 44u8, 38u8,
							247u8,
						],
					)
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
					::subxt::tx::StaticTxPayload::new(
						"ReleaseMembership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "ReleaseMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseMembership",
						"Members",
						vec![],
						[
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ReleaseMembership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod scheduler {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Schedule {
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Cancel {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ScheduleNamed {
				pub id: [::core::primitive::u8; 32usize],
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelNamed {
				pub id: [::core::primitive::u8; 32usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ScheduleAfter {
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ScheduleNamedAfter {
				pub id: [::core::primitive::u8; 32usize],
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Anonymously schedule a task."]
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Schedule> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule",
						Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							244u8, 10u8, 229u8, 132u8, 238u8, 216u8, 200u8, 208u8, 69u8, 6u8,
							127u8, 118u8, 242u8, 226u8, 3u8, 146u8, 168u8, 85u8, 6u8, 253u8, 49u8,
							10u8, 251u8, 22u8, 115u8, 124u8, 254u8, 144u8, 175u8, 110u8, 39u8,
							209u8,
						],
					)
				}
				#[doc = "Cancel an anonymously scheduled task."]
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Cancel> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"cancel",
						Cancel { when, index },
						[
							81u8, 251u8, 234u8, 17u8, 214u8, 75u8, 19u8, 59u8, 19u8, 30u8, 89u8,
							74u8, 6u8, 216u8, 238u8, 165u8, 7u8, 19u8, 153u8, 253u8, 161u8, 103u8,
							178u8, 227u8, 152u8, 180u8, 80u8, 156u8, 82u8, 126u8, 132u8, 120u8,
						],
					)
				}
				#[doc = "Schedule a named task."]
				pub fn schedule_named(
					&self,
					id: [::core::primitive::u8; 32usize],
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleNamed> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_named",
						ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							191u8, 158u8, 220u8, 5u8, 178u8, 155u8, 145u8, 34u8, 90u8, 28u8, 237u8,
							86u8, 2u8, 75u8, 226u8, 154u8, 137u8, 140u8, 67u8, 113u8, 193u8, 59u8,
							23u8, 91u8, 21u8, 130u8, 120u8, 65u8, 212u8, 156u8, 144u8, 168u8,
						],
					)
				}
				#[doc = "Cancel a named scheduled task."]
				pub fn cancel_named(
					&self,
					id: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::StaticTxPayload<CancelNamed> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"cancel_named",
						CancelNamed { id },
						[
							51u8, 3u8, 140u8, 50u8, 214u8, 211u8, 50u8, 4u8, 19u8, 43u8, 230u8,
							114u8, 18u8, 108u8, 138u8, 67u8, 99u8, 24u8, 255u8, 11u8, 246u8, 37u8,
							192u8, 207u8, 90u8, 157u8, 171u8, 93u8, 233u8, 189u8, 64u8, 180u8,
						],
					)
				}
				#[doc = "Anonymously schedule a task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule`]."]
				#[doc = "# </weight>"]
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleAfter> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_after",
						ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							201u8, 58u8, 255u8, 87u8, 24u8, 214u8, 229u8, 140u8, 189u8, 31u8, 79u8,
							13u8, 80u8, 231u8, 224u8, 212u8, 75u8, 229u8, 114u8, 55u8, 195u8,
							143u8, 166u8, 87u8, 1u8, 249u8, 66u8, 231u8, 182u8, 223u8, 139u8,
							211u8,
						],
					)
				}
				#[doc = "Schedule a named task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
				#[doc = "# </weight>"]
				pub fn schedule_named_after(
					&self,
					id: [::core::primitive::u8; 32usize],
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ScheduleNamedAfter> {
					::subxt::tx::StaticTxPayload::new(
						"Scheduler",
						"schedule_named_after",
						ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							42u8, 171u8, 18u8, 56u8, 203u8, 78u8, 28u8, 35u8, 95u8, 221u8, 118u8,
							167u8, 103u8, 41u8, 61u8, 192u8, 58u8, 213u8, 152u8, 208u8, 205u8,
							121u8, 146u8, 102u8, 217u8, 85u8, 16u8, 154u8, 238u8, 23u8, 160u8,
							138u8,
						],
					)
				}
			}
		}
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Scheduled some task."]
			pub struct Scheduled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Canceled some task."]
			pub struct Canceled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Dispatched some task."]
			pub struct Dispatched {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Dispatched {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Dispatched";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The call for the provided hash was not found so the task has been aborted."]
			pub struct CallUnavailable {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for CallUnavailable {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "CallUnavailable";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
			pub struct PeriodicFailed {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PeriodicFailed {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PeriodicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given task can never be executed since it is overweight."]
			pub struct PermanentlyOverweight {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PermanentlyOverweight {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PermanentlyOverweight";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn incomplete_since(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"IncompleteSince",
						vec![],
						[
							149u8, 66u8, 239u8, 67u8, 235u8, 219u8, 101u8, 182u8, 145u8, 56u8,
							252u8, 150u8, 253u8, 221u8, 125u8, 57u8, 38u8, 152u8, 153u8, 31u8,
							92u8, 238u8, 66u8, 246u8, 104u8, 163u8, 94u8, 73u8, 222u8, 168u8,
							193u8, 227u8,
						],
					)
				}
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::core::option::Option<
								runtime_types::pallet_scheduler::Scheduled<
									[::core::primitive::u8; 32usize],
									runtime_types::frame_support::traits::preimages::Bounded<
										runtime_types::composable_runtime::RuntimeCall,
									>,
									::core::primitive::u32,
									runtime_types::composable_runtime::OriginCaller,
									::subxt::utils::AccountId32,
								>,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Agenda",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							247u8, 134u8, 75u8, 40u8, 110u8, 187u8, 200u8, 150u8, 177u8, 103u8,
							245u8, 201u8, 255u8, 98u8, 135u8, 101u8, 156u8, 119u8, 21u8, 219u8,
							20u8, 82u8, 37u8, 213u8, 216u8, 246u8, 255u8, 101u8, 236u8, 80u8,
							238u8, 38u8,
						],
					)
				}
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::core::option::Option<
								runtime_types::pallet_scheduler::Scheduled<
									[::core::primitive::u8; 32usize],
									runtime_types::frame_support::traits::preimages::Bounded<
										runtime_types::composable_runtime::RuntimeCall,
									>,
									::core::primitive::u32,
									runtime_types::composable_runtime::OriginCaller,
									::subxt::utils::AccountId32,
								>,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Agenda",
						Vec::new(),
						[
							247u8, 134u8, 75u8, 40u8, 110u8, 187u8, 200u8, 150u8, 177u8, 103u8,
							245u8, 201u8, 255u8, 98u8, 135u8, 101u8, 156u8, 119u8, 21u8, 219u8,
							20u8, 82u8, 37u8, 213u8, 216u8, 246u8, 255u8, 101u8, 236u8, 80u8,
							238u8, 38u8,
						],
					)
				}
				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Lookup",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
						],
					)
				}
				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Scheduler",
						"Lookup",
						Vec::new(),
						[
							82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
							187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
							121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
							243u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight that may be scheduled per block for any dispatchables."]
				pub fn maximum_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Scheduler",
						"MaximumWeight",
						[
							206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
							206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
							210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
							106u8, 76u8,
						],
					)
				}
				#[doc = " The maximum number of scheduled calls in the queue for a single block."]
				pub fn max_scheduled_per_block(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Scheduler",
						"MaxScheduledPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod utility {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DispatchAs {
				pub as_origin: ::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceBatch {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct WithWeight {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Send a batch of dispatch calls."]
				#[doc = ""]
				#[doc = "May be called from any origin except `None`."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
				#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				#[doc = ""]
				#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
				#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
				#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
				#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
				#[doc = "event is deposited."]
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<Batch> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"batch",
						Batch { calls },
						[
							196u8, 102u8, 240u8, 249u8, 174u8, 119u8, 102u8, 29u8, 246u8, 65u8,
							147u8, 196u8, 255u8, 194u8, 77u8, 184u8, 124u8, 32u8, 218u8, 219u8,
							110u8, 150u8, 66u8, 130u8, 242u8, 34u8, 161u8, 16u8, 56u8, 118u8,
							211u8, 102u8,
						],
					)
				}
				#[doc = "Send a call through an indexed pseudonym of the sender."]
				#[doc = ""]
				#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
				#[doc = "use the same filter as the origin of this call."]
				#[doc = ""]
				#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
				#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
				#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
				#[doc = "in the Multisig pallet instead."]
				#[doc = ""]
				#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"as_derivative",
						AsDerivative { index, call: ::std::boxed::Box::new(call) },
						[
							81u8, 115u8, 223u8, 52u8, 231u8, 61u8, 39u8, 121u8, 196u8, 27u8, 205u8,
							80u8, 205u8, 180u8, 14u8, 20u8, 78u8, 46u8, 249u8, 85u8, 211u8, 199u8,
							46u8, 17u8, 246u8, 45u8, 122u8, 0u8, 113u8, 13u8, 239u8, 43u8,
						],
					)
				}
				#[doc = "Send a batch of dispatch calls and atomically execute them."]
				#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
				#[doc = ""]
				#[doc = "May be called from any origin except `None`."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
				#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<BatchAll> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"batch_all",
						BatchAll { calls },
						[
							56u8, 224u8, 93u8, 179u8, 193u8, 224u8, 45u8, 108u8, 206u8, 18u8,
							226u8, 42u8, 185u8, 74u8, 16u8, 145u8, 252u8, 171u8, 180u8, 178u8,
							165u8, 87u8, 241u8, 168u8, 98u8, 53u8, 227u8, 184u8, 160u8, 35u8,
							178u8, 111u8,
						],
					)
				}
				#[doc = "Dispatches a function call with a provided origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
				#[doc = "# </weight>"]
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::composable_runtime::OriginCaller,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"dispatch_as",
						DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							38u8, 15u8, 43u8, 15u8, 185u8, 171u8, 124u8, 8u8, 122u8, 145u8, 2u8,
							150u8, 86u8, 237u8, 208u8, 2u8, 124u8, 8u8, 205u8, 95u8, 53u8, 65u8,
							108u8, 231u8, 96u8, 83u8, 29u8, 58u8, 235u8, 92u8, 234u8, 171u8,
						],
					)
				}
				#[doc = "Send a batch of dispatch calls."]
				#[doc = "Unlike `batch`, it allows errors and won't interrupt."]
				#[doc = ""]
				#[doc = "May be called from any origin except `None`."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
				#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
				) -> ::subxt::tx::StaticTxPayload<ForceBatch> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"force_batch",
						ForceBatch { calls },
						[
							1u8, 213u8, 91u8, 226u8, 155u8, 210u8, 44u8, 39u8, 191u8, 161u8, 137u8,
							238u8, 221u8, 220u8, 204u8, 20u8, 253u8, 102u8, 140u8, 73u8, 24u8,
							127u8, 155u8, 145u8, 102u8, 127u8, 196u8, 25u8, 100u8, 19u8, 105u8,
							136u8,
						],
					)
				}
				#[doc = "Dispatch a function call with a specified weight."]
				#[doc = ""]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Root origin to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				pub fn with_weight(
					&self,
					call: runtime_types::composable_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::StaticTxPayload<WithWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"with_weight",
						WithWeight { call: ::std::boxed::Box::new(call), weight },
						[
							224u8, 92u8, 254u8, 228u8, 178u8, 192u8, 67u8, 230u8, 233u8, 111u8,
							46u8, 167u8, 48u8, 195u8, 32u8, 237u8, 30u8, 135u8, 172u8, 125u8, 39u8,
							137u8, 208u8, 104u8, 158u8, 63u8, 45u8, 9u8, 187u8, 0u8, 14u8, 111u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
			#[doc = "well as the error."]
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches completed fully with no error."]
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches completed but has errors."]
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompletedWithErrors";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with no error."]
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with error."]
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A call was dispatched."]
			pub struct DispatchedAs {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for DispatchedAs {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "DispatchedAs";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The limit on the number of batched calls."]
				pub fn batched_calls_limit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Utility",
						"batched_calls_limit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod preimage {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct NotePreimage {
				pub bytes: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UnnotePreimage {
				pub hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RequestPreimage {
				pub hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UnrequestPreimage {
				pub hash: ::subxt::utils::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Register a preimage on-chain."]
				#[doc = ""]
				#[doc = "If the preimage was previously requested, no fees or deposits are taken for providing"]
				#[doc = "the preimage. Otherwise, a deposit is taken proportional to the size of the preimage."]
				pub fn note_preimage(
					&self,
					bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<NotePreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"note_preimage",
						NotePreimage { bytes },
						[
							77u8, 48u8, 104u8, 3u8, 254u8, 65u8, 106u8, 95u8, 204u8, 89u8, 149u8,
							29u8, 144u8, 188u8, 99u8, 23u8, 146u8, 142u8, 35u8, 17u8, 125u8, 130u8,
							31u8, 206u8, 106u8, 83u8, 163u8, 192u8, 81u8, 23u8, 232u8, 230u8,
						],
					)
				}
				#[doc = "Clear an unrequested preimage from the runtime storage."]
				#[doc = ""]
				#[doc = "If `len` is provided, then it will be a much cheaper operation."]
				#[doc = ""]
				#[doc = "- `hash`: The hash of the preimage to be removed from the store."]
				#[doc = "- `len`: The length of the preimage of `hash`."]
				pub fn unnote_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<UnnotePreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"unnote_preimage",
						UnnotePreimage { hash },
						[
							211u8, 204u8, 205u8, 58u8, 33u8, 179u8, 68u8, 74u8, 149u8, 138u8,
							213u8, 45u8, 140u8, 27u8, 106u8, 81u8, 68u8, 212u8, 147u8, 116u8, 27u8,
							130u8, 84u8, 34u8, 231u8, 197u8, 135u8, 8u8, 19u8, 242u8, 207u8, 17u8,
						],
					)
				}
				#[doc = "Request a preimage be uploaded to the chain without paying any fees or deposits."]
				#[doc = ""]
				#[doc = "If the preimage requests has already been provided on-chain, we unreserve any deposit"]
				#[doc = "a user may have paid, and take the control of the preimage out of their hands."]
				pub fn request_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<RequestPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"request_preimage",
						RequestPreimage { hash },
						[
							195u8, 26u8, 146u8, 255u8, 79u8, 43u8, 73u8, 60u8, 115u8, 78u8, 99u8,
							197u8, 137u8, 95u8, 139u8, 141u8, 79u8, 213u8, 170u8, 169u8, 127u8,
							30u8, 236u8, 65u8, 38u8, 16u8, 118u8, 228u8, 141u8, 83u8, 162u8, 233u8,
						],
					)
				}
				#[doc = "Clear a previously made request for a preimage."]
				#[doc = ""]
				#[doc = "NOTE: THIS MUST NOT BE CALLED ON `hash` MORE TIMES THAN `request_preimage`."]
				pub fn unrequest_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<UnrequestPreimage> {
					::subxt::tx::StaticTxPayload::new(
						"Preimage",
						"unrequest_preimage",
						UnrequestPreimage { hash },
						[
							143u8, 225u8, 239u8, 44u8, 237u8, 83u8, 18u8, 105u8, 101u8, 68u8,
							111u8, 116u8, 66u8, 212u8, 63u8, 190u8, 38u8, 32u8, 105u8, 152u8, 69u8,
							177u8, 193u8, 15u8, 60u8, 26u8, 95u8, 130u8, 11u8, 113u8, 187u8, 108u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_preimage::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A preimage has been noted."]
			pub struct Noted {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Noted {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Noted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A preimage has been requested."]
			pub struct Requested {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Requested {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Requested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A preimage has ben cleared."]
			pub struct Cleared {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Cleared {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Cleared";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The request status of a given hash."]
				pub fn status_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_preimage::RequestStatus<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"StatusFor",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}
				#[doc = " The request status of a given hash."]
				pub fn status_for_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_preimage::RequestStatus<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"StatusFor",
						Vec::new(),
						[
							103u8, 208u8, 88u8, 167u8, 244u8, 198u8, 129u8, 134u8, 182u8, 80u8,
							71u8, 192u8, 73u8, 92u8, 190u8, 15u8, 20u8, 132u8, 37u8, 108u8, 88u8,
							233u8, 18u8, 145u8, 9u8, 235u8, 5u8, 132u8, 42u8, 17u8, 227u8, 56u8,
						],
					)
				}
				pub fn preimage_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"PreimageFor",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
						],
					)
				}
				pub fn preimage_for_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Preimage",
						"PreimageFor",
						Vec::new(),
						[
							96u8, 74u8, 30u8, 112u8, 120u8, 41u8, 52u8, 187u8, 252u8, 68u8, 42u8,
							5u8, 61u8, 228u8, 250u8, 192u8, 224u8, 61u8, 53u8, 222u8, 95u8, 148u8,
							6u8, 53u8, 43u8, 152u8, 88u8, 58u8, 185u8, 234u8, 131u8, 124u8,
						],
					)
				}
			}
		}
	}
	pub mod proxy {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Proxy {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub force_proxy_type: ::core::option::Option<
					runtime_types::composable_traits::account_proxy::ProxyType,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddProxy {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveProxy {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveProxies;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CreatePure {
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
				pub index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillPure {
				pub spawner: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub index: ::core::primitive::u16,
				#[codec(compact)]
				pub height: ::core::primitive::u32,
				#[codec(compact)]
				pub ext_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Announce {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveAnnouncement {
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RejectAnnouncement {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub call_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ProxyAnnounced {
				pub delegate: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub real: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub force_proxy_type: ::core::option::Option<
					runtime_types::composable_traits::account_proxy::ProxyType,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
				#[doc = "`add_proxy`."]
				#[doc = ""]
				#[doc = "Removes any corresponding announcement(s)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
				#[doc = "- `call`: The call to be made by the `real` account."]
				pub fn proxy(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<Proxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"proxy",
						Proxy { real, force_proxy_type, call: ::std::boxed::Box::new(call) },
						[
							187u8, 202u8, 209u8, 50u8, 107u8, 71u8, 99u8, 23u8, 66u8, 14u8, 186u8,
							27u8, 87u8, 206u8, 247u8, 192u8, 36u8, 138u8, 150u8, 179u8, 61u8, 54u8,
							190u8, 239u8, 179u8, 44u8, 77u8, 81u8, 126u8, 172u8, 97u8, 199u8,
						],
					)
				}
				#[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
				#[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
				#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
				#[doc = "zero."]
				pub fn add_proxy(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<AddProxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"add_proxy",
						AddProxy { delegate, proxy_type, delay },
						[
							167u8, 117u8, 94u8, 25u8, 229u8, 153u8, 10u8, 136u8, 43u8, 105u8, 59u8,
							191u8, 21u8, 253u8, 0u8, 196u8, 125u8, 99u8, 233u8, 129u8, 220u8,
							152u8, 154u8, 81u8, 160u8, 190u8, 80u8, 140u8, 6u8, 33u8, 175u8, 9u8,
						],
					)
				}
				#[doc = "Unregister a proxy account for the sender."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
				#[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
				pub fn remove_proxy(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveProxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_proxy",
						RemoveProxy { delegate, proxy_type, delay },
						[
							80u8, 164u8, 180u8, 127u8, 48u8, 192u8, 248u8, 115u8, 128u8, 195u8,
							206u8, 87u8, 53u8, 141u8, 82u8, 113u8, 211u8, 26u8, 168u8, 14u8, 132u8,
							128u8, 255u8, 245u8, 86u8, 189u8, 81u8, 105u8, 72u8, 235u8, 179u8,
							106u8,
						],
					)
				}
				#[doc = "Unregister all proxy accounts for the sender."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "WARNING: This may be called on accounts created by `pure`, however if done, then"]
				#[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
				pub fn remove_proxies(&self) -> ::subxt::tx::StaticTxPayload<RemoveProxies> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_proxies",
						RemoveProxies {},
						[
							15u8, 237u8, 27u8, 166u8, 254u8, 218u8, 92u8, 5u8, 213u8, 239u8, 99u8,
							59u8, 1u8, 26u8, 73u8, 252u8, 81u8, 94u8, 214u8, 227u8, 169u8, 58u8,
							40u8, 253u8, 187u8, 225u8, 192u8, 26u8, 19u8, 23u8, 121u8, 129u8,
						],
					)
				}
				#[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
				#[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
				#[doc = ""]
				#[doc = "Requires a `Signed` origin."]
				#[doc = ""]
				#[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
				#[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
				#[doc = "allow for maximum flexibility."]
				#[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
				#[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
				#[doc = "want to use `0`."]
				#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
				#[doc = "zero."]
				#[doc = ""]
				#[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
				#[doc = "same sender, with the same parameters."]
				#[doc = ""]
				#[doc = "Fails if there are insufficient funds to pay for deposit."]
				pub fn create_pure(
					&self,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					delay: ::core::primitive::u32,
					index: ::core::primitive::u16,
				) -> ::subxt::tx::StaticTxPayload<CreatePure> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"create_pure",
						CreatePure { proxy_type, delay, index },
						[
							171u8, 135u8, 187u8, 46u8, 63u8, 151u8, 174u8, 100u8, 0u8, 63u8, 17u8,
							243u8, 33u8, 168u8, 32u8, 177u8, 183u8, 169u8, 126u8, 162u8, 108u8,
							115u8, 61u8, 152u8, 49u8, 208u8, 204u8, 116u8, 44u8, 163u8, 242u8,
							201u8,
						],
					)
				}
				#[doc = "Removes a previously spawned pure proxy."]
				#[doc = ""]
				#[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
				#[doc = "inaccessible."]
				#[doc = ""]
				#[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
				#[doc = "`pure` with corresponding parameters."]
				#[doc = ""]
				#[doc = "- `spawner`: The account that originally called `pure` to create this account."]
				#[doc = "- `index`: The disambiguation index originally passed to `pure`. Probably `0`."]
				#[doc = "- `proxy_type`: The proxy type originally passed to `pure`."]
				#[doc = "- `height`: The height of the chain when the call to `pure` was processed."]
				#[doc = "- `ext_index`: The extrinsic index in which the call to `pure` was processed."]
				#[doc = ""]
				#[doc = "Fails with `NoPermission` in case the caller is not a previously created pure"]
				#[doc = "account whose `pure` call has corresponding parameters."]
				pub fn kill_pure(
					&self,
					spawner: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
					index: ::core::primitive::u16,
					height: ::core::primitive::u32,
					ext_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillPure> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"kill_pure",
						KillPure { spawner, proxy_type, index, height, ext_index },
						[
							109u8, 96u8, 111u8, 212u8, 211u8, 107u8, 224u8, 196u8, 208u8, 160u8,
							221u8, 79u8, 107u8, 113u8, 234u8, 66u8, 27u8, 211u8, 249u8, 241u8,
							173u8, 44u8, 87u8, 14u8, 207u8, 162u8, 235u8, 247u8, 14u8, 126u8,
							139u8, 209u8,
						],
					)
				}
				#[doc = "Publish the hash of a proxy-call that will be made in the future."]
				#[doc = ""]
				#[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
				#[doc = "if the delay associated with the proxy relationship is greater than zero."]
				#[doc = ""]
				#[doc = "No more than `MaxPending` announcements may be made at any one time."]
				#[doc = ""]
				#[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
				#[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
				pub fn announce(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<Announce> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"announce",
						Announce { real, call_hash },
						[
							226u8, 252u8, 69u8, 50u8, 248u8, 212u8, 209u8, 225u8, 201u8, 236u8,
							51u8, 136u8, 56u8, 85u8, 36u8, 130u8, 233u8, 84u8, 44u8, 192u8, 174u8,
							119u8, 245u8, 62u8, 150u8, 78u8, 217u8, 90u8, 167u8, 154u8, 228u8,
							141u8,
						],
					)
				}
				#[doc = "Remove a given announcement."]
				#[doc = ""]
				#[doc = "May be called by a proxy account to remove a call they previously announced and return"]
				#[doc = "the deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
				pub fn remove_announcement(
					&self,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<RemoveAnnouncement> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_announcement",
						RemoveAnnouncement { real, call_hash },
						[
							251u8, 236u8, 113u8, 182u8, 125u8, 244u8, 31u8, 144u8, 66u8, 28u8,
							65u8, 97u8, 67u8, 94u8, 225u8, 210u8, 46u8, 143u8, 242u8, 124u8, 120u8,
							93u8, 23u8, 165u8, 83u8, 177u8, 250u8, 171u8, 58u8, 66u8, 70u8, 64u8,
						],
					)
				}
				#[doc = "Remove the given announcement of a delegate."]
				#[doc = ""]
				#[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
				#[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `delegate`: The account that previously announced the call."]
				#[doc = "- `call_hash`: The hash of the call to be made."]
				pub fn reject_announcement(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::StaticTxPayload<RejectAnnouncement> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"reject_announcement",
						RejectAnnouncement { delegate, call_hash },
						[
							122u8, 165u8, 114u8, 85u8, 209u8, 197u8, 11u8, 96u8, 211u8, 93u8,
							201u8, 42u8, 1u8, 131u8, 254u8, 177u8, 191u8, 212u8, 229u8, 13u8, 28u8,
							163u8, 133u8, 200u8, 113u8, 28u8, 132u8, 45u8, 105u8, 177u8, 82u8,
							206u8,
						],
					)
				}
				#[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
				#[doc = "`add_proxy`."]
				#[doc = ""]
				#[doc = "Removes any corresponding announcement(s)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
				#[doc = "- `call`: The call to be made by the `real` account."]
				pub fn proxy_announced(
					&self,
					delegate: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					real: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					force_proxy_type: ::core::option::Option<
						runtime_types::composable_traits::account_proxy::ProxyType,
					>,
					call: runtime_types::composable_runtime::RuntimeCall,
				) -> ::subxt::tx::StaticTxPayload<ProxyAnnounced> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"proxy_announced",
						ProxyAnnounced {
							delegate,
							real,
							force_proxy_type,
							call: ::std::boxed::Box::new(call),
						},
						[
							103u8, 235u8, 77u8, 225u8, 216u8, 111u8, 37u8, 198u8, 184u8, 4u8,
							118u8, 97u8, 106u8, 79u8, 183u8, 183u8, 15u8, 161u8, 11u8, 35u8, 41u8,
							208u8, 163u8, 49u8, 99u8, 14u8, 178u8, 208u8, 209u8, 39u8, 139u8,
							201u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_proxy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was executed correctly, with the given."]
			pub struct ProxyExecuted {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for ProxyExecuted {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A pure account has been created by new proxy with given"]
			#[doc = "disambiguation index and proxy type."]
			pub struct PureCreated {
				pub pure: ::subxt::utils::AccountId32,
				pub who: ::subxt::utils::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub disambiguation_index: ::core::primitive::u16,
			}
			impl ::subxt::events::StaticEvent for PureCreated {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "PureCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An announcement was placed to make a call in the future."]
			pub struct Announced {
				pub real: ::subxt::utils::AccountId32,
				pub proxy: ::subxt::utils::AccountId32,
				pub call_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Announced {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "Announced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was added."]
			pub struct ProxyAdded {
				pub delegator: ::subxt::utils::AccountId32,
				pub delegatee: ::subxt::utils::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyAdded {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was removed."]
			pub struct ProxyRemoved {
				pub delegator: ::subxt::utils::AccountId32,
				pub delegatee: ::subxt::utils::AccountId32,
				pub proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyRemoved {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
				#[doc = " which are being delegated to, together with the amount held on deposit."]
				pub fn proxies(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								::subxt::utils::AccountId32,
								runtime_types::composable_traits::account_proxy::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Proxies",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							40u8, 92u8, 112u8, 242u8, 63u8, 121u8, 150u8, 79u8, 47u8, 99u8, 191u8,
							214u8, 231u8, 74u8, 248u8, 133u8, 166u8, 152u8, 17u8, 237u8, 19u8,
							225u8, 31u8, 88u8, 91u8, 43u8, 102u8, 129u8, 119u8, 94u8, 170u8, 210u8,
						],
					)
				}
				#[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
				#[doc = " which are being delegated to, together with the amount held on deposit."]
				pub fn proxies_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								::subxt::utils::AccountId32,
								runtime_types::composable_traits::account_proxy::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Proxies",
						Vec::new(),
						[
							40u8, 92u8, 112u8, 242u8, 63u8, 121u8, 150u8, 79u8, 47u8, 99u8, 191u8,
							214u8, 231u8, 74u8, 248u8, 133u8, 166u8, 152u8, 17u8, 237u8, 19u8,
							225u8, 31u8, 88u8, 91u8, 43u8, 102u8, 129u8, 119u8, 94u8, 170u8, 210u8,
						],
					)
				}
				#[doc = " The announcements made by the proxy (key)."]
				pub fn announcements(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								::subxt::utils::AccountId32,
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Announcements",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " The announcements made by the proxy (key)."]
				pub fn announcements_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								::subxt::utils::AccountId32,
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Announcements",
						Vec::new(),
						[
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The base amount of currency needed to reserve for creating a proxy."]
				#[doc = ""]
				#[doc = " This is held for an additional storage item whose value size is"]
				#[doc = " `sizeof(Balance)` bytes and whose key size is `sizeof(AccountId)` bytes."]
				pub fn proxy_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"ProxyDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per proxy added."]
				#[doc = ""]
				#[doc = " This is held for adding 32 bytes plus an instance of `ProxyType` more into a"]
				#[doc = " pre-existing storage value. Thus, when configuring `ProxyDepositFactor` one should take"]
				#[doc = " into account `32 + proxy_type.encode().len()` bytes of data."]
				pub fn proxy_deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"ProxyDepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum amount of proxies allowed for a single account."]
				pub fn max_proxies(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"MaxProxies",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum amount of time-delayed announcements that are allowed to be pending."]
				pub fn max_pending(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"MaxPending",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The base amount of currency needed to reserve for creating an announcement."]
				#[doc = ""]
				#[doc = " This is held when a new storage item holding a `Balance` is created (typically 16"]
				#[doc = " bytes)."]
				pub fn announcement_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"AnnouncementDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per announcement made."]
				#[doc = ""]
				#[doc = " This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes)"]
				#[doc = " into a pre-existing storage value."]
				pub fn announcement_deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"AnnouncementDepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SuspendXcmExecution;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResumeXcmExecution;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateSuspendThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateDropThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateResumeThreshold {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateThresholdWeight {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateWeightRestrictDecay {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct UpdateXcmpMaxIndividualWeight {
				pub new: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Services a single overweight XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight XCM to service"]
				#[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
				#[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
				#[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
							217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
							55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
						],
					)
				}
				#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::tx::StaticTxPayload<SuspendXcmExecution> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"suspend_xcm_execution",
						SuspendXcmExecution {},
						[
							139u8, 76u8, 166u8, 86u8, 106u8, 144u8, 16u8, 47u8, 105u8, 185u8, 7u8,
							7u8, 63u8, 14u8, 250u8, 236u8, 99u8, 121u8, 101u8, 143u8, 28u8, 175u8,
							108u8, 197u8, 226u8, 43u8, 103u8, 92u8, 186u8, 12u8, 51u8, 153u8,
						],
					)
				}
				#[doc = "Resumes all XCM executions for the XCMP queue."]
				#[doc = ""]
				#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::tx::StaticTxPayload<ResumeXcmExecution> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"resume_xcm_execution",
						ResumeXcmExecution {},
						[
							67u8, 111u8, 47u8, 237u8, 79u8, 42u8, 90u8, 56u8, 245u8, 2u8, 20u8,
							23u8, 33u8, 121u8, 135u8, 50u8, 204u8, 147u8, 195u8, 80u8, 177u8,
							202u8, 8u8, 160u8, 164u8, 138u8, 64u8, 252u8, 178u8, 63u8, 102u8,
							245u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
				#[doc = "suspend their sending."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateSuspendThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_suspend_threshold",
						UpdateSuspendThreshold { new },
						[
							155u8, 120u8, 9u8, 228u8, 110u8, 62u8, 233u8, 36u8, 57u8, 85u8, 19u8,
							67u8, 246u8, 88u8, 81u8, 116u8, 243u8, 236u8, 174u8, 130u8, 8u8, 246u8,
							254u8, 97u8, 155u8, 207u8, 123u8, 60u8, 164u8, 14u8, 196u8, 97u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
				#[doc = "messages from the channel."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateDropThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_drop_threshold",
						UpdateDropThreshold { new },
						[
							146u8, 177u8, 164u8, 96u8, 247u8, 182u8, 229u8, 175u8, 194u8, 101u8,
							186u8, 168u8, 94u8, 114u8, 172u8, 119u8, 35u8, 222u8, 175u8, 21u8,
							67u8, 61u8, 216u8, 144u8, 194u8, 10u8, 181u8, 62u8, 166u8, 198u8,
							138u8, 243u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
				#[doc = "message sending may recommence after it has been suspended."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<UpdateResumeThreshold> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_resume_threshold",
						UpdateResumeThreshold { new },
						[
							231u8, 128u8, 80u8, 179u8, 61u8, 50u8, 103u8, 209u8, 103u8, 55u8,
							101u8, 113u8, 150u8, 10u8, 202u8, 7u8, 0u8, 77u8, 58u8, 4u8, 227u8,
							17u8, 225u8, 112u8, 121u8, 203u8, 184u8, 113u8, 231u8, 156u8, 174u8,
							154u8,
						],
					)
				}
				#[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
				pub fn update_threshold_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateThresholdWeight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_threshold_weight",
						UpdateThresholdWeight { new },
						[
							129u8, 208u8, 93u8, 179u8, 45u8, 236u8, 84u8, 209u8, 37u8, 226u8, 88u8,
							123u8, 156u8, 101u8, 93u8, 84u8, 110u8, 61u8, 56u8, 45u8, 14u8, 120u8,
							181u8, 71u8, 174u8, 104u8, 225u8, 36u8, 17u8, 74u8, 94u8, 59u8,
						],
					)
				}
				#[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
				#[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
				pub fn update_weight_restrict_decay(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateWeightRestrictDecay> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_weight_restrict_decay",
						UpdateWeightRestrictDecay { new },
						[
							73u8, 98u8, 189u8, 10u8, 137u8, 162u8, 71u8, 54u8, 24u8, 117u8, 15u8,
							137u8, 251u8, 121u8, 86u8, 5u8, 123u8, 42u8, 151u8, 244u8, 200u8,
							140u8, 104u8, 149u8, 101u8, 14u8, 58u8, 163u8, 208u8, 205u8, 177u8,
							142u8,
						],
					)
				}
				#[doc = "Overwrite the maximum amount of weight any individual message may consume."]
				#[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
				pub fn update_xcmp_max_individual_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<UpdateXcmpMaxIndividualWeight> {
					::subxt::tx::StaticTxPayload::new(
						"XcmpQueue",
						"update_xcmp_max_individual_weight",
						UpdateXcmpMaxIndividualWeight { new },
						[
							52u8, 93u8, 25u8, 215u8, 36u8, 235u8, 88u8, 49u8, 142u8, 132u8, 57u8,
							2u8, 204u8, 195u8, 166u8, 254u8, 235u8, 247u8, 142u8, 207u8, 224u8,
							43u8, 7u8, 106u8, 142u8, 3u8, 188u8, 101u8, 9u8, 75u8, 57u8, 39u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some XCM was executed ok."]
			pub struct Success {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some XCM failed."]
			pub struct Fail {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
				pub error: runtime_types::xcm::v2::traits::Error,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Bad XCM version used."]
			pub struct BadVersion {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Bad XCM format used."]
			pub struct BadFormat {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An upward message was sent to the relay chain."]
			pub struct UpwardMessageSent {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "UpwardMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An HRMP message was sent to a sibling parachain."]
			pub struct XcmpMessageSent {
				pub message_hash: ::core::option::Option<::subxt::utils::H256>,
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM exceeded the individual message weight budget."]
			pub struct OverweightEnqueued {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub sent_at: ::core::primitive::u32,
				pub index: ::core::primitive::u64,
				pub required: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
			pub struct OverweightServiced {
				pub index: ::core::primitive::u64,
				pub used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Status of the inbound XCMP channels."]
				pub fn inbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpStatus",
						vec![],
						[
							183u8, 198u8, 237u8, 153u8, 132u8, 201u8, 87u8, 182u8, 121u8, 164u8,
							129u8, 241u8, 58u8, 192u8, 115u8, 152u8, 7u8, 33u8, 95u8, 51u8, 2u8,
							176u8, 144u8, 12u8, 125u8, 83u8, 92u8, 198u8, 211u8, 101u8, 28u8, 50u8,
						],
					)
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub fn inbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpMessages",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
						],
					)
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub fn inbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"InboundXcmpMessages",
						Vec::new(),
						[
							157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
							130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
							175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
							25u8,
						],
					)
				}
				#[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
				#[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
				#[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
				#[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
				#[doc = " case of the need to send a high-priority signal message this block."]
				#[doc = " The bool is true if there is a signal message waiting to be sent."]
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpStatus",
						vec![],
						[
							238u8, 120u8, 185u8, 141u8, 82u8, 159u8, 41u8, 68u8, 204u8, 15u8, 46u8,
							152u8, 144u8, 74u8, 250u8, 83u8, 71u8, 105u8, 54u8, 53u8, 226u8, 87u8,
							14u8, 202u8, 58u8, 160u8, 54u8, 162u8, 239u8, 248u8, 227u8, 116u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u16>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpMessages",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OutboundXcmpMessages",
						Vec::new(),
						[
							50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
							90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
							186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"SignalMessages",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"SignalMessages",
						Vec::new(),
						[
							156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
							222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
							110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
							142u8,
						],
					)
				}
				#[doc = " The configuration which controls the dynamics of the outbound queue."]
				pub fn queue_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"QueueConfig",
						vec![],
						[
							154u8, 172u8, 227u8, 208u8, 130u8, 93u8, 173u8, 129u8, 33u8, 75u8,
							180u8, 100u8, 35u8, 154u8, 40u8, 188u8, 86u8, 53u8, 74u8, 118u8, 131u8,
							159u8, 240u8, 159u8, 185u8, 45u8, 165u8, 6u8, 90u8, 125u8, 77u8, 253u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These message stay in this storage map until they are manually dispatched via"]
				#[doc = " `service_overweight`."]
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"Overweight",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These message stay in this storage map until they are manually dispatched via"]
				#[doc = " `service_overweight`."]
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"Overweight",
						Vec::new(),
						[
							222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
							149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
							56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
							7u8,
						],
					)
				}
				#[doc = " The number of overweight messages ever recorded in `Overweight`. Also doubles as the next"]
				#[doc = " available free overweight index."]
				pub fn overweight_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"OverweightCount",
						vec![],
						[
							102u8, 180u8, 196u8, 148u8, 115u8, 62u8, 46u8, 238u8, 97u8, 116u8,
							117u8, 42u8, 14u8, 5u8, 72u8, 237u8, 230u8, 46u8, 150u8, 126u8, 89u8,
							64u8, 233u8, 166u8, 180u8, 137u8, 52u8, 233u8, 252u8, 255u8, 36u8,
							20u8,
						],
					)
				}
				#[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
				pub fn queue_suspended(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmpQueue",
						"QueueSuspended",
						vec![],
						[
							23u8, 37u8, 48u8, 112u8, 222u8, 17u8, 252u8, 65u8, 160u8, 217u8, 218u8,
							30u8, 2u8, 1u8, 204u8, 0u8, 251u8, 17u8, 138u8, 197u8, 164u8, 50u8,
							122u8, 0u8, 31u8, 238u8, 147u8, 213u8, 30u8, 132u8, 184u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod relayer_xcm {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Send {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				pub max_weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceXcmVersion {
				pub location:
					::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
				pub xcm_version: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedTeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::StaticTxPayload<Send> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"send",
						Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							190u8, 88u8, 197u8, 248u8, 111u8, 198u8, 199u8, 206u8, 39u8, 121u8,
							23u8, 121u8, 93u8, 82u8, 22u8, 61u8, 96u8, 210u8, 142u8, 249u8, 195u8,
							78u8, 44u8, 8u8, 118u8, 120u8, 113u8, 168u8, 99u8, 94u8, 232u8, 4u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<TeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"teleport_assets",
						TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							255u8, 5u8, 68u8, 38u8, 44u8, 181u8, 75u8, 221u8, 239u8, 103u8, 88u8,
							47u8, 136u8, 90u8, 253u8, 55u8, 0u8, 122u8, 217u8, 126u8, 13u8, 77u8,
							209u8, 41u8, 7u8, 35u8, 235u8, 171u8, 150u8, 235u8, 202u8, 240u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"reserve_transfer_assets",
						ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							177u8, 160u8, 188u8, 106u8, 153u8, 135u8, 121u8, 12u8, 83u8, 233u8,
							43u8, 161u8, 133u8, 26u8, 104u8, 79u8, 113u8, 8u8, 33u8, 128u8, 82u8,
							62u8, 30u8, 46u8, 203u8, 199u8, 175u8, 193u8, 55u8, 130u8, 206u8, 28u8,
						],
					)
				}
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
				#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
				#[doc = "attempt will be made."]
				#[doc = ""]
				#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
				#[doc = "to completion; only that *some* of it was executed."]
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm,
					max_weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"execute",
						Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							191u8, 177u8, 39u8, 21u8, 1u8, 110u8, 39u8, 58u8, 94u8, 27u8, 44u8,
							18u8, 253u8, 135u8, 100u8, 205u8, 0u8, 231u8, 68u8, 247u8, 5u8, 140u8,
							131u8, 184u8, 251u8, 197u8, 100u8, 113u8, 253u8, 255u8, 120u8, 206u8,
						],
					)
				}
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v1::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"force_xcm_version",
						ForceXcmVersion { location: ::std::boxed::Box::new(location), xcm_version },
						[
							231u8, 106u8, 60u8, 226u8, 31u8, 25u8, 20u8, 115u8, 107u8, 246u8,
							248u8, 11u8, 71u8, 183u8, 93u8, 3u8, 219u8, 21u8, 97u8, 188u8, 119u8,
							121u8, 239u8, 72u8, 200u8, 81u8, 6u8, 177u8, 111u8, 188u8, 168u8, 86u8,
						],
					)
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<ForceDefaultXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"force_default_xcm_version",
						ForceDefaultXcmVersion { maybe_xcm_version },
						[
							38u8, 36u8, 59u8, 231u8, 18u8, 79u8, 76u8, 9u8, 200u8, 125u8, 214u8,
							166u8, 37u8, 99u8, 111u8, 161u8, 135u8, 2u8, 133u8, 157u8, 165u8, 18u8,
							152u8, 81u8, 209u8, 255u8, 137u8, 237u8, 28u8, 126u8, 224u8, 141u8,
						],
					)
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::StaticTxPayload<ForceSubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"force_subscribe_version_notify",
						ForceSubscribeVersionNotify { location: ::std::boxed::Box::new(location) },
						[
							136u8, 216u8, 207u8, 51u8, 42u8, 153u8, 92u8, 70u8, 140u8, 169u8,
							172u8, 89u8, 69u8, 28u8, 200u8, 100u8, 209u8, 226u8, 194u8, 240u8,
							71u8, 38u8, 18u8, 6u8, 6u8, 83u8, 103u8, 254u8, 248u8, 241u8, 62u8,
							189u8,
						],
					)
				}
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::StaticTxPayload<ForceUnsubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"force_unsubscribe_version_notify",
						ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							51u8, 72u8, 5u8, 227u8, 251u8, 243u8, 199u8, 9u8, 8u8, 213u8, 191u8,
							52u8, 21u8, 215u8, 170u8, 6u8, 53u8, 242u8, 225u8, 89u8, 150u8, 142u8,
							104u8, 249u8, 225u8, 209u8, 142u8, 234u8, 161u8, 100u8, 153u8, 120u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<LimitedReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"limited_reserve_transfer_assets",
						LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							191u8, 81u8, 68u8, 116u8, 196u8, 125u8, 226u8, 154u8, 144u8, 126u8,
							159u8, 149u8, 17u8, 124u8, 205u8, 60u8, 249u8, 106u8, 38u8, 251u8,
							136u8, 128u8, 81u8, 201u8, 164u8, 242u8, 216u8, 80u8, 21u8, 234u8,
							20u8, 70u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<LimitedTeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"RelayerXcm",
						"limited_teleport_assets",
						LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							29u8, 31u8, 229u8, 83u8, 40u8, 60u8, 36u8, 185u8, 169u8, 74u8, 30u8,
							47u8, 118u8, 118u8, 22u8, 15u8, 246u8, 220u8, 169u8, 135u8, 72u8,
							154u8, 109u8, 192u8, 195u8, 58u8, 121u8, 240u8, 166u8, 243u8, 29u8,
							29u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Execution of an XCM message was attempted."]
			#[doc = ""]
			#[doc = "\\[ outcome \\]"]
			pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A XCM message was sent."]
			#[doc = ""]
			#[doc = "\\[ origin, destination, message \\]"]
			pub struct Sent(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v2::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			#[doc = ""]
			#[doc = "\\[ id, response \\]"]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The registered notification could"]
			#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
			#[doc = "originally budgeted by this runtime for the query result."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
			pub struct NotifyOverweight(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
				pub runtime_types::sp_weights::weight_v2::Weight,
				pub runtime_types::sp_weights::weight_v2::Weight,
			);
			impl ::subxt::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
			#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
			#[doc = "is not `(origin, QueryId, Response)`."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Expected query response has been received but the origin location of the response does"]
			#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			#[doc = ""]
			#[doc = "\\[ origin location, id, expected location \\]"]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
			);
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Expected query response has been received but the expected origin location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Received query response has been read and removed."]
			#[doc = ""]
			#[doc = "\\[ id \\]"]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets have been placed in an asset trap."]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsTrapped(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "\\[ destination, result \\]"]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			#[doc = ""]
			#[doc = "\\[ location, XCM version \\]"]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			#[doc = ""]
			#[doc = "\\[ location, query ID, error \\]"]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			#[doc = ""]
			#[doc = "\\[ location, query ID \\]"]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets have been claimed from an asset trap"]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsClaimed(
				pub ::subxt::utils::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsClaimed {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "AssetsClaimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest available query index."]
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"QueryCounter",
						vec![],
						[
							137u8, 58u8, 184u8, 88u8, 247u8, 22u8, 151u8, 64u8, 50u8, 77u8, 49u8,
							10u8, 234u8, 84u8, 213u8, 156u8, 26u8, 200u8, 214u8, 225u8, 125u8,
							231u8, 42u8, 93u8, 159u8, 168u8, 86u8, 201u8, 116u8, 153u8, 41u8,
							127u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"Queries",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"Queries",
						Vec::new(),
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"AssetTraps",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"AssetTraps",
						Vec::new(),
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
				#[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"SafeXcmVersion",
						vec![],
						[
							1u8, 223u8, 218u8, 204u8, 222u8, 129u8, 137u8, 237u8, 197u8, 142u8,
							233u8, 66u8, 229u8, 153u8, 138u8, 222u8, 113u8, 164u8, 135u8, 213u8,
							233u8, 34u8, 24u8, 23u8, 215u8, 59u8, 40u8, 188u8, 45u8, 244u8, 205u8,
							199u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"SupportedVersion",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"SupportedVersion",
						Vec::new(),
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"VersionNotifiers",
						Vec::new(),
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"VersionNotifyTargets",
						Vec::new(),
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
				#[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
				#[doc = " which is used as a prioritization."]
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<(
							runtime_types::xcm::VersionedMultiLocation,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"VersionDiscoveryQueue",
						vec![],
						[
							30u8, 163u8, 210u8, 133u8, 30u8, 63u8, 36u8, 9u8, 162u8, 133u8, 99u8,
							170u8, 34u8, 205u8, 27u8, 41u8, 226u8, 141u8, 165u8, 151u8, 46u8,
							140u8, 150u8, 242u8, 178u8, 88u8, 164u8, 12u8, 129u8, 118u8, 25u8,
							79u8,
						],
					)
				}
				#[doc = " The current migration's stage, if any."]
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"RelayerXcm",
						"CurrentMigration",
						vec![],
						[
							137u8, 144u8, 168u8, 185u8, 158u8, 90u8, 127u8, 243u8, 227u8, 134u8,
							150u8, 73u8, 15u8, 99u8, 23u8, 47u8, 68u8, 18u8, 39u8, 16u8, 24u8,
							43u8, 161u8, 56u8, 66u8, 111u8, 16u8, 7u8, 252u8, 125u8, 100u8, 225u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 8usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod dmp_queue {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Service a single overweight message."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight message to service."]
				#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `Unknown`: Message of `index` is unknown."]
				#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
					::subxt::tx::StaticTxPayload::new(
						"DmpQueue",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
							217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
							55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is invalid XCM."]
			pub struct InvalidFormat {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is unsupported version of XCM."]
			pub struct UnsupportedVersion {
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message executed with the given outcome."]
			pub struct ExecutedDownward {
				pub message_id: [::core::primitive::u8; 32usize],
				pub outcome: runtime_types::xcm::v2::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The weight limit for handling downward messages was reached."]
			pub struct WeightExhausted {
				pub message_id: [::core::primitive::u8; 32usize],
				pub remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message is overweight and was placed in the overweight queue."]
			pub struct OverweightEnqueued {
				pub message_id: [::core::primitive::u8; 32usize],
				pub overweight_index: ::core::primitive::u64,
				pub required_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Downward message from the overweight queue was executed."]
			pub struct OverweightServiced {
				pub overweight_index: ::core::primitive::u64,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The configuration."]
				pub fn configuration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Configuration",
						vec![],
						[
							133u8, 113u8, 115u8, 164u8, 128u8, 145u8, 234u8, 106u8, 150u8, 54u8,
							247u8, 135u8, 181u8, 197u8, 178u8, 30u8, 204u8, 46u8, 6u8, 137u8, 82u8,
							1u8, 75u8, 171u8, 7u8, 157u8, 3u8, 19u8, 92u8, 10u8, 234u8, 66u8,
						],
					)
				}
				#[doc = " The page index."]
				pub fn page_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"PageIndex",
						vec![],
						[
							94u8, 132u8, 34u8, 67u8, 10u8, 22u8, 235u8, 96u8, 168u8, 26u8, 57u8,
							200u8, 130u8, 218u8, 37u8, 71u8, 28u8, 119u8, 78u8, 107u8, 209u8,
							120u8, 190u8, 2u8, 101u8, 215u8, 122u8, 187u8, 94u8, 38u8, 255u8,
							234u8,
						],
					)
				}
				#[doc = " The queue pages."]
				pub fn pages(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Pages",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						],
					)
				}
				#[doc = " The queue pages."]
				pub fn pages_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Pages",
						Vec::new(),
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						],
					)
				}
				#[doc = " The overweight messages."]
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Overweight",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						],
					)
				}
				#[doc = " The overweight messages."]
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"DmpQueue",
						"Overweight",
						Vec::new(),
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						],
					)
				}
			}
		}
	}
	pub mod x_tokens {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferMultiasset {
				pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferWithFee {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub fee: ::core::primitive::u128,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferMultiassetWithFee {
				pub asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferMulticurrencies {
				pub currencies: ::std::vec::Vec<(
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u128,
				)>,
				pub fee_item: ::core::primitive::u32,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferMultiassets {
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_item: ::core::primitive::u32,
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer native currencies."]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer(
					&self,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer",
						Transfer {
							currency_id,
							amount,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							196u8, 138u8, 190u8, 218u8, 74u8, 1u8, 20u8, 209u8, 183u8, 229u8, 36u8,
							162u8, 149u8, 238u8, 228u8, 21u8, 187u8, 78u8, 13u8, 212u8, 253u8,
							30u8, 80u8, 174u8, 27u8, 71u8, 46u8, 142u8, 237u8, 181u8, 237u8, 194u8,
						],
					)
				}
				#[doc = "Transfer `MultiAsset`."]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer_multiasset(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<TransferMultiasset> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer_multiasset",
						TransferMultiasset {
							asset: ::std::boxed::Box::new(asset),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							168u8, 125u8, 4u8, 160u8, 48u8, 158u8, 253u8, 201u8, 45u8, 200u8,
							145u8, 223u8, 77u8, 78u8, 159u8, 123u8, 71u8, 29u8, 215u8, 184u8,
							132u8, 195u8, 211u8, 25u8, 223u8, 122u8, 240u8, 70u8, 119u8, 73u8,
							236u8, 180u8,
						],
					)
				}
				#[doc = "Transfer native currencies specifying the fee and amount as"]
				#[doc = "separate."]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "`fee` is the amount to be spent to pay for execution in destination"]
				#[doc = "chain. Both fee and amount will be subtracted form the callers"]
				#[doc = "balance."]
				#[doc = ""]
				#[doc = "If `fee` is not high enough to cover for the execution costs in the"]
				#[doc = "destination chain, then the assets will be trapped in the"]
				#[doc = "destination chain"]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer_with_fee(
					&self,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					fee: ::core::primitive::u128,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<TransferWithFee> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer_with_fee",
						TransferWithFee {
							currency_id,
							amount,
							fee,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							225u8, 124u8, 140u8, 20u8, 133u8, 111u8, 177u8, 188u8, 18u8, 32u8,
							102u8, 149u8, 232u8, 40u8, 145u8, 42u8, 96u8, 177u8, 19u8, 182u8,
							224u8, 123u8, 130u8, 222u8, 135u8, 9u8, 240u8, 123u8, 9u8, 155u8, 11u8,
							139u8,
						],
					)
				}
				#[doc = "Transfer `MultiAsset` specifying the fee and amount as separate."]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "`fee` is the multiasset to be spent to pay for execution in"]
				#[doc = "destination chain. Both fee and amount will be subtracted form the"]
				#[doc = "callers balance For now we only accept fee and asset having the same"]
				#[doc = "`MultiLocation` id."]
				#[doc = ""]
				#[doc = "If `fee` is not high enough to cover for the execution costs in the"]
				#[doc = "destination chain, then the assets will be trapped in the"]
				#[doc = "destination chain"]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer_multiasset_with_fee(
					&self,
					asset: runtime_types::xcm::VersionedMultiAsset,
					fee: runtime_types::xcm::VersionedMultiAsset,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<TransferMultiassetWithFee> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer_multiasset_with_fee",
						TransferMultiassetWithFee {
							asset: ::std::boxed::Box::new(asset),
							fee: ::std::boxed::Box::new(fee),
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							117u8, 223u8, 215u8, 138u8, 117u8, 150u8, 131u8, 106u8, 97u8, 222u8,
							152u8, 174u8, 29u8, 52u8, 5u8, 94u8, 121u8, 1u8, 15u8, 3u8, 216u8,
							79u8, 117u8, 216u8, 2u8, 10u8, 139u8, 87u8, 31u8, 227u8, 203u8, 63u8,
						],
					)
				}
				#[doc = "Transfer several currencies specifying the item to be used as fee"]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "`fee_item` is index of the currencies tuple that we want to use for"]
				#[doc = "payment"]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer_multicurrencies(
					&self,
					currencies: ::std::vec::Vec<(
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					)>,
					fee_item: ::core::primitive::u32,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<TransferMulticurrencies> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer_multicurrencies",
						TransferMulticurrencies {
							currencies,
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							192u8, 138u8, 22u8, 254u8, 59u8, 21u8, 245u8, 193u8, 178u8, 131u8,
							10u8, 193u8, 76u8, 16u8, 143u8, 81u8, 80u8, 140u8, 50u8, 241u8, 171u8,
							52u8, 212u8, 30u8, 212u8, 23u8, 67u8, 59u8, 135u8, 1u8, 204u8, 94u8,
						],
					)
				}
				#[doc = "Transfer several `MultiAsset` specifying the item to be used as fee"]
				#[doc = ""]
				#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
				#[doc = "chain, and it would be charged from the transferred assets. If set"]
				#[doc = "below requirements, the execution may fail and assets wouldn't be"]
				#[doc = "received."]
				#[doc = ""]
				#[doc = "`fee_item` is index of the MultiAssets that we want to use for"]
				#[doc = "payment"]
				#[doc = ""]
				#[doc = "It's a no-op if any error on local XCM execution or message sending."]
				#[doc = "Note sending assets out per se doesn't guarantee they would be"]
				#[doc = "received. Receiving depends on if the XCM message could be delivered"]
				#[doc = "by the network, and if the receiving chain would handle"]
				#[doc = "messages correctly."]
				pub fn transfer_multiassets(
					&self,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_item: ::core::primitive::u32,
					dest: runtime_types::xcm::VersionedMultiLocation,
					dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<TransferMultiassets> {
					::subxt::tx::StaticTxPayload::new(
						"XTokens",
						"transfer_multiassets",
						TransferMultiassets {
							assets: ::std::boxed::Box::new(assets),
							fee_item,
							dest: ::std::boxed::Box::new(dest),
							dest_weight_limit,
						},
						[
							140u8, 166u8, 47u8, 19u8, 45u8, 19u8, 105u8, 204u8, 64u8, 140u8, 42u8,
							98u8, 20u8, 67u8, 230u8, 116u8, 151u8, 244u8, 114u8, 246u8, 84u8, 55u8,
							95u8, 177u8, 196u8, 160u8, 226u8, 100u8, 22u8, 241u8, 53u8, 125u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::orml_xtokens::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Transferred `MultiAsset` with fee."]
			pub struct TransferredMultiAssets {
				pub sender: ::subxt::utils::AccountId32,
				pub assets: runtime_types::xcm::v1::multiasset::MultiAssets,
				pub fee: runtime_types::xcm::v1::multiasset::MultiAsset,
				pub dest: runtime_types::xcm::v1::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for TransferredMultiAssets {
				const PALLET: &'static str = "XTokens";
				const EVENT: &'static str = "TransferredMultiAssets";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Self chain location."]
				pub fn self_location(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::xcm::v1::multilocation::MultiLocation,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"XTokens",
						"SelfLocation",
						[
							17u8, 25u8, 246u8, 131u8, 96u8, 96u8, 11u8, 104u8, 167u8, 145u8, 236u8,
							252u8, 208u8, 71u8, 76u8, 167u8, 110u8, 207u8, 172u8, 152u8, 131u8,
							71u8, 116u8, 5u8, 227u8, 226u8, 68u8, 98u8, 156u8, 138u8, 180u8, 32u8,
						],
					)
				}
				#[doc = " Base XCM weight."]
				#[doc = ""]
				#[doc = " The actually weight for an XCM message is `T::BaseXcmWeight +"]
				#[doc = " T::Weigher::weight(&msg)`."]
				pub fn base_xcm_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"XTokens",
						"BaseXcmWeight",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod unknown_tokens {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::orml_unknown_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Deposit success."]
			pub struct Deposited {
				pub asset: runtime_types::xcm::v1::multiasset::MultiAsset,
				pub who: runtime_types::xcm::v1::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "UnknownTokens";
				const EVENT: &'static str = "Deposited";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Withdraw success."]
			pub struct Withdrawn {
				pub asset: runtime_types::xcm::v1::multiasset::MultiAsset,
				pub who: runtime_types::xcm::v1::multilocation::MultiLocation,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "UnknownTokens";
				const EVENT: &'static str = "Withdrawn";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Concrete fungible balances under a given location and a concrete"]
				#[doc = " fungible id."]
				#[doc = ""]
				#[doc = " double_map: who, asset_id => u128"]
				pub fn concrete_fungible_balances(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v1::multilocation::MultiLocation>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::v1::multilocation::MultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"UnknownTokens",
						"ConcreteFungibleBalances",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							209u8, 108u8, 116u8, 241u8, 240u8, 48u8, 22u8, 142u8, 204u8, 254u8,
							45u8, 213u8, 98u8, 254u8, 240u8, 178u8, 77u8, 175u8, 106u8, 110u8,
							109u8, 42u8, 239u8, 127u8, 79u8, 229u8, 178u8, 232u8, 251u8, 0u8,
							235u8, 245u8,
						],
					)
				}
				#[doc = " Concrete fungible balances under a given location and a concrete"]
				#[doc = " fungible id."]
				#[doc = ""]
				#[doc = " double_map: who, asset_id => u128"]
				pub fn concrete_fungible_balances_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"UnknownTokens",
						"ConcreteFungibleBalances",
						Vec::new(),
						[
							209u8, 108u8, 116u8, 241u8, 240u8, 48u8, 22u8, 142u8, 204u8, 254u8,
							45u8, 213u8, 98u8, 254u8, 240u8, 178u8, 77u8, 175u8, 106u8, 110u8,
							109u8, 42u8, 239u8, 127u8, 79u8, 229u8, 178u8, 232u8, 251u8, 0u8,
							235u8, 245u8,
						],
					)
				}
				#[doc = " Abstract fungible balances under a given location and a abstract"]
				#[doc = " fungible id."]
				#[doc = ""]
				#[doc = " double_map: who, asset_id => u128"]
				pub fn abstract_fungible_balances(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::v1::multilocation::MultiLocation>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"UnknownTokens",
						"AbstractFungibleBalances",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							167u8, 134u8, 51u8, 219u8, 157u8, 244u8, 11u8, 193u8, 209u8, 133u8,
							106u8, 27u8, 229u8, 7u8, 164u8, 57u8, 82u8, 212u8, 149u8, 108u8, 69u8,
							132u8, 98u8, 213u8, 126u8, 122u8, 126u8, 21u8, 70u8, 220u8, 51u8,
							249u8,
						],
					)
				}
				#[doc = " Abstract fungible balances under a given location and a abstract"]
				#[doc = " fungible id."]
				#[doc = ""]
				#[doc = " double_map: who, asset_id => u128"]
				pub fn abstract_fungible_balances_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"UnknownTokens",
						"AbstractFungibleBalances",
						Vec::new(),
						[
							167u8, 134u8, 51u8, 219u8, 157u8, 244u8, 11u8, 193u8, 209u8, 133u8,
							106u8, 27u8, 229u8, 7u8, 164u8, 57u8, 82u8, 212u8, 149u8, 108u8, 69u8,
							132u8, 98u8, 213u8, 126u8, 122u8, 126u8, 21u8, 70u8, 220u8, 51u8,
							249u8,
						],
					)
				}
			}
		}
	}
	pub mod tokens {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAll {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetBalance {
				pub who: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "It will decrease the total issuance of the system by the"]
				#[doc = "`TransferFee`. If the sender's account is below the existential"]
				#[doc = "deposit as a result of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Tokens",
						"transfer",
						Transfer { dest, currency_id, amount },
						[
							206u8, 83u8, 17u8, 48u8, 58u8, 130u8, 54u8, 103u8, 110u8, 163u8, 160u8,
							138u8, 162u8, 221u8, 65u8, 125u8, 126u8, 44u8, 210u8, 48u8, 212u8,
							83u8, 229u8, 173u8, 146u8, 129u8, 21u8, 111u8, 45u8, 85u8, 160u8,
							167u8,
						],
					)
				}
				#[doc = "Transfer all remaining balance to the given account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_"]
				#[doc = "balances. This means that any locked, reserved, or existential"]
				#[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
				#[doc = "this function. To ensure that this function results in a killed"]
				#[doc = "account, you might need to prepare the account by removing any"]
				#[doc = "reference counters, storage deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
				#[doc = "  operation should send all of the funds the account has, causing"]
				#[doc = "  the sender account to be killed (false), or transfer everything"]
				#[doc = "  except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Tokens",
						"transfer_all",
						TransferAll { dest, currency_id, keep_alive },
						[
							3u8, 187u8, 164u8, 247u8, 255u8, 219u8, 96u8, 91u8, 177u8, 2u8, 216u8,
							236u8, 119u8, 10u8, 114u8, 150u8, 166u8, 218u8, 88u8, 232u8, 119u8,
							132u8, 9u8, 185u8, 181u8, 40u8, 54u8, 107u8, 162u8, 201u8, 100u8,
							116u8,
						],
					)
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
				#[doc = "will not kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Tokens",
						"transfer_keep_alive",
						TransferKeepAlive { dest, currency_id, amount },
						[
							220u8, 153u8, 159u8, 112u8, 205u8, 69u8, 215u8, 153u8, 140u8, 202u8,
							205u8, 131u8, 61u8, 239u8, 33u8, 15u8, 133u8, 230u8, 2u8, 31u8, 61u8,
							4u8, 103u8, 190u8, 69u8, 89u8, 171u8, 57u8, 136u8, 54u8, 112u8, 194u8,
						],
					)
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source"]
				#[doc = "account may be specified."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `source`: The sender of the transfer."]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Tokens",
						"force_transfer",
						ForceTransfer { source, dest, currency_id, amount },
						[
							201u8, 63u8, 141u8, 47u8, 68u8, 174u8, 30u8, 110u8, 86u8, 85u8, 129u8,
							234u8, 133u8, 0u8, 122u8, 248u8, 80u8, 160u8, 1u8, 5u8, 194u8, 197u8,
							125u8, 162u8, 45u8, 116u8, 198u8, 249u8, 200u8, 108u8, 175u8, 22u8,
						],
					)
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
				#[doc = "will also decrease the total issuance of the system"]
				#[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
				#[doc = "existential deposit, it will reap the `AccountInfo`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Tokens",
						"set_balance",
						SetBalance { who, currency_id, new_free, new_reserved },
						[
							163u8, 191u8, 141u8, 39u8, 221u8, 176u8, 17u8, 59u8, 148u8, 120u8,
							11u8, 122u8, 195u8, 81u8, 44u8, 216u8, 33u8, 205u8, 119u8, 59u8, 77u8,
							92u8, 1u8, 107u8, 206u8, 78u8, 100u8, 51u8, 155u8, 122u8, 228u8, 24u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::orml_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was removed whose balance was non-zero but below"]
			#[doc = "ExistentialDeposit, resulting in an outright loss."]
			pub struct DustLost {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some reserved balance was repatriated (moved from reserved to"]
			#[doc = "another account)."]
			pub struct ReserveRepatriated {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The total issuance of an currency has been set"]
			pub struct TotalIssuanceSet {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "TotalIssuanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balances were withdrawn (e.g. pay for transaction fee)"]
			pub struct Withdrawn {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Withdrawn";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balances were slashed (e.g. due to mis-behavior)"]
			pub struct Slashed {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub free_amount: ::core::primitive::u128,
				pub reserved_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Deposited some balance into an account"]
			pub struct Deposited {
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposited {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Deposited";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some funds are locked"]
			pub struct LockSet {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for LockSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some locked funds were unlocked"]
			pub struct LockRemoved {
				pub lock_id: [::core::primitive::u8; 8usize],
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for LockRemoved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "LockRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total issuance of a token type."]
				pub fn total_issuance(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"TotalIssuance",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							203u8, 177u8, 143u8, 200u8, 190u8, 210u8, 147u8, 46u8, 47u8, 202u8,
							42u8, 57u8, 168u8, 246u8, 168u8, 203u8, 190u8, 234u8, 253u8, 130u8,
							72u8, 19u8, 2u8, 227u8, 115u8, 21u8, 106u8, 71u8, 239u8, 148u8, 192u8,
							106u8,
						],
					)
				}
				#[doc = " The total issuance of a token type."]
				pub fn total_issuance_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"TotalIssuance",
						Vec::new(),
						[
							203u8, 177u8, 143u8, 200u8, 190u8, 210u8, 147u8, 46u8, 47u8, 202u8,
							42u8, 57u8, 168u8, 246u8, 168u8, 203u8, 190u8, 234u8, 253u8, 130u8,
							72u8, 19u8, 2u8, 227u8, 115u8, 21u8, 106u8, 71u8, 239u8, 148u8, 192u8,
							106u8,
						],
					)
				}
				#[doc = " Any liquidity locks of a token type under an account."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Locks",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							102u8, 209u8, 145u8, 141u8, 113u8, 97u8, 120u8, 28u8, 130u8, 122u8,
							139u8, 193u8, 38u8, 34u8, 146u8, 166u8, 222u8, 97u8, 193u8, 137u8,
							116u8, 56u8, 3u8, 118u8, 192u8, 249u8, 74u8, 17u8, 224u8, 53u8, 209u8,
							195u8,
						],
					)
				}
				#[doc = " Any liquidity locks of a token type under an account."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Locks",
						Vec::new(),
						[
							102u8, 209u8, 145u8, 141u8, 113u8, 97u8, 120u8, 28u8, 130u8, 122u8,
							139u8, 193u8, 38u8, 34u8, 146u8, 166u8, 222u8, 97u8, 193u8, 137u8,
							116u8, 56u8, 3u8, 118u8, 192u8, 249u8, 74u8, 17u8, 224u8, 53u8, 209u8,
							195u8,
						],
					)
				}
				#[doc = " The balance of a token type under an account."]
				#[doc = ""]
				#[doc = " NOTE: If the total is ever zero, decrease account ref account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this module is used to store"]
				#[doc = " balances."]
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Accounts",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							80u8, 235u8, 135u8, 10u8, 123u8, 40u8, 79u8, 225u8, 219u8, 128u8,
							105u8, 19u8, 7u8, 57u8, 131u8, 239u8, 221u8, 8u8, 122u8, 212u8, 191u8,
							186u8, 232u8, 221u8, 196u8, 10u8, 150u8, 219u8, 132u8, 161u8, 60u8,
							247u8,
						],
					)
				}
				#[doc = " The balance of a token type under an account."]
				#[doc = ""]
				#[doc = " NOTE: If the total is ever zero, decrease account ref account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this module is used to store"]
				#[doc = " balances."]
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Accounts",
						Vec::new(),
						[
							80u8, 235u8, 135u8, 10u8, 123u8, 40u8, 79u8, 225u8, 219u8, 128u8,
							105u8, 19u8, 7u8, 57u8, 131u8, 239u8, 221u8, 8u8, 122u8, 212u8, 191u8,
							186u8, 232u8, 221u8, 196u8, 10u8, 150u8, 219u8, 132u8, 161u8, 60u8,
							247u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::orml_tokens::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Reserves",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							68u8, 199u8, 14u8, 150u8, 65u8, 10u8, 132u8, 26u8, 235u8, 92u8, 5u8,
							96u8, 117u8, 28u8, 179u8, 113u8, 214u8, 210u8, 136u8, 183u8, 137u8,
							23u8, 64u8, 12u8, 181u8, 8u8, 239u8, 215u8, 57u8, 78u8, 237u8, 94u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
							runtime_types::orml_tokens::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Tokens",
						"Reserves",
						Vec::new(),
						[
							68u8, 199u8, 14u8, 150u8, 65u8, 10u8, 132u8, 26u8, 235u8, 92u8, 5u8,
							96u8, 117u8, 28u8, 179u8, 113u8, 214u8, 210u8, 136u8, 183u8, 137u8,
							23u8, 64u8, 12u8, 181u8, 8u8, 239u8, 215u8, 57u8, 78u8, 237u8, 94u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_locks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Tokens",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Tokens",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod currency_factory {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct AddRange {
				pub length: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMetadata {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn add_range(
					&self,
					length: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<AddRange> {
					::subxt::tx::StaticTxPayload::new(
						"CurrencyFactory",
						"add_range",
						AddRange { length },
						[
							239u8, 242u8, 170u8, 252u8, 41u8, 195u8, 156u8, 238u8, 196u8, 166u8,
							6u8, 228u8, 202u8, 48u8, 230u8, 140u8, 228u8, 214u8, 157u8, 67u8, 81u8,
							9u8, 215u8, 113u8, 199u8, 238u8, 2u8, 163u8, 239u8, 192u8, 155u8, 38u8,
						],
					)
				}
				#[doc = "Sets metadata"]
				pub fn set_metadata(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
				) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
					::subxt::tx::StaticTxPayload::new(
						"CurrencyFactory",
						"set_metadata",
						SetMetadata { asset_id, metadata },
						[
							247u8, 186u8, 131u8, 92u8, 172u8, 202u8, 106u8, 118u8, 77u8, 255u8,
							150u8, 218u8, 247u8, 1u8, 131u8, 42u8, 160u8, 162u8, 191u8, 154u8,
							150u8, 65u8, 23u8, 188u8, 183u8, 58u8, 102u8, 64u8, 16u8, 229u8, 234u8,
							32u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_currency_factory::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RangeCreated {
				pub range: runtime_types::pallet_currency_factory::ranges::Range<
					runtime_types::primitives::currency::CurrencyId,
				>,
			}
			impl ::subxt::events::StaticEvent for RangeCreated {
				const PALLET: &'static str = "CurrencyFactory";
				const EVENT: &'static str = "RangeCreated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn asset_id_ranges(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_currency_factory::ranges::Ranges<
							runtime_types::primitives::currency::CurrencyId,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CurrencyFactory",
						"AssetIdRanges",
						vec![],
						[
							22u8, 227u8, 15u8, 251u8, 150u8, 72u8, 61u8, 107u8, 142u8, 193u8,
							253u8, 199u8, 241u8, 219u8, 138u8, 28u8, 59u8, 177u8, 155u8, 80u8,
							26u8, 245u8, 85u8, 141u8, 122u8, 161u8, 215u8, 147u8, 202u8, 168u8,
							149u8, 156u8,
						],
					)
				}
				pub fn asset_ed(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CurrencyFactory",
						"AssetEd",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox128,
						)],
						[
							157u8, 246u8, 89u8, 3u8, 170u8, 111u8, 221u8, 215u8, 106u8, 78u8, 11u8,
							245u8, 15u8, 218u8, 143u8, 173u8, 188u8, 148u8, 224u8, 153u8, 82u8,
							54u8, 242u8, 102u8, 164u8, 129u8, 100u8, 119u8, 69u8, 227u8, 144u8,
							62u8,
						],
					)
				}
				pub fn asset_ed_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CurrencyFactory",
						"AssetEd",
						Vec::new(),
						[
							157u8, 246u8, 89u8, 3u8, 170u8, 111u8, 221u8, 215u8, 106u8, 78u8, 11u8,
							245u8, 15u8, 218u8, 143u8, 173u8, 188u8, 148u8, 224u8, 153u8, 82u8,
							54u8, 242u8, 102u8, 164u8, 129u8, 100u8, 119u8, 69u8, 227u8, 144u8,
							62u8,
						],
					)
				}
				pub fn asset_metadata(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_traits::assets::BasicAssetMetadata,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CurrencyFactory",
						"AssetMetadata",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox128,
						)],
						[
							185u8, 114u8, 217u8, 111u8, 55u8, 241u8, 125u8, 51u8, 95u8, 89u8, 39u8,
							166u8, 183u8, 208u8, 129u8, 214u8, 56u8, 6u8, 0u8, 44u8, 134u8, 242u8,
							45u8, 238u8, 61u8, 41u8, 155u8, 137u8, 166u8, 53u8, 130u8, 28u8,
						],
					)
				}
				pub fn asset_metadata_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_traits::assets::BasicAssetMetadata,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CurrencyFactory",
						"AssetMetadata",
						Vec::new(),
						[
							185u8, 114u8, 217u8, 111u8, 55u8, 241u8, 125u8, 51u8, 95u8, 89u8, 39u8,
							166u8, 183u8, 208u8, 129u8, 214u8, 56u8, 6u8, 0u8, 44u8, 134u8, 242u8,
							45u8, 238u8, 61u8, 41u8, 155u8, 137u8, 166u8, 53u8, 130u8, 28u8,
						],
					)
				}
			}
		}
	}
	pub mod crowdloan_rewards {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Initialize;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct InitializeAt {
				pub at: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Populate {
				pub rewards: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Associate {
				pub reward_account: ::subxt::utils::AccountId32,
				pub proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
					[::core::primitive::u8; 32usize],
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Claim;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UnlockRewardsFor {
				pub reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Add {
				pub additions: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Initialize the pallet at the current timestamp."]
				pub fn initialize(&self) -> ::subxt::tx::StaticTxPayload<Initialize> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"initialize",
						Initialize {},
						[
							210u8, 6u8, 171u8, 194u8, 188u8, 76u8, 163u8, 192u8, 223u8, 241u8,
							194u8, 189u8, 221u8, 190u8, 28u8, 191u8, 208u8, 85u8, 140u8, 167u8,
							160u8, 29u8, 155u8, 216u8, 185u8, 27u8, 109u8, 39u8, 4u8, 82u8, 50u8,
							180u8,
						],
					)
				}
				#[doc = "Initialize the pallet at the given timestamp."]
				pub fn initialize_at(
					&self,
					at: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<InitializeAt> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"initialize_at",
						InitializeAt { at },
						[
							213u8, 36u8, 13u8, 147u8, 34u8, 81u8, 248u8, 154u8, 70u8, 189u8, 57u8,
							225u8, 107u8, 84u8, 25u8, 18u8, 160u8, 135u8, 118u8, 251u8, 223u8,
							204u8, 43u8, 65u8, 50u8, 130u8, 31u8, 80u8, 16u8, 158u8, 173u8, 20u8,
						],
					)
				}
				#[doc = "Populate pallet by adding more rewards."]
				#[doc = ""]
				#[doc = "Each index in the rewards vector should contain: `remote_account`, `reward_account`,"]
				#[doc = "`vesting_period`."]
				#[doc = ""]
				#[doc = "Can be called multiple times. If an remote account"]
				#[doc = "already has a reward, it will be replaced by the new reward value."]
				#[doc = ""]
				#[doc = "Can only be called before `initialize`."]
				pub fn populate(
					&self,
					rewards: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::StaticTxPayload<Populate> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"populate",
						Populate { rewards },
						[
							218u8, 90u8, 77u8, 124u8, 0u8, 48u8, 38u8, 89u8, 185u8, 77u8, 126u8,
							131u8, 109u8, 112u8, 52u8, 19u8, 65u8, 60u8, 0u8, 81u8, 45u8, 173u8,
							161u8, 245u8, 167u8, 248u8, 246u8, 147u8, 244u8, 181u8, 23u8, 138u8,
						],
					)
				}
				#[doc = "Associate a reward account. A valid proof has to be provided."]
				#[doc = "This call also claim the first reward (a.k.a. the first payment, which is a % of the"]
				#[doc = "vested reward)."]
				#[doc = "If logic gate pass, no fees are applied."]
				#[doc = ""]
				#[doc = "The proof should be:"]
				#[doc = "```haskell"]
				#[doc = "proof = sign (concat prefix (hex reward_account))"]
				#[doc = "```"]
				pub fn associate(
					&self,
					reward_account: ::subxt::utils::AccountId32,
					proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
						[::core::primitive::u8; 32usize],
					>,
				) -> ::subxt::tx::StaticTxPayload<Associate> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"associate",
						Associate { reward_account, proof },
						[
							172u8, 177u8, 98u8, 224u8, 100u8, 149u8, 103u8, 198u8, 237u8, 49u8,
							31u8, 231u8, 222u8, 173u8, 28u8, 233u8, 20u8, 109u8, 135u8, 134u8,
							170u8, 40u8, 244u8, 28u8, 174u8, 139u8, 51u8, 229u8, 120u8, 251u8,
							73u8, 5u8,
						],
					)
				}
				#[doc = "Claim a reward from the associated reward account."]
				#[doc = "A previous call to `associate` should have been made."]
				#[doc = "If logic gate pass, no fees are applied."]
				pub fn claim(&self) -> ::subxt::tx::StaticTxPayload<Claim> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"claim",
						Claim {},
						[
							45u8, 97u8, 229u8, 222u8, 255u8, 43u8, 179u8, 22u8, 163u8, 231u8, 33u8,
							96u8, 167u8, 206u8, 213u8, 116u8, 80u8, 254u8, 184u8, 3u8, 96u8, 5u8,
							160u8, 81u8, 148u8, 30u8, 117u8, 255u8, 107u8, 177u8, 200u8, 78u8,
						],
					)
				}
				pub fn unlock_rewards_for(
					&self,
					reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<UnlockRewardsFor> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"unlock_rewards_for",
						UnlockRewardsFor { reward_accounts },
						[
							116u8, 71u8, 22u8, 93u8, 198u8, 85u8, 61u8, 147u8, 75u8, 125u8, 232u8,
							122u8, 54u8, 186u8, 142u8, 244u8, 235u8, 65u8, 164u8, 187u8, 11u8,
							90u8, 72u8, 111u8, 104u8, 109u8, 239u8, 164u8, 148u8, 43u8, 248u8,
							187u8,
						],
					)
				}
				#[doc = "Adds all accounts in the `additions` vector. Add may be called even if the pallet has"]
				#[doc = "been initialized."]
				pub fn add(
					&self,
					additions: ::std::vec::Vec<(
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
						::core::primitive::u128,
						::core::primitive::u64,
					)>,
				) -> ::subxt::tx::StaticTxPayload<Add> {
					::subxt::tx::StaticTxPayload::new(
						"CrowdloanRewards",
						"add",
						Add { additions },
						[
							253u8, 134u8, 219u8, 228u8, 49u8, 178u8, 201u8, 105u8, 69u8, 236u8,
							199u8, 179u8, 132u8, 113u8, 73u8, 110u8, 181u8, 233u8, 215u8, 145u8,
							27u8, 32u8, 221u8, 185u8, 168u8, 129u8, 56u8, 102u8, 94u8, 172u8, 59u8,
							129u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_crowdloan_rewards::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "The crowdloan has been initialized or set to initialize at some time."]
			pub struct Initialized {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Initialized {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Initialized";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A claim has been made."]
			pub struct Claimed {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					[::core::primitive::u8; 32usize],
				>,
				pub reward_account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Claimed {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Claimed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A remote account has been associated with a reward account."]
			pub struct Associated {
				pub remote_account: runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
					[::core::primitive::u8; 32usize],
				>,
				pub reward_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Associated {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "Associated";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "The crowdloan was successfully initialized, but with excess funds that won't be"]
			#[doc = "claimed."]
			pub struct OverFunded {
				pub excess_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for OverFunded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "OverFunded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A portion of rewards have been unlocked and future claims will not have locks"]
			pub struct RewardsUnlocked {
				pub at: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for RewardsUnlocked {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsUnlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Called after rewards have been added through the `add` extrinsic."]
			pub struct RewardsAdded {
				pub additions: ::std::vec::Vec<(
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
					::core::primitive::u128,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for RewardsAdded {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Called after rewards have been deleted through the `delete` extrinsic."]
			pub struct RewardsDeleted {
				pub deletions: ::std::vec::Vec<
					runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
						[::core::primitive::u8; 32usize],
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for RewardsDeleted {
				const PALLET: &'static str = "CrowdloanRewards";
				const EVENT: &'static str = "RewardsDeleted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn rewards(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_crowdloan_rewards::models::Reward<
							::core::primitive::u128,
							::core::primitive::u64,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"Rewards",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							82u8, 149u8, 137u8, 45u8, 253u8, 248u8, 147u8, 196u8, 112u8, 11u8,
							148u8, 238u8, 132u8, 84u8, 242u8, 5u8, 204u8, 201u8, 41u8, 27u8, 252u8,
							42u8, 94u8, 250u8, 127u8, 169u8, 230u8, 130u8, 193u8, 10u8, 252u8,
							145u8,
						],
					)
				}
				pub fn rewards_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_crowdloan_rewards::models::Reward<
							::core::primitive::u128,
							::core::primitive::u64,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"Rewards",
						Vec::new(),
						[
							82u8, 149u8, 137u8, 45u8, 253u8, 248u8, 147u8, 196u8, 112u8, 11u8,
							148u8, 238u8, 132u8, 84u8, 242u8, 5u8, 204u8, 201u8, 41u8, 27u8, 252u8,
							42u8, 94u8, 250u8, 127u8, 169u8, 230u8, 130u8, 193u8, 10u8, 252u8,
							145u8,
						],
					)
				}
				#[doc = " The total amount of rewards to be claimed."]
				pub fn total_rewards(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"TotalRewards",
						vec![],
						[
							37u8, 36u8, 124u8, 79u8, 45u8, 126u8, 177u8, 179u8, 118u8, 125u8,
							178u8, 245u8, 125u8, 208u8, 201u8, 248u8, 51u8, 5u8, 202u8, 199u8,
							82u8, 75u8, 64u8, 150u8, 40u8, 196u8, 223u8, 17u8, 32u8, 105u8, 208u8,
							126u8,
						],
					)
				}
				#[doc = " The rewards claimed so far."]
				pub fn claimed_rewards(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"ClaimedRewards",
						vec![],
						[
							250u8, 96u8, 206u8, 11u8, 109u8, 190u8, 255u8, 1u8, 24u8, 244u8, 7u8,
							255u8, 93u8, 85u8, 138u8, 87u8, 165u8, 25u8, 154u8, 246u8, 135u8,
							210u8, 89u8, 170u8, 227u8, 236u8, 123u8, 161u8, 77u8, 214u8, 44u8,
							240u8,
						],
					)
				}
				#[doc = " The total number of contributors."]
				pub fn total_contributors(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"TotalContributors",
						vec![],
						[
							236u8, 88u8, 207u8, 169u8, 18u8, 55u8, 31u8, 213u8, 140u8, 154u8,
							142u8, 214u8, 66u8, 114u8, 157u8, 35u8, 172u8, 205u8, 122u8, 169u8,
							45u8, 64u8, 132u8, 177u8, 180u8, 21u8, 208u8, 12u8, 20u8, 23u8, 13u8,
							30u8,
						],
					)
				}
				#[doc = " The timestamp at which the users are able to claim their rewards."]
				pub fn vesting_time_start(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"VestingTimeStart",
						vec![],
						[
							93u8, 101u8, 112u8, 233u8, 17u8, 239u8, 82u8, 207u8, 167u8, 62u8,
							181u8, 104u8, 114u8, 195u8, 132u8, 255u8, 106u8, 152u8, 75u8, 200u8,
							76u8, 193u8, 89u8, 137u8, 224u8, 62u8, 225u8, 206u8, 157u8, 28u8,
							126u8, 48u8,
						],
					)
				}
				#[doc = " Associations of reward accounts to remote accounts."]
				pub fn associations(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"Associations",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							85u8, 12u8, 50u8, 120u8, 143u8, 116u8, 152u8, 188u8, 100u8, 72u8, 80u8,
							64u8, 16u8, 169u8, 122u8, 10u8, 221u8, 178u8, 231u8, 78u8, 151u8, 31u8,
							216u8, 254u8, 118u8, 243u8, 237u8, 37u8, 127u8, 238u8, 206u8, 101u8,
						],
					)
				}
				#[doc = " Associations of reward accounts to remote accounts."]
				pub fn associations_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
							[::core::primitive::u8; 32usize],
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"Associations",
						Vec::new(),
						[
							85u8, 12u8, 50u8, 120u8, 143u8, 116u8, 152u8, 188u8, 100u8, 72u8, 80u8,
							64u8, 16u8, 169u8, 122u8, 10u8, 221u8, 178u8, 231u8, 78u8, 151u8, 31u8,
							216u8, 254u8, 118u8, 243u8, 237u8, 37u8, 127u8, 238u8, 206u8, 101u8,
						],
					)
				}
				#[doc = " If set, new locks will not be added to claims"]
				pub fn remove_reward_locks(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CrowdloanRewards",
						"RemoveRewardLocks",
						vec![],
						[
							88u8, 210u8, 233u8, 161u8, 138u8, 199u8, 210u8, 0u8, 71u8, 237u8,
							189u8, 204u8, 252u8, 44u8, 191u8, 207u8, 81u8, 76u8, 220u8, 222u8,
							13u8, 236u8, 71u8, 55u8, 224u8, 246u8, 57u8, 31u8, 58u8, 191u8, 158u8,
							13u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The upfront liquidity unlocked at first claim."]
				pub fn initial_payment(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"InitialPayment",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}
				#[doc = " The percentage of excess funds required to trigger the `OverFunded` event."]
				pub fn over_funded_threshold(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"OverFundedThreshold",
						[
							225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
							80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
							177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
						],
					)
				}
				#[doc = " The time you have to wait to unlock another part of your reward."]
				pub fn vesting_step(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"VestingStep",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " The arbitrary prefix used for the proof."]
				pub fn prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"Prefix",
						[
							106u8, 50u8, 57u8, 116u8, 43u8, 202u8, 37u8, 248u8, 102u8, 22u8, 62u8,
							22u8, 242u8, 54u8, 152u8, 168u8, 107u8, 64u8, 72u8, 172u8, 124u8, 40u8,
							42u8, 110u8, 104u8, 145u8, 31u8, 144u8, 242u8, 189u8, 145u8, 208u8,
						],
					)
				}
				#[doc = " The unique identifier of this pallet."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"PalletId",
						[
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
						],
					)
				}
				#[doc = " The unique identifier for locks maintained by this pallet."]
				pub fn lock_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 8usize]>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"LockId",
						[
							224u8, 197u8, 247u8, 125u8, 62u8, 180u8, 69u8, 91u8, 226u8, 36u8, 82u8,
							148u8, 70u8, 147u8, 209u8, 40u8, 210u8, 229u8, 181u8, 191u8, 170u8,
							205u8, 138u8, 97u8, 127u8, 59u8, 124u8, 244u8, 252u8, 30u8, 213u8,
							179u8,
						],
					)
				}
				#[doc = " If claimed amounts should be locked by the pallet"]
				pub fn lock_by_default(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"LockByDefault",
						[
							165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
							252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
							100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
						],
					)
				}
				#[doc = " The AccountId of this pallet."]
				pub fn account_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CrowdloanRewards",
						"account_id",
						[
							167u8, 71u8, 0u8, 47u8, 217u8, 107u8, 29u8, 163u8, 157u8, 187u8, 110u8,
							219u8, 88u8, 213u8, 82u8, 107u8, 46u8, 199u8, 41u8, 110u8, 102u8,
							187u8, 45u8, 201u8, 247u8, 66u8, 33u8, 228u8, 33u8, 99u8, 242u8, 80u8,
						],
					)
				}
			}
		}
	}
	pub mod assets {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferNative {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransferNative {
				pub source: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAll {
				pub asset: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAllNative {
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MintInitialize {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MintInitializeWithGovernance {
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
				pub governance_origin: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MintInto {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BurnFrom {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub dest: ::subxt::utils::MultiAddress<
					::subxt::utils::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer `amount` of `asset` from `origin` to `dest`."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not signed."]
				#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
				#[doc = "   cannot be respected."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer",
						Transfer { asset, dest, amount, keep_alive },
						[
							191u8, 249u8, 227u8, 177u8, 227u8, 30u8, 137u8, 210u8, 170u8, 186u8,
							138u8, 181u8, 23u8, 51u8, 178u8, 172u8, 107u8, 134u8, 163u8, 172u8,
							190u8, 202u8, 127u8, 160u8, 205u8, 98u8, 205u8, 39u8, 15u8, 68u8,
							165u8, 80u8,
						],
					)
				}
				#[doc = "Transfer `amount` of the native asset from `origin` to `dest`. This is slightly"]
				#[doc = "cheaper to call, as it avoids an asset lookup."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not signed."]
				#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
				#[doc = "   cannot be respected."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn transfer_native(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferNative> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_native",
						TransferNative { dest, value, keep_alive },
						[
							203u8, 255u8, 186u8, 102u8, 209u8, 83u8, 227u8, 118u8, 11u8, 209u8,
							70u8, 190u8, 67u8, 158u8, 173u8, 231u8, 41u8, 137u8, 127u8, 209u8,
							160u8, 160u8, 59u8, 226u8, 154u8, 116u8, 108u8, 210u8, 87u8, 108u8,
							141u8, 18u8,
						],
					)
				}
				#[doc = "Transfer `amount` of the `asset` from `origin` to `dest`. This requires root."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not root."]
				#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
				#[doc = "   cannot be respected."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn force_transfer(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_transfer",
						ForceTransfer { asset, source, dest, value, keep_alive },
						[
							123u8, 143u8, 36u8, 52u8, 57u8, 12u8, 209u8, 44u8, 106u8, 69u8, 200u8,
							38u8, 79u8, 3u8, 59u8, 128u8, 242u8, 132u8, 83u8, 22u8, 13u8, 7u8,
							185u8, 221u8, 193u8, 73u8, 242u8, 55u8, 109u8, 194u8, 15u8, 163u8,
						],
					)
				}
				#[doc = "Transfer `amount` of the the native asset from `origin` to `dest`. This requires root."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not root."]
				#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
				#[doc = "   cannot be respected."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn force_transfer_native(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransferNative> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"force_transfer_native",
						ForceTransferNative { source, dest, value, keep_alive },
						[
							109u8, 219u8, 2u8, 9u8, 154u8, 57u8, 173u8, 220u8, 132u8, 248u8, 31u8,
							203u8, 185u8, 230u8, 252u8, 89u8, 92u8, 152u8, 87u8, 44u8, 21u8, 209u8,
							202u8, 159u8, 229u8, 5u8, 156u8, 252u8, 219u8, 9u8, 138u8, 135u8,
						],
					)
				}
				#[doc = "Transfer all free balance of the `asset` from `origin` to `dest`."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not signed."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn transfer_all(
					&self,
					asset: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_all",
						TransferAll { asset, dest, keep_alive },
						[
							252u8, 242u8, 56u8, 229u8, 110u8, 245u8, 215u8, 78u8, 248u8, 237u8,
							202u8, 143u8, 219u8, 104u8, 121u8, 75u8, 53u8, 234u8, 134u8, 214u8,
							73u8, 250u8, 151u8, 124u8, 247u8, 60u8, 230u8, 36u8, 26u8, 222u8,
							240u8, 108u8,
						],
					)
				}
				#[doc = "Transfer all free balance of the native asset from `origin` to `dest`."]
				#[doc = ""]
				#[doc = "# Errors"]
				#[doc = " - When `origin` is not signed."]
				#[doc = " - If the `dest` cannot be looked up."]
				pub fn transfer_all_native(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAllNative> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"transfer_all_native",
						TransferAllNative { dest, keep_alive },
						[
							199u8, 166u8, 244u8, 2u8, 74u8, 109u8, 252u8, 7u8, 251u8, 242u8, 80u8,
							154u8, 164u8, 73u8, 144u8, 79u8, 83u8, 188u8, 208u8, 23u8, 127u8, 19u8,
							234u8, 226u8, 111u8, 93u8, 176u8, 171u8, 178u8, 132u8, 74u8, 63u8,
						],
					)
				}
				#[doc = "Creates a new asset, minting `amount` of funds into the `dest` account. Intended to be"]
				#[doc = "used for creating wrapped assets, not associated with any project."]
				pub fn mint_initialize(
					&self,
					amount: ::core::primitive::u128,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<MintInitialize> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"mint_initialize",
						MintInitialize { amount, dest },
						[
							46u8, 118u8, 244u8, 196u8, 195u8, 185u8, 222u8, 58u8, 151u8, 155u8,
							118u8, 131u8, 134u8, 226u8, 8u8, 155u8, 76u8, 98u8, 92u8, 157u8, 133u8,
							62u8, 166u8, 172u8, 200u8, 39u8, 11u8, 184u8, 87u8, 73u8, 62u8, 36u8,
						],
					)
				}
				#[doc = "Creates a new asset, minting `amount` of funds into the `dest` account. The `dest`"]
				#[doc = "account can use the democracy pallet to mint further assets, or if the governance_origin"]
				#[doc = "is set to an owned account, using signed transactions. In general the"]
				#[doc = "`governance_origin` should be generated from the pallet id."]
				pub fn mint_initialize_with_governance(
					&self,
					amount: ::core::primitive::u128,
					governance_origin: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::StaticTxPayload<MintInitializeWithGovernance> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"mint_initialize_with_governance",
						MintInitializeWithGovernance { amount, governance_origin, dest },
						[
							112u8, 237u8, 174u8, 228u8, 234u8, 128u8, 152u8, 223u8, 18u8, 220u8,
							251u8, 233u8, 136u8, 177u8, 214u8, 237u8, 151u8, 115u8, 86u8, 68u8,
							220u8, 98u8, 98u8, 101u8, 94u8, 55u8, 195u8, 248u8, 233u8, 20u8, 186u8,
							45u8,
						],
					)
				}
				#[doc = "Mints `amount` of `asset_id` into the `dest` account."]
				pub fn mint_into(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<MintInto> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"mint_into",
						MintInto { asset_id, dest, amount },
						[
							67u8, 51u8, 185u8, 110u8, 243u8, 173u8, 151u8, 175u8, 141u8, 214u8,
							194u8, 139u8, 176u8, 25u8, 49u8, 248u8, 121u8, 103u8, 178u8, 128u8,
							5u8, 52u8, 66u8, 232u8, 182u8, 57u8, 192u8, 55u8, 136u8, 90u8, 60u8,
							32u8,
						],
					)
				}
				#[doc = "Burns `amount` of `asset_id` into the `dest` account."]
				pub fn burn_from(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<BurnFrom> {
					::subxt::tx::StaticTxPayload::new(
						"Assets",
						"burn_from",
						BurnFrom { asset_id, dest, amount },
						[
							97u8, 142u8, 84u8, 209u8, 163u8, 111u8, 93u8, 46u8, 152u8, 84u8, 142u8,
							82u8, 3u8, 128u8, 43u8, 26u8, 148u8, 160u8, 230u8, 48u8, 239u8, 34u8,
							174u8, 88u8, 52u8, 149u8, 146u8, 77u8, 139u8, 31u8, 225u8, 102u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn native_asset_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::primitives::currency::CurrencyId,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Assets",
						"NativeAssetId",
						[
							150u8, 207u8, 49u8, 178u8, 254u8, 209u8, 81u8, 36u8, 235u8, 117u8,
							62u8, 166u8, 4u8, 173u8, 64u8, 189u8, 19u8, 182u8, 131u8, 166u8, 234u8,
							145u8, 83u8, 23u8, 246u8, 20u8, 47u8, 34u8, 66u8, 162u8, 146u8, 49u8,
						],
					)
				}
			}
		}
	}
	pub mod governance_registry {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Set {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub value: ::subxt::utils::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct GrantRoot {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Remove {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the value of an `asset_id` to the signed account id. Only callable by root."]
				pub fn set(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					value: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<Set> {
					::subxt::tx::StaticTxPayload::new(
						"GovernanceRegistry",
						"set",
						Set { asset_id, value },
						[
							38u8, 114u8, 100u8, 157u8, 78u8, 228u8, 102u8, 210u8, 55u8, 88u8, 52u8,
							18u8, 121u8, 34u8, 119u8, 225u8, 203u8, 174u8, 141u8, 25u8, 51u8,
							148u8, 40u8, 130u8, 222u8, 173u8, 49u8, 174u8, 41u8, 106u8, 3u8, 4u8,
						],
					)
				}
				#[doc = "Sets the value of an `asset_id` to root. Only callable by root."]
				pub fn grant_root(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::StaticTxPayload<GrantRoot> {
					::subxt::tx::StaticTxPayload::new(
						"GovernanceRegistry",
						"grant_root",
						GrantRoot { asset_id },
						[
							108u8, 52u8, 102u8, 38u8, 197u8, 192u8, 164u8, 84u8, 15u8, 112u8,
							103u8, 138u8, 152u8, 15u8, 42u8, 241u8, 242u8, 96u8, 189u8, 37u8, 49u8,
							133u8, 70u8, 97u8, 144u8, 98u8, 115u8, 78u8, 151u8, 73u8, 45u8, 71u8,
						],
					)
				}
				#[doc = "Removes mapping of an `asset_id`. Only callable by root."]
				pub fn remove(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::tx::StaticTxPayload<Remove> {
					::subxt::tx::StaticTxPayload::new(
						"GovernanceRegistry",
						"remove",
						Remove { asset_id },
						[
							44u8, 136u8, 86u8, 109u8, 250u8, 117u8, 227u8, 246u8, 40u8, 114u8,
							204u8, 219u8, 208u8, 23u8, 103u8, 113u8, 249u8, 99u8, 199u8, 86u8,
							25u8, 165u8, 160u8, 59u8, 187u8, 67u8, 192u8, 111u8, 177u8, 80u8,
							254u8, 69u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_governance_registry::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Set {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub value: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Set {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "Set";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct GrantRoot {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for GrantRoot {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "GrantRoot";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Remove {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::events::StaticEvent for Remove {
				const PALLET: &'static str = "GovernanceRegistry";
				const EVENT: &'static str = "Remove";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn origins_by_asset_id(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_traits::governance::SignedRawOrigin<
							::subxt::utils::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"GovernanceRegistry",
						"OriginsByAssetId",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							188u8, 140u8, 55u8, 118u8, 91u8, 24u8, 34u8, 167u8, 239u8, 89u8, 100u8,
							77u8, 158u8, 53u8, 227u8, 26u8, 137u8, 42u8, 189u8, 220u8, 210u8, 52u8,
							4u8, 178u8, 33u8, 226u8, 30u8, 231u8, 168u8, 115u8, 189u8, 238u8,
						],
					)
				}
				pub fn origins_by_asset_id_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::composable_traits::governance::SignedRawOrigin<
							::subxt::utils::AccountId32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"GovernanceRegistry",
						"OriginsByAssetId",
						Vec::new(),
						[
							188u8, 140u8, 55u8, 118u8, 91u8, 24u8, 34u8, 167u8, 239u8, 89u8, 100u8,
							77u8, 158u8, 53u8, 227u8, 26u8, 137u8, 42u8, 189u8, 220u8, 210u8, 52u8,
							4u8, 178u8, 33u8, 226u8, 30u8, 231u8, 168u8, 115u8, 189u8, 238u8,
						],
					)
				}
			}
		}
	}
	pub mod call_filter {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Disable {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Enable {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Disable a pallet function."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the sender must be"]
				#[doc = "`DisableOrigin`."]
				#[doc = ""]
				#[doc = "Possibly emits a `Disabled` event."]
				pub fn disable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::StaticTxPayload<Disable> {
					::subxt::tx::StaticTxPayload::new(
						"CallFilter",
						"disable",
						Disable { entry },
						[
							135u8, 13u8, 52u8, 184u8, 86u8, 89u8, 77u8, 78u8, 232u8, 107u8, 230u8,
							67u8, 122u8, 192u8, 193u8, 4u8, 59u8, 44u8, 175u8, 209u8, 194u8, 49u8,
							73u8, 116u8, 232u8, 227u8, 56u8, 254u8, 72u8, 54u8, 206u8, 27u8,
						],
					)
				}
				#[doc = "Enable a previously disabled pallet function."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the sender must be"]
				#[doc = "`EnableOrigin`."]
				#[doc = ""]
				#[doc = "Possibly emits an `Enabled` event."]
				pub fn enable(
					&self,
					entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
						runtime_types::common::MaxStringSize,
					>,
				) -> ::subxt::tx::StaticTxPayload<Enable> {
					::subxt::tx::StaticTxPayload::new(
						"CallFilter",
						"enable",
						Enable { entry },
						[
							24u8, 54u8, 83u8, 13u8, 223u8, 77u8, 229u8, 162u8, 164u8, 107u8, 208u8,
							132u8, 0u8, 252u8, 176u8, 125u8, 236u8, 185u8, 128u8, 209u8, 252u8,
							116u8, 112u8, 242u8, 25u8, 76u8, 69u8, 22u8, 4u8, 205u8, 227u8, 207u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_call_filter::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Paused transaction"]
			pub struct Disabled {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			impl ::subxt::events::StaticEvent for Disabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Disabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Unpaused transaction"]
			pub struct Enabled {
				pub entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
					runtime_types::common::MaxStringSize,
				>,
			}
			impl ::subxt::events::StaticEvent for Enabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Enabled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The list of disabled extrinsics."]
				pub fn disabled_calls(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CallFilter",
						"DisabledCalls",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							54u8, 93u8, 201u8, 230u8, 97u8, 19u8, 95u8, 130u8, 213u8, 155u8, 135u8,
							208u8, 52u8, 55u8, 238u8, 157u8, 6u8, 135u8, 46u8, 136u8, 82u8, 53u8,
							1u8, 182u8, 38u8, 172u8, 140u8, 63u8, 56u8, 88u8, 173u8, 16u8,
						],
					)
				}
				#[doc = " The list of disabled extrinsics."]
				pub fn disabled_calls_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"CallFilter",
						"DisabledCalls",
						Vec::new(),
						[
							54u8, 93u8, 201u8, 230u8, 97u8, 19u8, 95u8, 130u8, 213u8, 155u8, 135u8,
							208u8, 52u8, 55u8, 238u8, 157u8, 6u8, 135u8, 46u8, 136u8, 82u8, 53u8,
							1u8, 182u8, 38u8, 172u8, 140u8, 63u8, 56u8, 88u8, 173u8, 16u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn max_string_size(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"CallFilter",
						"MaxStringSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod ibc {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Deliver {
				pub messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub amount: ::core::primitive::u128,
				pub memo:
					::core::option::Option<runtime_types::composable_runtime::ibc::MemoMessage>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetParams {
				pub params: runtime_types::pallet_ibc::PalletParams,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpgradeClient {
				pub params: runtime_types::pallet_ibc::UpgradeParams,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FreezeClient {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn deliver(
					&self,
					messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any>,
				) -> ::subxt::tx::StaticTxPayload<Deliver> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"deliver",
						Deliver { messages },
						[
							179u8, 205u8, 83u8, 66u8, 171u8, 103u8, 175u8, 57u8, 35u8, 60u8, 170u8,
							172u8, 60u8, 57u8, 56u8, 226u8, 130u8, 222u8, 121u8, 25u8, 230u8,
							143u8, 253u8, 77u8, 111u8, 152u8, 89u8, 150u8, 129u8, 239u8, 141u8,
							61u8,
						],
					)
				}
				pub fn transfer(
					&self,
					params: runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
					memo: ::core::option::Option<
						runtime_types::composable_runtime::ibc::MemoMessage,
					>,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"transfer",
						Transfer { params, asset_id, amount, memo },
						[
							104u8, 136u8, 36u8, 50u8, 105u8, 241u8, 120u8, 243u8, 74u8, 48u8,
							173u8, 124u8, 56u8, 78u8, 20u8, 193u8, 27u8, 73u8, 16u8, 127u8, 125u8,
							113u8, 70u8, 115u8, 43u8, 2u8, 103u8, 109u8, 208u8, 40u8, 10u8, 36u8,
						],
					)
				}
				pub fn set_params(
					&self,
					params: runtime_types::pallet_ibc::PalletParams,
				) -> ::subxt::tx::StaticTxPayload<SetParams> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"set_params",
						SetParams { params },
						[
							116u8, 243u8, 44u8, 94u8, 198u8, 240u8, 175u8, 200u8, 234u8, 175u8,
							193u8, 228u8, 45u8, 51u8, 89u8, 123u8, 211u8, 209u8, 214u8, 0u8, 124u8,
							86u8, 142u8, 43u8, 104u8, 198u8, 156u8, 224u8, 51u8, 82u8, 220u8,
							165u8,
						],
					)
				}
				#[doc = "We write the consensus & client state under these predefined paths so that"]
				#[doc = "we can produce state proofs of the values to connected chains"]
				#[doc = "in order to execute client upgrades."]
				pub fn upgrade_client(
					&self,
					params: runtime_types::pallet_ibc::UpgradeParams,
				) -> ::subxt::tx::StaticTxPayload<UpgradeClient> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"upgrade_client",
						UpgradeClient { params },
						[
							113u8, 38u8, 218u8, 101u8, 66u8, 187u8, 155u8, 238u8, 185u8, 82u8,
							16u8, 26u8, 35u8, 122u8, 0u8, 124u8, 239u8, 54u8, 134u8, 255u8, 40u8,
							224u8, 3u8, 49u8, 200u8, 214u8, 212u8, 165u8, 224u8, 19u8, 11u8, 28u8,
						],
					)
				}
				#[doc = "Freeze a client at a specific height"]
				pub fn freeze_client(
					&self,
					client_id: ::std::vec::Vec<::core::primitive::u8>,
					height: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<FreezeClient> {
					::subxt::tx::StaticTxPayload::new(
						"Ibc",
						"freeze_client",
						FreezeClient { client_id, height },
						[
							71u8, 208u8, 79u8, 70u8, 132u8, 43u8, 127u8, 140u8, 240u8, 38u8, 18u8,
							3u8, 50u8, 179u8, 10u8, 210u8, 28u8, 174u8, 60u8, 81u8, 229u8, 211u8,
							120u8, 55u8, 109u8, 191u8, 182u8, 207u8, 37u8, 52u8, 224u8, 239u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_ibc::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Events emitted by the ibc subsystem"]
			pub struct Events {
				pub events: ::std::vec::Vec<
					::core::result::Result<
						runtime_types::pallet_ibc::events::IbcEvent,
						runtime_types::pallet_ibc::errors::IbcError,
					>,
				>,
			}
			impl ::subxt::events::StaticEvent for Events {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "Events";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An Ibc token transfer has been started"]
			pub struct TokenTransferInitiated {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferInitiated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferInitiated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A channel has been opened"]
			pub struct ChannelOpened {
				pub channel_id: ::std::vec::Vec<::core::primitive::u8>,
				pub port_id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ChannelOpened {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ChannelOpened";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Pallet params updated"]
			pub struct ParamsUpdated {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for ParamsUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ParamsUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An outgoing Ibc token transfer has been completed and burnt"]
			pub struct TokenTransferCompleted {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferCompleted {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Ibc tokens have been received and minted"]
			pub struct TokenReceived {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_receiver_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenReceived {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Ibc transfer failed, received an acknowledgement error, tokens have been refunded"]
			pub struct TokenTransferFailed {
				pub from: ::std::vec::Vec<::core::primitive::u8>,
				pub to: ::std::vec::Vec<::core::primitive::u8>,
				pub ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
				pub local_asset_id:
					::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
				pub amount: ::core::primitive::u128,
				pub is_sender_source: ::core::primitive::bool,
				pub source_channel: ::std::vec::Vec<::core::primitive::u8>,
				pub destination_channel: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for TokenTransferFailed {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "TokenTransferFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "On recv packet was not processed successfully processes"]
			pub struct OnRecvPacketError {
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for OnRecvPacketError {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "OnRecvPacketError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Client upgrade path has been set"]
			pub struct ClientUpgradeSet;
			impl ::subxt::events::StaticEvent for ClientUpgradeSet {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientUpgradeSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Client has been frozen"]
			pub struct ClientFrozen {
				pub client_id: ::std::vec::Vec<::core::primitive::u8>,
				pub height: ::core::primitive::u64,
				pub revision_number: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ClientFrozen {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "ClientFrozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Asset Admin Account Updated"]
			pub struct AssetAdminUpdated {
				pub admin_account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for AssetAdminUpdated {
				const PALLET: &'static str = "Ibc";
				const EVENT: &'static str = "AssetAdminUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " client_id , Height => Height"]
				pub fn client_update_height(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateHeight",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							169u8, 6u8, 192u8, 186u8, 79u8, 156u8, 202u8, 105u8, 213u8, 28u8,
							186u8, 112u8, 216u8, 170u8, 8u8, 166u8, 181u8, 179u8, 111u8, 212u8,
							35u8, 121u8, 7u8, 86u8, 212u8, 69u8, 66u8, 3u8, 19u8, 220u8, 114u8,
							167u8,
						],
					)
				}
				#[doc = " client_id , Height => Height"]
				pub fn client_update_height_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateHeight",
						Vec::new(),
						[
							169u8, 6u8, 192u8, 186u8, 79u8, 156u8, 202u8, 105u8, 213u8, 28u8,
							186u8, 112u8, 216u8, 170u8, 8u8, 166u8, 181u8, 179u8, 111u8, 212u8,
							35u8, 121u8, 7u8, 86u8, 212u8, 69u8, 66u8, 3u8, 19u8, 220u8, 114u8,
							167u8,
						],
					)
				}
				#[doc = " client_id , Height => Timestamp"]
				pub fn client_update_time(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateTime",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							98u8, 194u8, 46u8, 221u8, 34u8, 111u8, 178u8, 66u8, 21u8, 234u8, 174u8,
							27u8, 188u8, 45u8, 219u8, 211u8, 68u8, 207u8, 23u8, 228u8, 175u8,
							165u8, 179u8, 18u8, 219u8, 248u8, 34u8, 60u8, 202u8, 106u8, 171u8,
							68u8,
						],
					)
				}
				#[doc = " client_id , Height => Timestamp"]
				pub fn client_update_time_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientUpdateTime",
						Vec::new(),
						[
							98u8, 194u8, 46u8, 221u8, 34u8, 111u8, 178u8, 66u8, 21u8, 234u8, 174u8,
							27u8, 188u8, 45u8, 219u8, 211u8, 68u8, 207u8, 23u8, 228u8, 175u8,
							165u8, 179u8, 18u8, 219u8, 248u8, 34u8, 60u8, 202u8, 106u8, 171u8,
							68u8,
						],
					)
				}
				pub fn channel_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ChannelCounter",
						vec![],
						[
							227u8, 20u8, 185u8, 41u8, 83u8, 61u8, 150u8, 45u8, 251u8, 243u8, 199u8,
							188u8, 94u8, 160u8, 194u8, 25u8, 245u8, 89u8, 69u8, 105u8, 37u8, 220u8,
							143u8, 106u8, 244u8, 161u8, 215u8, 129u8, 220u8, 79u8, 193u8, 255u8,
						],
					)
				}
				pub fn packet_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"PacketCounter",
						vec![],
						[
							90u8, 180u8, 164u8, 48u8, 0u8, 236u8, 78u8, 205u8, 206u8, 248u8, 91u8,
							28u8, 64u8, 96u8, 73u8, 159u8, 230u8, 81u8, 41u8, 88u8, 165u8, 107u8,
							85u8, 85u8, 56u8, 240u8, 122u8, 230u8, 165u8, 216u8, 232u8, 223u8,
						],
					)
				}
				#[doc = " connection_identifier => Vec<(port_id, channel_id)>"]
				pub fn channels_connection(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ChannelsConnection",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
							218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
							56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
						],
					)
				}
				#[doc = " connection_identifier => Vec<(port_id, channel_id)>"]
				pub fn channels_connection_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ChannelsConnection",
						Vec::new(),
						[
							175u8, 74u8, 214u8, 39u8, 82u8, 72u8, 28u8, 110u8, 105u8, 136u8, 218u8,
							218u8, 110u8, 111u8, 182u8, 21u8, 180u8, 80u8, 66u8, 44u8, 85u8, 138u8,
							56u8, 102u8, 121u8, 201u8, 111u8, 240u8, 73u8, 7u8, 8u8, 115u8,
						],
					)
				}
				#[doc = " counter for clients"]
				pub fn client_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ClientCounter",
						vec![],
						[
							236u8, 121u8, 82u8, 20u8, 184u8, 116u8, 197u8, 237u8, 123u8, 236u8,
							221u8, 90u8, 88u8, 89u8, 224u8, 171u8, 143u8, 145u8, 221u8, 242u8,
							195u8, 89u8, 31u8, 185u8, 251u8, 92u8, 250u8, 50u8, 47u8, 171u8, 31u8,
							124u8,
						],
					)
				}
				#[doc = " counter for clients"]
				pub fn connection_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConnectionCounter",
						vec![],
						[
							155u8, 106u8, 125u8, 78u8, 71u8, 212u8, 217u8, 208u8, 192u8, 39u8,
							220u8, 52u8, 226u8, 76u8, 191u8, 4u8, 196u8, 118u8, 136u8, 3u8, 90u8,
							155u8, 168u8, 89u8, 103u8, 155u8, 220u8, 150u8, 4u8, 118u8, 164u8, 2u8,
						],
					)
				}
				#[doc = " counter for acknowledgments"]
				pub fn acknowledgement_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"AcknowledgementCounter",
						vec![],
						[
							169u8, 90u8, 79u8, 11u8, 28u8, 186u8, 46u8, 204u8, 147u8, 76u8, 77u8,
							162u8, 45u8, 137u8, 52u8, 50u8, 67u8, 212u8, 51u8, 49u8, 156u8, 128u8,
							213u8, 182u8, 158u8, 71u8, 152u8, 162u8, 250u8, 196u8, 71u8, 170u8,
						],
					)
				}
				#[doc = " counter for packet receipts"]
				pub fn packet_receipt_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"PacketReceiptCounter",
						vec![],
						[
							149u8, 146u8, 199u8, 15u8, 127u8, 62u8, 135u8, 23u8, 188u8, 232u8,
							218u8, 239u8, 194u8, 219u8, 28u8, 34u8, 21u8, 153u8, 149u8, 155u8,
							26u8, 39u8, 50u8, 201u8, 88u8, 36u8, 177u8, 239u8, 55u8, 51u8, 141u8,
							241u8,
						],
					)
				}
				#[doc = " client_id => Vec<Connection_id>"]
				pub fn connection_client(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConnectionClient",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							134u8, 166u8, 43u8, 43u8, 142u8, 200u8, 83u8, 81u8, 252u8, 1u8, 153u8,
							167u8, 197u8, 170u8, 154u8, 242u8, 241u8, 178u8, 166u8, 147u8, 223u8,
							188u8, 118u8, 48u8, 40u8, 203u8, 29u8, 17u8, 120u8, 250u8, 79u8, 111u8,
						],
					)
				}
				#[doc = " client_id => Vec<Connection_id>"]
				pub fn connection_client_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConnectionClient",
						Vec::new(),
						[
							134u8, 166u8, 43u8, 43u8, 142u8, 200u8, 83u8, 81u8, 252u8, 1u8, 153u8,
							167u8, 197u8, 170u8, 154u8, 242u8, 241u8, 178u8, 166u8, 147u8, 223u8,
							188u8, 118u8, 48u8, 40u8, 203u8, 29u8, 17u8, 120u8, 250u8, 79u8, 111u8,
						],
					)
				}
				#[doc = " Pallet Params used to disable sending or receipt of ibc tokens"]
				pub fn params(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_ibc::PalletParams>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"Params",
						vec![],
						[
							53u8, 220u8, 56u8, 9u8, 21u8, 121u8, 177u8, 62u8, 240u8, 196u8, 215u8,
							157u8, 220u8, 38u8, 85u8, 220u8, 196u8, 38u8, 44u8, 236u8, 64u8, 11u8,
							242u8, 82u8, 230u8, 33u8, 60u8, 148u8, 35u8, 176u8, 81u8, 188u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_asset_ids(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::primitives::currency::CurrencyId>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcAssetIds",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							95u8, 163u8, 91u8, 54u8, 240u8, 186u8, 53u8, 241u8, 234u8, 178u8,
							228u8, 71u8, 139u8, 33u8, 124u8, 205u8, 97u8, 75u8, 125u8, 55u8, 234u8,
							37u8, 128u8, 129u8, 119u8, 196u8, 227u8, 212u8, 43u8, 154u8, 153u8,
							214u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (T::AssetId, Vec<u8>)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_asset_ids_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcAssetIds",
						Vec::new(),
						[
							95u8, 163u8, 91u8, 54u8, 240u8, 186u8, 53u8, 241u8, 234u8, 178u8,
							228u8, 71u8, 139u8, 33u8, 124u8, 205u8, 97u8, 75u8, 125u8, 55u8, 234u8,
							37u8, 128u8, 129u8, 119u8, 196u8, 227u8, 212u8, 43u8, 154u8, 153u8,
							214u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_ibc_asset_ids(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"CounterForIbcAssetIds",
						vec![],
						[
							115u8, 253u8, 221u8, 253u8, 101u8, 232u8, 174u8, 9u8, 2u8, 79u8, 212u8,
							243u8, 233u8, 90u8, 34u8, 251u8, 140u8, 100u8, 153u8, 60u8, 240u8,
							60u8, 36u8, 90u8, 81u8, 31u8, 241u8, 224u8, 157u8, 89u8, 194u8, 105u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_denoms(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::primitives::currency::CurrencyId,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcDenoms",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							66u8, 43u8, 17u8, 209u8, 96u8, 87u8, 219u8, 181u8, 26u8, 207u8, 178u8,
							232u8, 121u8, 119u8, 194u8, 108u8, 240u8, 228u8, 95u8, 246u8, 247u8,
							223u8, 55u8, 66u8, 128u8, 110u8, 200u8, 161u8, 164u8, 229u8, 205u8,
							8u8,
						],
					)
				}
				#[doc = " Map of asset id to ibc denom pairs (Vec<u8>, T::AssetId)"]
				#[doc = " ibc denoms represented as utf8 string bytes"]
				pub fn ibc_denoms_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::primitives::currency::CurrencyId,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"IbcDenoms",
						Vec::new(),
						[
							66u8, 43u8, 17u8, 209u8, 96u8, 87u8, 219u8, 181u8, 26u8, 207u8, 178u8,
							232u8, 121u8, 119u8, 194u8, 108u8, 240u8, 228u8, 95u8, 246u8, 247u8,
							223u8, 55u8, 66u8, 128u8, 110u8, 200u8, 161u8, 164u8, 229u8, 205u8,
							8u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_ibc_denoms(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"CounterForIbcDenoms",
						vec![],
						[
							254u8, 185u8, 194u8, 13u8, 31u8, 147u8, 250u8, 253u8, 56u8, 226u8,
							58u8, 135u8, 7u8, 228u8, 50u8, 135u8, 175u8, 249u8, 5u8, 148u8, 42u8,
							24u8, 125u8, 212u8, 245u8, 134u8, 213u8, 102u8, 17u8, 2u8, 135u8,
							194u8,
						],
					)
				}
				#[doc = " ChannelIds open from this module"]
				pub fn channel_ids(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ChannelIds",
						vec![],
						[
							21u8, 104u8, 164u8, 90u8, 17u8, 181u8, 235u8, 0u8, 67u8, 67u8, 164u8,
							217u8, 126u8, 131u8, 214u8, 38u8, 143u8, 134u8, 215u8, 173u8, 53u8,
							154u8, 118u8, 215u8, 175u8, 207u8, 14u8, 134u8, 241u8, 215u8, 188u8,
							69u8,
						],
					)
				}
				#[doc = " Active Escrow addresses"]
				pub fn escrow_addresses(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::utils::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"EscrowAddresses",
						vec![],
						[
							184u8, 122u8, 32u8, 42u8, 200u8, 130u8, 61u8, 220u8, 100u8, 177u8,
							62u8, 197u8, 90u8, 210u8, 142u8, 56u8, 70u8, 70u8, 59u8, 246u8, 203u8,
							118u8, 32u8, 237u8, 123u8, 115u8, 73u8, 67u8, 175u8, 199u8, 167u8,
							25u8,
						],
					)
				}
				#[doc = " Consensus heights"]
				#[doc = " Stored as a tuple of (revision_number, revision_height)"]
				pub fn consensus_heights(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_btree_set::BoundedBTreeSet<
							runtime_types::ibc::core::ics02_client::height::Height,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConsensusHeights",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							238u8, 213u8, 231u8, 8u8, 158u8, 84u8, 100u8, 101u8, 78u8, 142u8,
							125u8, 133u8, 128u8, 92u8, 138u8, 184u8, 144u8, 221u8, 58u8, 101u8,
							206u8, 217u8, 140u8, 30u8, 206u8, 26u8, 242u8, 223u8, 113u8, 46u8,
							227u8, 240u8,
						],
					)
				}
				#[doc = " Consensus heights"]
				#[doc = " Stored as a tuple of (revision_number, revision_height)"]
				pub fn consensus_heights_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_core::bounded::bounded_btree_set::BoundedBTreeSet<
							runtime_types::ibc::core::ics02_client::height::Height,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ibc",
						"ConsensusHeights",
						Vec::new(),
						[
							238u8, 213u8, 231u8, 8u8, 158u8, 84u8, 100u8, 101u8, 78u8, 142u8,
							125u8, 133u8, 128u8, 92u8, 138u8, 184u8, 144u8, 221u8, 58u8, 101u8,
							206u8, 217u8, 140u8, 30u8, 206u8, 26u8, 242u8, 223u8, 113u8, 46u8,
							227u8, 240u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The native asset id, this will use the `NativeCurrency` for all operations."]
				pub fn native_asset_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::primitives::currency::CurrencyId,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"NativeAssetId",
						[
							150u8, 207u8, 49u8, 178u8, 254u8, 209u8, 81u8, 36u8, 235u8, 117u8,
							62u8, 166u8, 4u8, 173u8, 64u8, 189u8, 19u8, 182u8, 131u8, 166u8, 234u8,
							145u8, 83u8, 23u8, 246u8, 20u8, 47u8, 34u8, 66u8, 162u8, 146u8, 49u8,
						],
					)
				}
				#[doc = " Prefix for events stored in the Off-chain DB via Indexing API, child trie and connection"]
				pub fn pallet_prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"PalletPrefix",
						[
							106u8, 50u8, 57u8, 116u8, 43u8, 202u8, 37u8, 248u8, 102u8, 22u8, 62u8,
							22u8, 242u8, 54u8, 152u8, 168u8, 107u8, 64u8, 72u8, 172u8, 124u8, 40u8,
							42u8, 110u8, 104u8, 145u8, 31u8, 144u8, 242u8, 189u8, 145u8, 208u8,
						],
					)
				}
				#[doc = " Light client protocol this chain is operating"]
				pub fn light_client_protocol(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_ibc::LightClientProtocol,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"LightClientProtocol",
						[
							200u8, 116u8, 16u8, 82u8, 41u8, 118u8, 80u8, 243u8, 53u8, 143u8, 22u8,
							2u8, 167u8, 246u8, 7u8, 151u8, 169u8, 50u8, 102u8, 67u8, 255u8, 148u8,
							204u8, 202u8, 89u8, 187u8, 48u8, 204u8, 36u8, 113u8, 253u8, 204u8,
						],
					)
				}
				#[doc = " Expected block time in milliseconds"]
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " Minimum connection delay period in seconds for ibc connections that can be created or"]
				#[doc = " accepted. Ensure that this is non-zero in production as it's a critical vulnerability."]
				pub fn minimum_connection_delay(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"MinimumConnectionDelay",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " Amount to be reserved for client and connection creation"]
				pub fn spam_protection_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Ibc",
						"SpamProtectionDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod common {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MaxStringSize;
		}
		pub mod composable_runtime {
			use super::runtime_types;
			pub mod ibc {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct MemoMessage;
			}
			pub mod opaque {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct SessionKeys {
					pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>),
				#[codec(index = 72)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 74)]
				ReleaseCommittee(
					runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 41)]
				RelayerXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec(index = 6)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 11)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec(index = 20)]
				Authorship(runtime_types::pallet_authorship::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Call),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 41)]
				RelayerXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Call),
				#[codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Call),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Call),
				#[codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Call),
				#[codec(index = 56)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Call),
				#[codec(index = 57)]
				Assets(runtime_types::pallet_assets::pallet::Call),
				#[codec(index = 58)]
				GovernanceRegistry(runtime_types::pallet_governance_registry::pallet::Call),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Call),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 4)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 72)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 73)]
				TechnicalCommitteeMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 74)]
				ReleaseCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 75)]
				ReleaseMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 36)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 37)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 41)]
				RelayerXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 44)]
				XTokens(runtime_types::orml_xtokens::module::Event),
				#[codec(index = 45)]
				UnknownTokens(runtime_types::orml_unknown_tokens::module::Event),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Event),
				#[codec(index = 53)]
				CurrencyFactory(runtime_types::pallet_currency_factory::pallet::Event),
				#[codec(index = 56)]
				CrowdloanRewards(runtime_types::pallet_crowdloan_rewards::pallet::Event),
				#[codec(index = 58)]
				GovernanceRegistry(runtime_types::pallet_governance_registry::pallet::Event),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Event),
				#[codec(index = 190)]
				Ibc(runtime_types::pallet_ibc::pallet::Event),
			}
		}
		pub mod composable_support {
			use super::runtime_types;
			pub mod collections {
				use super::runtime_types;
				pub mod vec {
					use super::runtime_types;
					pub mod bounded {
						use super::runtime_types;
						pub mod bi_bounded_vec {
							use super::runtime_types;
							#[derive(
								:: subxt :: ext :: codec :: Decode,
								:: subxt :: ext :: codec :: Encode,
								Debug,
							)]
							pub struct BiBoundedVec<_0> {
								pub inner: ::std::vec::Vec<_0>,
							}
						}
					}
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct EcdsaSignature(pub [::core::primitive::u8; 65usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
			}
		}
		pub mod composable_traits {
			use super::runtime_types;
			pub mod account_proxy {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum ProxyType {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Governance,
					#[codec(index = 2)]
					CancelProxy,
					#[codec(index = 3)]
					Bridge,
				}
			}
			pub mod assets {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BasicAssetMetadata { pub symbol : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , pub name : runtime_types :: composable_support :: collections :: vec :: bounded :: bi_bounded_vec :: BiBoundedVec < :: core :: primitive :: u8 > , }
			}
			pub mod governance {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum SignedRawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
				}
			}
		}
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Service a single overweight message."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
					#[doc = "- `index`: The index of the overweight message to service."]
					#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `Unknown`: Message of `index` is unknown."]
					#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
					#[doc = ""]
					#[doc = "Events:"]
					#[doc = "- `OverweightServiced`: On success."]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The message index given is unknown."]
					Unknown,
					#[codec(index = 1)]
					#[doc = "The amount of weight given is possibly not enough for executing the message."]
					OverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v2::traits::Outcome,
					},
					#[codec(index = 3)]
					#[doc = "The weight limit for handling downward messages was reached."]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					#[doc = "Downward message is overweight and was placed in the overweight queue."]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 5)]
					#[doc = "Downward message from the overweight queue was executed."]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PageIndexData {
				pub begin_used: ::core::primitive::u32,
				pub end_used: ::core::primitive::u32,
				pub overweight_count: ::core::primitive::u64,
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: utils :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to upgrade validation function while existing upgrade pending"]
					OverlappingUpgrades,
					#[codec(index = 1)]
					#[doc = "Polkadot currently prohibits this parachain from upgrading its validation function"]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					#[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
					#[doc = "willing to run"]
					TooBig,
					#[codec(index = 3)]
					#[doc = "The inherent which supplies the validation data did not run this block"]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					#[doc = "The inherent which supplies the host configuration did not run this block"]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					#[doc = "No validation function upgrade is currently scheduled."]
					NotScheduled,
					#[codec(index = 6)]
					#[doc = "No code upgrade has been authorized."]
					NothingAuthorized,
					#[codec(index = 7)]
					#[doc = "The given code upgrade has not been authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The validation function has been scheduled to apply."]
					ValidationFunctionStored,
					#[codec(index = 1)]
					#[doc = "The validation function was applied as of the contained relay chain block number."]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "The relay-chain aborted the upgrade process."]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					#[doc = "An upgrade has been authorized."]
					UpgradeAuthorized { code_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "Some downward messages have been received and will be processed."]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Downward messages were processed using the given weight."]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::utils::H256,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct MessagingStateSnapshot {
					pub dmq_mqc_head: ::subxt::utils::H256,
					pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
					pub ingress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
					)>,
					pub egress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
					)>,
				}
			}
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 8usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 8usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 8usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Origin {
					#[codec(index = 0)]
					Relay,
					#[codec(index = 1)]
					SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Services a single overweight XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
					#[doc = "- `index`: The index of the overweight XCM to service"]
					#[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
					#[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
					#[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
					#[doc = ""]
					#[doc = "Events:"]
					#[doc = "- `OverweightServiced`: On success."]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					suspend_xcm_execution,
					#[codec(index = 2)]
					#[doc = "Resumes all XCM executions for the XCMP queue."]
					#[doc = ""]
					#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					resume_xcm_execution,
					#[codec(index = 3)]
					#[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
					#[doc = "suspend their sending."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
					#[doc = "messages from the channel."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
					#[doc = "message sending may recommence after it has been suspended."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
					update_threshold_weight { new: ::core::primitive::u64 },
					#[codec(index = 7)]
					#[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
					#[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
					update_weight_restrict_decay { new: ::core::primitive::u64 },
					#[codec(index = 8)]
					#[doc = "Overwrite the maximum amount of weight any individual message may consume."]
					#[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
					update_xcmp_max_individual_weight { new: ::core::primitive::u64 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to send XCM message."]
					FailedToSend,
					#[codec(index = 1)]
					#[doc = "Bad XCM origin."]
					BadXcmOrigin,
					#[codec(index = 2)]
					#[doc = "Bad XCM data."]
					BadXcm,
					#[codec(index = 3)]
					#[doc = "Bad overweight index."]
					BadOverweightIndex,
					#[codec(index = 4)]
					#[doc = "Provided weight is possibly not enough to execute the message."]
					WeightOverLimit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Some XCM was executed ok."]
					Success {
						message_hash: ::core::option::Option<::subxt::utils::H256>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					#[doc = "Some XCM failed."]
					Fail {
						message_hash: ::core::option::Option<::subxt::utils::H256>,
						error: runtime_types::xcm::v2::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Bad XCM version used."]
					BadVersion { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 3)]
					#[doc = "Bad XCM format used."]
					BadFormat { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 4)]
					#[doc = "An upward message was sent to the relay chain."]
					UpwardMessageSent { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 5)]
					#[doc = "An HRMP message was sent to a sibling parachain."]
					XcmpMessageSent { message_hash: ::core::option::Option<::subxt::utils::H256> },
					#[codec(index = 6)]
					#[doc = "An XCM exceeded the individual message weight budget."]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 7)]
					#[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum InboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub weight_restrict_decay: runtime_types::sp_weights::weight_v2::Weight,
				pub xcmp_max_individual_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct MessageQueueChain(pub ::subxt::utils::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v2::PersistedValidationData<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
				pub horizontal_messages: ::subxt::utils::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
					>,
				>,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod preimages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Bounded<_0> {
						#[codec(index = 0)]
						Legacy {
							hash: ::subxt::utils::H256,
						},
						#[codec(index = 1)]
						Inline(
							runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Lookup {
							hash: ::subxt::utils::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Debug,
						)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`"]
					#[doc = "# </weight>"]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Set the new runtime code."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
					#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
					#[doc = "  expensive)."]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
					#[doc = "expensive. We will treat this as a full block."]
					#[doc = "# </weight>"]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C)` where `C` length of `code`"]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
					#[doc = "block. # </weight>"]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "Kill some items from storage."]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "Make some on-chain remark and emit event."]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod ibc {
			use super::runtime_types;
			pub mod core {
				use super::runtime_types;
				pub mod ics02_client {
					use super::runtime_types;
					pub mod height {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Debug,
						)]
						pub struct Height {
							pub revision_number: ::core::primitive::u64,
							pub revision_height: ::core::primitive::u64,
						}
					}
				}
			}
		}
		pub mod ibc_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Timeout {
				#[codec(index = 0)]
				Offset {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
				#[codec(index = 1)]
				Absolute {
					timestamp: ::core::option::Option<::core::primitive::u64>,
					height: ::core::option::Option<::core::primitive::u64>,
				},
			}
		}
		pub mod orml_tokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "It will decrease the total issuance of the system by the"]
					#[doc = "`TransferFee`. If the sender's account is below the existential"]
					#[doc = "deposit as a result of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
					transfer {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Transfer all remaining balance to the given account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_"]
					#[doc = "balances. This means that any locked, reserved, or existential"]
					#[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
					#[doc = "this function. To ensure that this function results in a killed"]
					#[doc = "account, you might need to prepare the account by removing any"]
					#[doc = "reference counters, storage deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
					#[doc = "  operation should send all of the funds the account has, causing"]
					#[doc = "  the sender account to be killed (false), or transfer everything"]
					#[doc = "  except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
					#[doc = "will not kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Exactly as `transfer`, except the origin must be root and the source"]
					#[doc = "account may be specified."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `source`: The sender of the transfer."]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
					force_transfer {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
					#[doc = "will also decrease the total issuance of the system"]
					#[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
					#[doc = "existential deposit, it will reap the `AccountInfo`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					set_balance {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The balance is too low"]
					BalanceTooLow,
					#[codec(index = 1)]
					#[doc = "Cannot convert Amount into Balance type"]
					AmountIntoBalanceFailed,
					#[codec(index = 2)]
					#[doc = "Failed because liquidity restrictions due to locking"]
					LiquidityRestrictions,
					#[codec(index = 3)]
					#[doc = "Failed because the maximum locks was exceeded"]
					MaxLocksExceeded,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below"]
					#[doc = "ExistentialDeposit, resulting in an outright loss."]
					DustLost {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some reserved balance was repatriated (moved from reserved to"]
					#[doc = "another account)."]
					ReserveRepatriated {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 6)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "The total issuance of an currency has been set"]
					TotalIssuanceSet {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some balances were withdrawn (e.g. pay for transaction fee)"]
					Withdrawn {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some balances were slashed (e.g. due to mis-behavior)"]
					Slashed {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						free_amount: ::core::primitive::u128,
						reserved_amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Deposited some balance into an account"]
					Deposited {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Some funds are locked"]
					LockSet {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Some locked funds were unlocked"]
					LockRemoved {
						lock_id: [::core::primitive::u8; 8usize],
						currency_id: runtime_types::primitives::currency::CurrencyId,
						who: ::subxt::utils::AccountId32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub frozen: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod orml_unknown_tokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The balance is too low."]
					BalanceTooLow,
					#[codec(index = 1)]
					#[doc = "The operation will cause balance to overflow."]
					BalanceOverflow,
					#[codec(index = 2)]
					#[doc = "Unhandled asset."]
					UnhandledAsset,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Deposit success."]
					Deposited {
						asset: runtime_types::xcm::v1::multiasset::MultiAsset,
						who: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 1)]
					#[doc = "Withdraw success."]
					Withdrawn {
						asset: runtime_types::xcm::v1::multiasset::MultiAsset,
						who: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
				}
			}
		}
		pub mod orml_xtokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer native currencies."]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 1)]
					#[doc = "Transfer `MultiAsset`."]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer_multiasset {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 2)]
					#[doc = "Transfer native currencies specifying the fee and amount as"]
					#[doc = "separate."]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "`fee` is the amount to be spent to pay for execution in destination"]
					#[doc = "chain. Both fee and amount will be subtracted form the callers"]
					#[doc = "balance."]
					#[doc = ""]
					#[doc = "If `fee` is not high enough to cover for the execution costs in the"]
					#[doc = "destination chain, then the assets will be trapped in the"]
					#[doc = "destination chain"]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer_with_fee {
						currency_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						fee: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 3)]
					#[doc = "Transfer `MultiAsset` specifying the fee and amount as separate."]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "`fee` is the multiasset to be spent to pay for execution in"]
					#[doc = "destination chain. Both fee and amount will be subtracted form the"]
					#[doc = "callers balance For now we only accept fee and asset having the same"]
					#[doc = "`MultiLocation` id."]
					#[doc = ""]
					#[doc = "If `fee` is not high enough to cover for the execution costs in the"]
					#[doc = "destination chain, then the assets will be trapped in the"]
					#[doc = "destination chain"]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer_multiasset_with_fee {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 4)]
					#[doc = "Transfer several currencies specifying the item to be used as fee"]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "`fee_item` is index of the currencies tuple that we want to use for"]
					#[doc = "payment"]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer_multicurrencies {
						currencies: ::std::vec::Vec<(
							runtime_types::primitives::currency::CurrencyId,
							::core::primitive::u128,
						)>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 5)]
					#[doc = "Transfer several `MultiAsset` specifying the item to be used as fee"]
					#[doc = ""]
					#[doc = "`dest_weight_limit` is the weight for XCM execution on the dest"]
					#[doc = "chain, and it would be charged from the transferred assets. If set"]
					#[doc = "below requirements, the execution may fail and assets wouldn't be"]
					#[doc = "received."]
					#[doc = ""]
					#[doc = "`fee_item` is index of the MultiAssets that we want to use for"]
					#[doc = "payment"]
					#[doc = ""]
					#[doc = "It's a no-op if any error on local XCM execution or message sending."]
					#[doc = "Note sending assets out per se doesn't guarantee they would be"]
					#[doc = "received. Receiving depends on if the XCM message could be delivered"]
					#[doc = "by the network, and if the receiving chain would handle"]
					#[doc = "messages correctly."]
					transfer_multiassets {
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Asset has no reserve location."]
					AssetHasNoReserve,
					#[codec(index = 1)]
					#[doc = "Not cross-chain transfer."]
					NotCrossChainTransfer,
					#[codec(index = 2)]
					#[doc = "Invalid transfer destination."]
					InvalidDest,
					#[codec(index = 3)]
					#[doc = "Currency is not cross-chain transferable."]
					NotCrossChainTransferableCurrency,
					#[codec(index = 4)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 5)]
					#[doc = "XCM execution failed."]
					XcmExecutionFailed,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the"]
					#[doc = "destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Could not get ancestry of asset reserve location."]
					InvalidAncestry,
					#[codec(index = 8)]
					#[doc = "The MultiAsset is invalid."]
					InvalidAsset,
					#[codec(index = 9)]
					#[doc = "The destination `MultiLocation` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 10)]
					#[doc = "The version of the `Versioned` value used is not able to be"]
					#[doc = "interpreted."]
					BadVersion,
					#[codec(index = 11)]
					#[doc = "We tried sending distinct asset and fee but they have different"]
					#[doc = "reserve chains."]
					DistinctReserveForAssetAndFee,
					#[codec(index = 12)]
					#[doc = "The fee is zero."]
					ZeroFee,
					#[codec(index = 13)]
					#[doc = "The transfering asset amount is zero."]
					ZeroAmount,
					#[codec(index = 14)]
					#[doc = "The number of assets to be sent is over the maximum."]
					TooManyAssetsBeingSent,
					#[codec(index = 15)]
					#[doc = "The specified index does not exist in a MultiAssets struct."]
					AssetIndexNonExistent,
					#[codec(index = 16)]
					#[doc = "Fee is not enough."]
					FeeNotEnough,
					#[codec(index = 17)]
					#[doc = "Not supported MultiLocation"]
					NotSupportedMultiLocation,
					#[codec(index = 18)]
					#[doc = "MinXcmFee not registered for certain reserve location"]
					MinXcmFeeNotDefined,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Transferred `MultiAsset` with fee."]
					TransferredMultiAssets {
						sender: ::subxt::utils::AccountId32,
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						fee: runtime_types::xcm::v1::multiasset::MultiAsset,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
				}
			}
		}
		pub mod pallet_assets {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer `amount` of `asset` from `origin` to `dest`."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not signed."]
					#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
					#[doc = "   cannot be respected."]
					#[doc = " - If the `dest` cannot be looked up."]
					transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					#[doc = "Transfer `amount` of the native asset from `origin` to `dest`. This is slightly"]
					#[doc = "cheaper to call, as it avoids an asset lookup."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not signed."]
					#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
					#[doc = "   cannot be respected."]
					#[doc = " - If the `dest` cannot be looked up."]
					transfer_native {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "Transfer `amount` of the `asset` from `origin` to `dest`. This requires root."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not root."]
					#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
					#[doc = "   cannot be respected."]
					#[doc = " - If the `dest` cannot be looked up."]
					force_transfer {
						asset: runtime_types::primitives::currency::CurrencyId,
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 3)]
					#[doc = "Transfer `amount` of the the native asset from `origin` to `dest`. This requires root."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not root."]
					#[doc = " - If the account has insufficient free balance to make the transfer, or if `keep_alive`"]
					#[doc = "   cannot be respected."]
					#[doc = " - If the `dest` cannot be looked up."]
					force_transfer_native {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Transfer all free balance of the `asset` from `origin` to `dest`."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not signed."]
					#[doc = " - If the `dest` cannot be looked up."]
					transfer_all {
						asset: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Transfer all free balance of the native asset from `origin` to `dest`."]
					#[doc = ""]
					#[doc = "# Errors"]
					#[doc = " - When `origin` is not signed."]
					#[doc = " - If the `dest` cannot be looked up."]
					transfer_all_native {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 6)]
					#[doc = "Creates a new asset, minting `amount` of funds into the `dest` account. Intended to be"]
					#[doc = "used for creating wrapped assets, not associated with any project."]
					mint_initialize {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 7)]
					#[doc = "Creates a new asset, minting `amount` of funds into the `dest` account. The `dest`"]
					#[doc = "account can use the democracy pallet to mint further assets, or if the governance_origin"]
					#[doc = "is set to an owned account, using signed transactions. In general the"]
					#[doc = "`governance_origin` should be generated from the pallet id."]
					mint_initialize_with_governance {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						governance_origin: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 8)]
					#[doc = "Mints `amount` of `asset_id` into the `dest` account."]
					mint_into {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Burns `amount` of `asset_id` into the `dest` account."]
					burn_from {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					CannotSetNewCurrencyToRegistry,
					#[codec(index = 1)]
					InvalidCurrency,
				}
			}
		}
		pub mod pallet_authorship {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Provide a set of uncles."]
					set_uncles {
						new_uncles: ::std::vec::Vec<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The uncle parent not in the chain."]
					InvalidUncleParent,
					#[codec(index = 1)]
					#[doc = "Uncles already set in the block."]
					UnclesAlreadySet,
					#[codec(index = 2)]
					#[doc = "Too many uncles."]
					TooManyUncles,
					#[codec(index = 3)]
					#[doc = "The uncle is genesis."]
					GenesisUncle,
					#[codec(index = 4)]
					#[doc = "The uncle is too high in chain."]
					TooHighUncle,
					#[codec(index = 5)]
					#[doc = "The uncle is already included."]
					UncleAlreadyIncluded,
					#[codec(index = 6)]
					#[doc = "The uncle isn't recent enough to be included."]
					OldUncle,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum UncleEntryItem<_0, _1, _2> {
				#[codec(index = 0)]
				InclusionHeight(_0),
				#[codec(index = 1)]
				Uncle(_1, ::core::option::Option<_2>),
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
					#[doc = "  types. See related functions below."]
					#[doc = "- It contains a limited number of reads and writes internally and no complex"]
					#[doc = "  computation."]
					#[doc = ""]
					#[doc = "Related functions:"]
					#[doc = ""]
					#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
					#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
					#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
					#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
					#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
					#[doc = "    that the transfer will not kill the origin account."]
					#[doc = "---------------------------------"]
					#[doc = "- Origin account is already in memory, so no DB operations for them."]
					#[doc = "# </weight>"]
					transfer {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
					#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
					#[doc = "If the new free or reserved balance is below the existential deposit,"]
					#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					set_balance {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
					#[doc = "specified."]
					#[doc = "# <weight>"]
					#[doc = "- Same as transfer, but additional read and write because the source account is not"]
					#[doc = "  assumed to be in the overlay."]
					#[doc = "# </weight>"]
					force_transfer {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
					#[doc = "origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true). # <weight>"]
					#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
					#[doc = "  #</weight>"]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value"]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal"]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account"]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed MaxReserves"]
					TooManyReserves,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Reasons {
				#[codec(index = 0)]
				Fee,
				#[codec(index = 1)]
				Misc,
				#[codec(index = 2)]
				All,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_call_filter {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Disable a pallet function."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the sender must be"]
					#[doc = "`DisableOrigin`."]
					#[doc = ""]
					#[doc = "Possibly emits a `Disabled` event."]
					disable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Enable a previously disabled pallet function."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the sender must be"]
					#[doc = "`EnableOrigin`."]
					#[doc = ""]
					#[doc = "Possibly emits an `Enabled` event."]
					enable {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "We tried to disable an extrinsic that cannot be disabled."]
					CannotDisable,
					#[codec(index = 1)]
					#[doc = "The pallet name is not a valid UTF8 string."]
					InvalidString,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Paused transaction"]
					Disabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Unpaused transaction"]
					Enabled {
						entry: runtime_types::pallet_call_filter::types::CallFilterEntry<
							runtime_types::common::MaxStringSize,
						>,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CallFilterEntry<_0> {
					pub pallet_name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub function_name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the list of invulnerable (fixed) collators."]
					set_invulnerables { new: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					#[doc = "Set the ideal number of collators (not including the invulnerables)."]
					#[doc = "If lowering this number, then the number of running collators could be higher than this figure."]
					#[doc = "Aside from that edge case, there should be no other way to have more collators than the desired number."]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Set the candidacy bond amount."]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					#[doc = "Register this account as a collator candidate. The account must (a) already have"]
					#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					register_as_candidate,
					#[codec(index = 4)]
					#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
					#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
					#[doc = ""]
					#[doc = "This call will fail if the total number of candidates would drop below `MinCandidates`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					leave_intent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many candidates"]
					TooManyCandidates,
					#[codec(index = 1)]
					#[doc = "Too few candidates"]
					TooFewCandidates,
					#[codec(index = 2)]
					#[doc = "Unknown error"]
					Unknown,
					#[codec(index = 3)]
					#[doc = "Permission issue"]
					Permission,
					#[codec(index = 4)]
					#[doc = "User is already a candidate"]
					AlreadyCandidate,
					#[codec(index = 5)]
					#[doc = "User is not a candidate"]
					NotCandidate,
					#[codec(index = 6)]
					#[doc = "Too many invulnerables"]
					TooManyInvulnerables,
					#[codec(index = 7)]
					#[doc = "User is already an Invulnerable"]
					AlreadyInvulnerable,
					#[codec(index = 8)]
					#[doc = "Account has no associated validator ID"]
					NoAssociatedValidatorId,
					#[codec(index = 9)]
					#[doc = "Validator ID is not yet registered"]
					ValidatorNotRegistered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					NewInvulnerables { invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					NewDesiredCandidates { desired_candidates: ::core::primitive::u32 },
					#[codec(index = 2)]
					NewCandidacyBond { bond_amount: ::core::primitive::u128 },
					#[codec(index = 3)]
					CandidateAdded {
						account_id: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					CandidateRemoved { account_id: ::subxt::utils::AccountId32 },
				}
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the collective's membership."]
					#[doc = ""]
					#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
					#[doc = "- `prime`: The prime member whose vote sets the default."]
					#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
					#[doc = "  weight estimation."]
					#[doc = ""]
					#[doc = "Requires root origin."]
					#[doc = ""]
					#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
					#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
					#[doc = ""]
					#[doc = "# WARNING:"]
					#[doc = ""]
					#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
					#[doc = "implementation of the trait [`ChangeMembers`]."]
					#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
					#[doc = "with other logic managing the member set."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(MP + N)` where:"]
					#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
					#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
					#[doc = "  - `P` proposals-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
					#[doc = "    members"]
					#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
					#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
					#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
					#[doc = "# </weight>"]
					set_members {
						new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
						prime: ::core::option::Option<::subxt::utils::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Dispatch a proposal from a member using the `Member` origin."]
					#[doc = ""]
					#[doc = "Origin must be a member of the collective."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
					#[doc = "  `proposal`"]
					#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					execute {
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Add a new proposal to either be voted on or executed directly."]
					#[doc = ""]
					#[doc = "Requires the sender to be member."]
					#[doc = ""]
					#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
					#[doc = "or put up for voting."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - branching is influenced by `threshold` where:"]
					#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
					#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
					#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
					#[doc = "  - DB accesses influenced by `threshold`:"]
					#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
					#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
					#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
					#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
					#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
					#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
					#[doc = "  - 1 event"]
					#[doc = "# </weight>"]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Add an aye or nay vote for the sender to the given proposal."]
					#[doc = ""]
					#[doc = "Requires the sender to be a member."]
					#[doc = ""]
					#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
					#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
					#[doc = "fee."]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
					#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					vote {
						proposal: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
					#[doc = ""]
					#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
					#[doc = ""]
					#[doc = "If called before the end of the voting period it will only close the vote if it is"]
					#[doc = "has enough votes to be approved or disapproved."]
					#[doc = ""]
					#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
					#[doc = "unless there is a prime member set and the prime member cast an approval."]
					#[doc = ""]
					#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
					#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
					#[doc = ""]
					#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
					#[doc = "proposal."]
					#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
					#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1 + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - `P1` is the complexity of `proposal` preimage."]
					#[doc = "  - `P2` is proposal-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
					#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
					#[doc = "   `O(P2)`)"]
					#[doc = " - any mutations done while executing `proposal` (`P1`)"]
					#[doc = "- up to 3 events"]
					#[doc = "# </weight>"]
					close_old_weight {
						proposal_hash: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						proposal_weight_bound: runtime_types::sp_weights::OldWeight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
					#[doc = "state."]
					#[doc = ""]
					#[doc = "Must be called by the Root origin."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Complexity: O(P) where P is the number of max proposals"]
					#[doc = "DB Weight:"]
					#[doc = "* Reads: Proposals"]
					#[doc = "* Writes: Voting, Proposals, ProposalOf"]
					#[doc = "# </weight>"]
					disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 6)]
					#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
					#[doc = ""]
					#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
					#[doc = ""]
					#[doc = "If called before the end of the voting period it will only close the vote if it is"]
					#[doc = "has enough votes to be approved or disapproved."]
					#[doc = ""]
					#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
					#[doc = "unless there is a prime member set and the prime member cast an approval."]
					#[doc = ""]
					#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
					#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
					#[doc = ""]
					#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
					#[doc = "proposal."]
					#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
					#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1 + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - `P1` is the complexity of `proposal` preimage."]
					#[doc = "  - `P2` is proposal-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
					#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
					#[doc = "   `O(P2)`)"]
					#[doc = " - any mutations done while executing `proposal` (`P1`)"]
					#[doc = "- up to 3 events"]
					#[doc = "# </weight>"]
					close {
						proposal_hash: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Account is not a member"]
					NotMember,
					#[codec(index = 1)]
					#[doc = "Duplicate proposals not allowed"]
					DuplicateProposal,
					#[codec(index = 2)]
					#[doc = "Proposal must exist"]
					ProposalMissing,
					#[codec(index = 3)]
					#[doc = "Mismatched index"]
					WrongIndex,
					#[codec(index = 4)]
					#[doc = "Duplicate vote ignored"]
					DuplicateVote,
					#[codec(index = 5)]
					#[doc = "Members are already initialized!"]
					AlreadyInitialized,
					#[codec(index = 6)]
					#[doc = "The close call was made too early, before the end of the voting."]
					TooEarly,
					#[codec(index = 7)]
					#[doc = "There can only be a maximum of `MaxProposals` active proposals."]
					TooManyProposals,
					#[codec(index = 8)]
					#[doc = "The given weight bound for the proposal was too low."]
					WrongProposalWeight,
					#[codec(index = 9)]
					#[doc = "The given length bound for the proposal was too low."]
					WrongProposalLength,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
					#[doc = "`MemberCount`)."]
					Proposed {
						account: ::subxt::utils::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::utils::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A motion (given hash) has been voted on by given account, leaving"]
					#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
					Voted {
						account: ::subxt::utils::AccountId32,
						proposal_hash: ::subxt::utils::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "A motion was approved by the required threshold."]
					Approved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "A motion was not approved by the required threshold."]
					Disapproved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
					Executed {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
					MemberExecuted {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
					Closed {
						proposal_hash: ::subxt::utils::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Votes<_0, _1> {
				pub index: _1,
				pub threshold: _1,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_crowdloan_rewards {
			use super::runtime_types;
			pub mod models {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Proof<_0> {
					#[codec(index = 0)]
					RelayChain(_0, runtime_types::sp_runtime::MultiSignature),
					#[codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EcdsaSignature),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum RemoteAccount<_0> {
					#[codec(index = 0)]
					RelayChain(_0),
					#[codec(index = 1)]
					Ethereum(runtime_types::composable_support::types::EthereumAddress),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Reward<_0, _1> {
					pub total: _0,
					pub claimed: _0,
					pub vesting_period: _1,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Initialize the pallet at the current timestamp."]
					initialize,
					#[codec(index = 1)]
					#[doc = "Initialize the pallet at the given timestamp."]
					initialize_at { at: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Populate pallet by adding more rewards."]
					#[doc = ""]
					#[doc = "Each index in the rewards vector should contain: `remote_account`, `reward_account`,"]
					#[doc = "`vesting_period`."]
					#[doc = ""]
					#[doc = "Can be called multiple times. If an remote account"]
					#[doc = "already has a reward, it will be replaced by the new reward value."]
					#[doc = ""]
					#[doc = "Can only be called before `initialize`."]
					populate {
						rewards: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 3)]
					#[doc = "Associate a reward account. A valid proof has to be provided."]
					#[doc = "This call also claim the first reward (a.k.a. the first payment, which is a % of the"]
					#[doc = "vested reward)."]
					#[doc = "If logic gate pass, no fees are applied."]
					#[doc = ""]
					#[doc = "The proof should be:"]
					#[doc = "```haskell"]
					#[doc = "proof = sign (concat prefix (hex reward_account))"]
					#[doc = "```"]
					associate {
						reward_account: ::subxt::utils::AccountId32,
						proof: runtime_types::pallet_crowdloan_rewards::models::Proof<
							[::core::primitive::u8; 32usize],
						>,
					},
					#[codec(index = 4)]
					#[doc = "Claim a reward from the associated reward account."]
					#[doc = "A previous call to `associate` should have been made."]
					#[doc = "If logic gate pass, no fees are applied."]
					claim,
					#[codec(index = 5)]
					unlock_rewards_for {
						reward_accounts: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 6)]
					#[doc = "Adds all accounts in the `additions` vector. Add may be called even if the pallet has"]
					#[doc = "been initialized."]
					add {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					NotInitialized,
					#[codec(index = 1)]
					AlreadyInitialized,
					#[codec(index = 2)]
					BackToTheFuture,
					#[codec(index = 3)]
					RewardsNotFunded,
					#[codec(index = 4)]
					InvalidProof,
					#[codec(index = 5)]
					InvalidClaim,
					#[codec(index = 6)]
					NothingToClaim,
					#[codec(index = 7)]
					NotAssociated,
					#[codec(index = 8)]
					AlreadyAssociated,
					#[codec(index = 9)]
					NotClaimableYet,
					#[codec(index = 10)]
					#[doc = "Returned by `delete` if the provided expected reward mismatches the actual reward."]
					UnexpectedRewardAmount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The crowdloan has been initialized or set to initialize at some time."]
					Initialized { at: ::core::primitive::u64 },
					#[codec(index = 1)]
					#[doc = "A claim has been made."]
					Claimed {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						reward_account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "A remote account has been associated with a reward account."]
					Associated {
						remote_account:
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						reward_account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "The crowdloan was successfully initialized, but with excess funds that won't be"]
					#[doc = "claimed."]
					OverFunded { excess_funds: ::core::primitive::u128 },
					#[codec(index = 4)]
					#[doc = "A portion of rewards have been unlocked and future claims will not have locks"]
					RewardsUnlocked { at: ::core::primitive::u64 },
					#[codec(index = 5)]
					#[doc = "Called after rewards have been added through the `add` extrinsic."]
					RewardsAdded {
						additions: ::std::vec::Vec<(
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
							::core::primitive::u128,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 6)]
					#[doc = "Called after rewards have been deleted through the `delete` extrinsic."]
					RewardsDeleted {
						deletions: ::std::vec::Vec<
							runtime_types::pallet_crowdloan_rewards::models::RemoteAccount<
								[::core::primitive::u8; 32usize],
							>,
						>,
					},
				}
			}
		}
		pub mod pallet_currency_factory {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					add_range { length: ::core::primitive::u64 },
					#[codec(index = 1)]
					#[doc = "Sets metadata"]
					set_metadata {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						metadata: runtime_types::composable_traits::assets::BasicAssetMetadata,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					AssetNotFound,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					RangeCreated {
						range: runtime_types::pallet_currency_factory::ranges::Range<
							runtime_types::primitives::currency::CurrencyId,
						>,
					},
				}
			}
			pub mod ranges {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Range<_0> {
					pub current: _0,
					pub end: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Ranges<_0> {
					pub ranges: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
						runtime_types::pallet_currency_factory::ranges::Range<_0>,
					>,
				}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Propose a sensitive action to be taken."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
					#[doc = "have funds to cover the deposit."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
					#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
					#[doc = ""]
					#[doc = "Emits `Proposed`."]
					propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Signals agreement with a particular proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
					#[doc = "must have funds to cover the deposit, equal to the original deposit."]
					#[doc = ""]
					#[doc = "- `proposal`: The index of the proposal to second."]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
					#[doc = "otherwise it is a vote to keep the status quo."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to vote for."]
					#[doc = "- `vote`: The vote configuration."]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
					#[doc = ""]
					#[doc = "-`ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					external_propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 5)]
					#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
					#[doc = "an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_majority {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 6)]
					#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
					#[doc = "schedule an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_default {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::composable_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 7)]
					#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
					#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
					#[doc = "but it is not a majority-carries referendum then it fails."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the current external proposal."]
					#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
					#[doc = "\tMust be always greater than zero."]
					#[doc = "\tFor `FastTrackOrigin` must be equal or greater than `FastTrackVotingPeriod`."]
					#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
					#[doc = "  enacted. This doesn't have a minimum amount."]
					#[doc = ""]
					#[doc = "Emits `Started`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					fast_track {
						proposal_hash: ::subxt::utils::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Veto and blacklist the external proposal hash."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
					#[doc = ""]
					#[doc = "Emits `Vetoed`."]
					#[doc = ""]
					#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
					veto_external { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 9)]
					#[doc = "Remove a referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "# Weight: `O(1)`."]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
					#[doc = ""]
					#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
					#[doc = "time appropriate for the conviction's lock period."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
					#[doc = "  - be delegating already; or"]
					#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
					#[doc = "    through `reap_vote` or `unvote`)."]
					#[doc = ""]
					#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
					#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
					#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
					#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
					#[doc = "  be more than the account's current balance."]
					#[doc = ""]
					#[doc = "Emits `Delegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					delegate {
						to: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Undelegate the voting power of the sending account."]
					#[doc = ""]
					#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
					#[doc = "of the conviction with which the delegation was issued."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
					#[doc = "currently delegating."]
					#[doc = ""]
					#[doc = "Emits `Undelegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					undelegate,
					#[codec(index = 12)]
					#[doc = "Clears all public proposals."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					clear_public_proposals,
					#[codec(index = 13)]
					#[doc = "Unlock tokens that have an expired lock."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account to remove the lock on."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` with R number of vote of target."]
					unlock {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If:"]
					#[doc = "- the referendum was cancelled, or"]
					#[doc = "- the referendum is ongoing, or"]
					#[doc = "- the referendum has ended such that"]
					#[doc = "  - the vote of the account was in opposition to the result; or"]
					#[doc = "  - there was no conviction to the account's vote; or"]
					#[doc = "  - the account made a split vote"]
					#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
					#[doc = "funds being available."]
					#[doc = ""]
					#[doc = "If, however, the referendum has ended and:"]
					#[doc = "- it finished corresponding to the vote of the account, and"]
					#[doc = "- the account made a standard vote with conviction, and"]
					#[doc = "- the lock period of the conviction is not over"]
					#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
					#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
					#[doc = "of both the amount locked and the time is it locked for)."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
					#[doc = "registered for referendum `index`."]
					#[doc = ""]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 15)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
					#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
					#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
					#[doc = "because the conviction period is over."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
					#[doc = "  referendum `index`."]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_other_vote {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 16)]
					#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
					#[doc = "proposed again."]
					#[doc = ""]
					#[doc = "If called on a queued public or external proposal, then this will result in it being"]
					#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
					#[doc = "then it will be cancelled."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
					#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
					#[doc = "cancelled."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
					#[doc = "  reasonable value)."]
					blacklist {
						proposal_hash: ::subxt::utils::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 17)]
					#[doc = "Remove a proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
					#[doc = ""]
					#[doc = "- `prop_index`: The index of the proposal to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Value too low"]
					ValueLow,
					#[codec(index = 1)]
					#[doc = "Proposal does not exist"]
					ProposalMissing,
					#[codec(index = 2)]
					#[doc = "Cannot cancel the same proposal twice"]
					AlreadyCanceled,
					#[codec(index = 3)]
					#[doc = "Proposal already made"]
					DuplicateProposal,
					#[codec(index = 4)]
					#[doc = "Proposal still blacklisted"]
					ProposalBlacklisted,
					#[codec(index = 5)]
					#[doc = "Next external proposal not simple majority"]
					NotSimpleMajority,
					#[codec(index = 6)]
					#[doc = "Invalid hash"]
					InvalidHash,
					#[codec(index = 7)]
					#[doc = "No external proposal"]
					NoProposal,
					#[codec(index = 8)]
					#[doc = "Identity may not veto a proposal twice"]
					AlreadyVetoed,
					#[codec(index = 9)]
					#[doc = "Vote given for invalid referendum"]
					ReferendumInvalid,
					#[codec(index = 10)]
					#[doc = "No proposals waiting"]
					NoneWaiting,
					#[codec(index = 11)]
					#[doc = "The given account did not vote on the referendum."]
					NotVoter,
					#[codec(index = 12)]
					#[doc = "The actor has no permission to conduct the action."]
					NoPermission,
					#[codec(index = 13)]
					#[doc = "The account is already delegating."]
					AlreadyDelegating,
					#[codec(index = 14)]
					#[doc = "Too high a balance was provided that the account cannot afford."]
					InsufficientFunds,
					#[codec(index = 15)]
					#[doc = "The account is not currently delegating."]
					NotDelegating,
					#[codec(index = 16)]
					#[doc = "The account currently has votes attached to it and the operation cannot succeed until"]
					#[doc = "these are removed, either through `unvote` or `reap_vote`."]
					VotesExist,
					#[codec(index = 17)]
					#[doc = "The instant referendum origin is currently disallowed."]
					InstantNotAllowed,
					#[codec(index = 18)]
					#[doc = "Delegation to oneself makes no sense."]
					Nonsense,
					#[codec(index = 19)]
					#[doc = "Invalid upper bound."]
					WrongUpperBound,
					#[codec(index = 20)]
					#[doc = "Maximum number of votes reached."]
					MaxVotesReached,
					#[codec(index = 21)]
					#[doc = "Maximum number of items reached."]
					TooMany,
					#[codec(index = 22)]
					#[doc = "Voting period too low"]
					VotingPeriodLow,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion has been proposed by a public account."]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "A public proposal has been tabled for referendum vote."]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "An external proposal has been tabled."]
					ExternalTabled,
					#[codec(index = 3)]
					#[doc = "A referendum has begun."]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					#[doc = "A proposal has been approved by referendum."]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "A proposal has been rejected by referendum."]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "A referendum has been cancelled."]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					#[doc = "An account has delegated their vote to another account."]
					Delegated {
						who: ::subxt::utils::AccountId32,
						target: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "An account has cancelled a previous delegation operation."]
					Undelegated { account: ::subxt::utils::AccountId32 },
					#[codec(index = 9)]
					#[doc = "An external proposal has been vetoed."]
					Vetoed {
						who: ::subxt::utils::AccountId32,
						proposal_hash: ::subxt::utils::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					#[doc = "A proposal_hash has been blacklisted permanently."]
					Blacklisted { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 11)]
					#[doc = "An account has voted in a referendum"]
					Voted {
						voter: ::subxt::utils::AccountId32,
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 12)]
					#[doc = "An account has secconded a proposal"]
					Seconded {
						seconder: ::subxt::utils::AccountId32,
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 13)]
					#[doc = "A proposal got canceled."]
					ProposalCanceled { prop_index: ::core::primitive::u32 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Voting<_0, _1, _2> {
					#[codec(index = 0)]
					Direct {
						votes: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec(index = 1)]
					Delegating {
						balance: _0,
						target: _1,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
				}
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
		}
		pub mod pallet_governance_registry {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the value of an `asset_id` to the signed account id. Only callable by root."]
					set {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						value: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					#[doc = "Sets the value of an `asset_id` to root. Only callable by root."]
					grant_root { asset_id: runtime_types::primitives::currency::CurrencyId },
					#[codec(index = 2)]
					#[doc = "Removes mapping of an `asset_id`. Only callable by root."]
					remove { asset_id: runtime_types::primitives::currency::CurrencyId },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Not found"]
					NoneError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					Set {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						value: ::subxt::utils::AccountId32,
					},
					#[codec(index = 1)]
					GrantRoot { asset_id: runtime_types::primitives::currency::CurrencyId },
					#[codec(index = 2)]
					Remove { asset_id: runtime_types::primitives::currency::CurrencyId },
				}
			}
		}
		pub mod pallet_ibc {
			use super::runtime_types;
			pub mod errors {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum IbcError {
					#[codec(index = 0)]
					Ics02Client { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					Ics03Connection { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					Ics04Channel { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					Ics20FungibleTokenTransfer { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					UnknownMessageTypeUrl { message: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					MalformedMessageBytes { message: ::std::vec::Vec<::core::primitive::u8> },
				}
			}
			pub mod events {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum IbcEvent {
					#[codec(index = 0)]
					NewBlock {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					CreateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					UpdateClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					UpgradeClient {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					ClientMisbehaviour {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						client_type: ::std::vec::Vec<::core::primitive::u8>,
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						consensus_height: ::core::primitive::u64,
						consensus_revision_number: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					OpenInitConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					OpenConfirmConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					OpenTryConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 8)]
					OpenAckConnection {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_connection_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						counterparty_client_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 9)]
					OpenInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 10)]
					OpenConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 11)]
					OpenTryChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 12)]
					OpenAckChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 13)]
					CloseInitChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 14)]
					CloseConfirmChannel {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						channel_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						connection_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_port_id: ::std::vec::Vec<::core::primitive::u8>,
						counterparty_channel_id:
							::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 15)]
					ReceivePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 16)]
					SendPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 17)]
					AcknowledgePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 18)]
					WriteAcknowledgement {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						dest_port: ::std::vec::Vec<::core::primitive::u8>,
						dest_channel: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					TimeoutPacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 20)]
					TimeoutOnClosePacket {
						revision_height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						sequence: ::core::primitive::u64,
					},
					#[codec(index = 21)]
					Empty,
					#[codec(index = 22)]
					ChainError,
					#[codec(index = 23)]
					AppModule {
						kind: ::std::vec::Vec<::core::primitive::u8>,
						module_id: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					deliver { messages: ::std::vec::Vec<runtime_types::pallet_ibc::Any> },
					#[codec(index = 1)]
					transfer {
						params:
							runtime_types::pallet_ibc::TransferParams<::subxt::utils::AccountId32>,
						asset_id: runtime_types::primitives::currency::CurrencyId,
						amount: ::core::primitive::u128,
						memo: ::core::option::Option<
							runtime_types::composable_runtime::ibc::MemoMessage,
						>,
					},
					#[codec(index = 2)]
					set_params { params: runtime_types::pallet_ibc::PalletParams },
					#[codec(index = 3)]
					#[doc = "We write the consensus & client state under these predefined paths so that"]
					#[doc = "we can produce state proofs of the values to connected chains"]
					#[doc = "in order to execute client upgrades."]
					upgrade_client { params: runtime_types::pallet_ibc::UpgradeParams },
					#[codec(index = 4)]
					#[doc = "Freeze a client at a specific height"]
					freeze_client {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Errors inform users that something went wrong."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Error processing ibc messages"]
					ProcessingError,
					#[codec(index = 1)]
					#[doc = "Error decoding some type"]
					DecodingError,
					#[codec(index = 2)]
					#[doc = "Error encoding some type"]
					EncodingError,
					#[codec(index = 3)]
					#[doc = "Error generating trie proof"]
					ProofGenerationError,
					#[codec(index = 4)]
					#[doc = "Client consensus state not found for height"]
					ConsensusStateNotFound,
					#[codec(index = 5)]
					#[doc = "Channel not found"]
					ChannelNotFound,
					#[codec(index = 6)]
					#[doc = "Client state not found"]
					ClientStateNotFound,
					#[codec(index = 7)]
					#[doc = "Connection not found"]
					ConnectionNotFound,
					#[codec(index = 8)]
					#[doc = "Packet commitment wasn't found"]
					PacketCommitmentNotFound,
					#[codec(index = 9)]
					#[doc = "Packet receipt wasn't found"]
					PacketReceiptNotFound,
					#[codec(index = 10)]
					#[doc = "Packet Acknowledgment wasn't found"]
					PacketAcknowledgmentNotFound,
					#[codec(index = 11)]
					#[doc = "Error constructing packet"]
					SendPacketError,
					#[codec(index = 12)]
					#[doc = "Invalid channel id"]
					InvalidChannelId,
					#[codec(index = 13)]
					#[doc = "Invalid port id"]
					InvalidPortId,
					#[codec(index = 14)]
					#[doc = "Other forms of errors"]
					Other,
					#[codec(index = 15)]
					#[doc = "Invalid route"]
					InvalidRoute,
					#[codec(index = 16)]
					#[doc = "Invalid message for extrinsic"]
					InvalidMessageType,
					#[codec(index = 17)]
					#[doc = "The interchain token transfer was not successfully initiated"]
					TransferFailed,
					#[codec(index = 18)]
					#[doc = "Error Decoding utf8 bytes"]
					Utf8Error,
					#[codec(index = 19)]
					#[doc = "Invalid asset id"]
					InvalidAssetId,
					#[codec(index = 20)]
					#[doc = "Invalid Ibc denom"]
					InvalidIbcDenom,
					#[codec(index = 21)]
					#[doc = "Invalid amount"]
					InvalidAmount,
					#[codec(index = 22)]
					#[doc = "Invalid timestamp"]
					InvalidTimestamp,
					#[codec(index = 23)]
					#[doc = "Unable to get client revision number"]
					FailedToGetRevisionNumber,
					#[codec(index = 24)]
					#[doc = "Invalid params passed"]
					InvalidParams,
					#[codec(index = 25)]
					#[doc = "Error opening channel"]
					ChannelInitError,
					#[codec(index = 26)]
					#[doc = "Latest height and timestamp for a client not found"]
					TimestampAndHeightNotFound,
					#[codec(index = 27)]
					#[doc = "Failed to derive channel escrow address"]
					ChannelEscrowAddress,
					#[codec(index = 28)]
					#[doc = "Error writing acknowledgement to storage"]
					WriteAckError,
					#[codec(index = 29)]
					#[doc = "Client update time and height not found"]
					ClientUpdateNotFound,
					#[codec(index = 30)]
					#[doc = "Error Freezing client"]
					ClientFreezeFailed,
					#[codec(index = 31)]
					#[doc = "Access denied"]
					AccessDenied,
					#[codec(index = 32)]
					RateLimiter,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Events emitted by the ibc subsystem"]
					Events {
						events: ::std::vec::Vec<
							::core::result::Result<
								runtime_types::pallet_ibc::events::IbcEvent,
								runtime_types::pallet_ibc::errors::IbcError,
							>,
						>,
					},
					#[codec(index = 1)]
					#[doc = "An Ibc token transfer has been started"]
					TokenTransferInitiated {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					#[doc = "A channel has been opened"]
					ChannelOpened {
						channel_id: ::std::vec::Vec<::core::primitive::u8>,
						port_id: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					#[doc = "Pallet params updated"]
					ParamsUpdated {
						send_enabled: ::core::primitive::bool,
						receive_enabled: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "An outgoing Ibc token transfer has been completed and burnt"]
					TokenTransferCompleted {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					#[doc = "Ibc tokens have been received and minted"]
					TokenReceived {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_receiver_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 6)]
					#[doc = "Ibc transfer failed, received an acknowledgement error, tokens have been refunded"]
					TokenTransferFailed {
						from: ::std::vec::Vec<::core::primitive::u8>,
						to: ::std::vec::Vec<::core::primitive::u8>,
						ibc_denom: ::std::vec::Vec<::core::primitive::u8>,
						local_asset_id:
							::core::option::Option<runtime_types::primitives::currency::CurrencyId>,
						amount: ::core::primitive::u128,
						is_sender_source: ::core::primitive::bool,
						source_channel: ::std::vec::Vec<::core::primitive::u8>,
						destination_channel: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 7)]
					#[doc = "On recv packet was not processed successfully processes"]
					OnRecvPacketError { msg: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 8)]
					#[doc = "Client upgrade path has been set"]
					ClientUpgradeSet,
					#[codec(index = 9)]
					#[doc = "Client has been frozen"]
					ClientFrozen {
						client_id: ::std::vec::Vec<::core::primitive::u8>,
						height: ::core::primitive::u64,
						revision_number: ::core::primitive::u64,
					},
					#[codec(index = 10)]
					#[doc = "Asset Admin Account Updated"]
					AssetAdminUpdated { admin_account: ::subxt::utils::AccountId32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Any {
				pub type_url: ::std::vec::Vec<::core::primitive::u8>,
				pub value: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum LightClientProtocol {
				#[codec(index = 0)]
				Beefy,
				#[codec(index = 1)]
				Grandpa,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiAddress<_0> {
				#[codec(index = 0)]
				Id(_0),
				#[codec(index = 1)]
				Raw(::std::vec::Vec<::core::primitive::u8>),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PalletParams {
				pub send_enabled: ::core::primitive::bool,
				pub receive_enabled: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferParams<_0> {
				pub to: runtime_types::pallet_ibc::MultiAddress<_0>,
				pub source_channel: ::core::primitive::u64,
				pub timeout: runtime_types::ibc_primitives::Timeout,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UpgradeParams {
				pub client_state: ::std::vec::Vec<::core::primitive::u8>,
				pub consensus_state: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Assign an previously unassigned index."]
					#[doc = ""]
					#[doc = "Payment: `Deposit` is reserved from the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be claimed. This must not be in use."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
					#[doc = "is effectively transferred to the new account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One transfer operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
					#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
					#[doc = "# </weight>"]
					transfer {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Free up an index owned by the sender."]
					#[doc = ""]
					#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
					#[doc = ""]
					#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
					#[doc = ""]
					#[doc = "Emits `IndexFreed` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
					#[doc = "held, then any deposit is reimbursed to its current owner."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be (re-)assigned."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
					#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
					#[doc = "# </weight>"]
					force_transfer {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
					#[doc = "deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
					#[doc = "non-frozen account `index`."]
					#[doc = ""]
					#[doc = "- `index`: the index to be frozen in place."]
					#[doc = ""]
					#[doc = "Emits `IndexFrozen` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one slash operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The index was not already assigned."]
					NotAssigned,
					#[codec(index = 1)]
					#[doc = "The index is assigned to another account."]
					NotOwner,
					#[codec(index = 2)]
					#[doc = "The index was not available."]
					InUse,
					#[codec(index = 3)]
					#[doc = "The source and destination accounts are identical."]
					NotTransfer,
					#[codec(index = 4)]
					#[doc = "The index is permanent and may not be freed/changed."]
					Permanent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A account index was assigned."]
					IndexAssigned {
						who: ::subxt::utils::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A account index has been freed up (unassigned)."]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "A account index has been frozen to its current account ID."]
					IndexFrozen { index: ::core::primitive::u32, who: ::subxt::utils::AccountId32 },
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Add a member `who` to the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::AddOrigin`."]
					add_member {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Remove a member `who` from the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::RemoveOrigin`."]
					remove_member {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 2)]
					#[doc = "Swap out one member `remove` for another `add`."]
					#[doc = ""]
					#[doc = "May only be called from `T::SwapOrigin`."]
					#[doc = ""]
					#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
					swap_member {
						remove: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						add: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
					#[doc = "pass `members` pre-sorted."]
					#[doc = ""]
					#[doc = "May only be called from `T::ResetOrigin`."]
					reset_members { members: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 4)]
					#[doc = "Swap out the sending member for some other key `new`."]
					#[doc = ""]
					#[doc = "May only be called from `Signed` origin of a current member."]
					#[doc = ""]
					#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
					change_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 5)]
					#[doc = "Set the prime member. Must be a current member."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					set_prime {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "Remove the prime member if it exists."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 2)]
					#[doc = "Too many members."]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The given member was added; see the transaction for who."]
					MemberAdded,
					#[codec(index = 1)]
					#[doc = "The given member was removed; see the transaction for who."]
					MemberRemoved,
					#[codec(index = 2)]
					#[doc = "Two members were swapped; see the transaction for who."]
					MembersSwapped,
					#[codec(index = 3)]
					#[doc = "The membership was reset; see the transaction for who the new set is."]
					MembersReset,
					#[codec(index = 4)]
					#[doc = "One of the members' keys changed."]
					KeyChanged,
					#[codec(index = 5)]
					#[doc = "Phantom member, never used."]
					Dummy,
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Register a preimage on-chain."]
					#[doc = ""]
					#[doc = "If the preimage was previously requested, no fees or deposits are taken for providing"]
					#[doc = "the preimage. Otherwise, a deposit is taken proportional to the size of the preimage."]
					note_preimage { bytes: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					#[doc = "Clear an unrequested preimage from the runtime storage."]
					#[doc = ""]
					#[doc = "If `len` is provided, then it will be a much cheaper operation."]
					#[doc = ""]
					#[doc = "- `hash`: The hash of the preimage to be removed from the store."]
					#[doc = "- `len`: The length of the preimage of `hash`."]
					unnote_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "Request a preimage be uploaded to the chain without paying any fees or deposits."]
					#[doc = ""]
					#[doc = "If the preimage requests has already been provided on-chain, we unreserve any deposit"]
					#[doc = "a user may have paid, and take the control of the preimage out of their hands."]
					request_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "Clear a previously made request for a preimage."]
					#[doc = ""]
					#[doc = "NOTE: THIS MUST NOT BE CALLED ON `hash` MORE TIMES THAN `request_preimage`."]
					unrequest_preimage { hash: ::subxt::utils::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Preimage is too large to store on-chain."]
					TooBig,
					#[codec(index = 1)]
					#[doc = "Preimage has already been noted on-chain."]
					AlreadyNoted,
					#[codec(index = 2)]
					#[doc = "The user is not authorized to perform this action."]
					NotAuthorized,
					#[codec(index = 3)]
					#[doc = "The preimage cannot be removed since it has not yet been noted."]
					NotNoted,
					#[codec(index = 4)]
					#[doc = "A preimage may not be removed when there are outstanding requests."]
					Requested,
					#[codec(index = 5)]
					#[doc = "The preimage request cannot be removed since no outstanding requests exist."]
					NotRequested,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A preimage has been noted."]
					Noted { hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					#[doc = "A preimage has been requested."]
					Requested { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "A preimage has ben cleared."]
					Cleared { hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RequestStatus<_0, _1> {
				#[codec(index = 0)]
				Unrequested { deposit: (_0, _1), len: ::core::primitive::u32 },
				#[codec(index = 1)]
				Requested {
					deposit: ::core::option::Option<(_0, _1)>,
					count: ::core::primitive::u32,
					len: ::core::option::Option<::core::primitive::u32>,
				},
			}
		}
		pub mod pallet_proxy {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
					#[doc = "`add_proxy`."]
					#[doc = ""]
					#[doc = "Removes any corresponding announcement(s)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
					#[doc = "- `call`: The call to be made by the `real` account."]
					proxy {
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
					#[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
					#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
					#[doc = "zero."]
					add_proxy {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Unregister a proxy account for the sender."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
					#[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
					remove_proxy {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Unregister all proxy accounts for the sender."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "WARNING: This may be called on accounts created by `pure`, however if done, then"]
					#[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
					remove_proxies,
					#[codec(index = 4)]
					#[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
					#[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
					#[doc = ""]
					#[doc = "Requires a `Signed` origin."]
					#[doc = ""]
					#[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
					#[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
					#[doc = "allow for maximum flexibility."]
					#[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
					#[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
					#[doc = "want to use `0`."]
					#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
					#[doc = "zero."]
					#[doc = ""]
					#[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
					#[doc = "same sender, with the same parameters."]
					#[doc = ""]
					#[doc = "Fails if there are insufficient funds to pay for deposit."]
					create_pure {
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec(index = 5)]
					#[doc = "Removes a previously spawned pure proxy."]
					#[doc = ""]
					#[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
					#[doc = "inaccessible."]
					#[doc = ""]
					#[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
					#[doc = "`pure` with corresponding parameters."]
					#[doc = ""]
					#[doc = "- `spawner`: The account that originally called `pure` to create this account."]
					#[doc = "- `index`: The disambiguation index originally passed to `pure`. Probably `0`."]
					#[doc = "- `proxy_type`: The proxy type originally passed to `pure`."]
					#[doc = "- `height`: The height of the chain when the call to `pure` was processed."]
					#[doc = "- `ext_index`: The extrinsic index in which the call to `pure` was processed."]
					#[doc = ""]
					#[doc = "Fails with `NoPermission` in case the caller is not a previously created pure"]
					#[doc = "account whose `pure` call has corresponding parameters."]
					kill_pure {
						spawner: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						index: ::core::primitive::u16,
						#[codec(compact)]
						height: ::core::primitive::u32,
						#[codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "Publish the hash of a proxy-call that will be made in the future."]
					#[doc = ""]
					#[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
					#[doc = "if the delay associated with the proxy relationship is greater than zero."]
					#[doc = ""]
					#[doc = "No more than `MaxPending` announcements may be made at any one time."]
					#[doc = ""]
					#[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
					#[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
					announce {
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 7)]
					#[doc = "Remove a given announcement."]
					#[doc = ""]
					#[doc = "May be called by a proxy account to remove a call they previously announced and return"]
					#[doc = "the deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
					remove_announcement {
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 8)]
					#[doc = "Remove the given announcement of a delegate."]
					#[doc = ""]
					#[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
					#[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `delegate`: The account that previously announced the call."]
					#[doc = "- `call_hash`: The hash of the call to be made."]
					reject_announcement {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 9)]
					#[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
					#[doc = "`add_proxy`."]
					#[doc = ""]
					#[doc = "Removes any corresponding announcement(s)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
					#[doc = "- `call`: The call to be made by the `real` account."]
					proxy_announced {
						delegate: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						real: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						force_proxy_type: ::core::option::Option<
							runtime_types::composable_traits::account_proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "There are too many proxies registered or too many announcements pending."]
					TooMany,
					#[codec(index = 1)]
					#[doc = "Proxy registration not found."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Sender is not a proxy of the account to be proxied."]
					NotProxy,
					#[codec(index = 3)]
					#[doc = "A call which is incompatible with the proxy type's filter was attempted."]
					Unproxyable,
					#[codec(index = 4)]
					#[doc = "Account is already a proxy."]
					Duplicate,
					#[codec(index = 5)]
					#[doc = "Call may not be made by proxy because it may escalate its privileges."]
					NoPermission,
					#[codec(index = 6)]
					#[doc = "Announcement, if made at all, was made too recently."]
					Unannounced,
					#[codec(index = 7)]
					#[doc = "Cannot add self as proxy."]
					NoSelfProxy,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A proxy was executed correctly, with the given."]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "A pure account has been created by new proxy with given"]
					#[doc = "disambiguation index and proxy type."]
					PureCreated {
						pure: ::subxt::utils::AccountId32,
						who: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					#[doc = "An announcement was placed to make a call in the future."]
					Announced {
						real: ::subxt::utils::AccountId32,
						proxy: ::subxt::utils::AccountId32,
						call_hash: ::subxt::utils::H256,
					},
					#[codec(index = 3)]
					#[doc = "A proxy was added."]
					ProxyAdded {
						delegator: ::subxt::utils::AccountId32,
						delegatee: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A proxy was removed."]
					ProxyRemoved {
						delegator: ::subxt::utils::AccountId32,
						delegatee: ::subxt::utils::AccountId32,
						proxy_type: runtime_types::composable_traits::account_proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Announcement<_0, _1, _2> {
				pub real: _0,
				pub call_hash: _1,
				pub height: _2,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ProxyDefinition<_0, _1, _2> {
				pub delegate: _0,
				pub proxy_type: _1,
				pub delay: _2,
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Anonymously schedule a task."]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "Cancel an anonymously scheduled task."]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Schedule a named task."]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "Cancel a named scheduled task."]
					cancel_named { id: [::core::primitive::u8; 32usize] },
					#[codec(index = 4)]
					#[doc = "Anonymously schedule a task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule`]."]
					#[doc = "# </weight>"]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "Schedule a named task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
					#[doc = "# </weight>"]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to schedule a call"]
					FailedToSchedule,
					#[codec(index = 1)]
					#[doc = "Cannot find the scheduled call."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Given target block number is in the past."]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					#[doc = "Reschedule failed because it does not change scheduled time."]
					RescheduleNoChange,
					#[codec(index = 4)]
					#[doc = "Attempt to use a non-named function on a named task."]
					Named,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Scheduled some task."]
					Scheduled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "Canceled some task."]
					Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Dispatched some task."]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "The call for the provided hash was not found so the task has been aborted."]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					#[doc = "The given task can never be executed since it is overweight."]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
					#[doc = "- DbWrites: `origin account`, `NextKeys`"]
					#[doc = "- DbReads per key id: `KeyOwner`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					set_keys {
						keys: runtime_types::composable_runtime::opaque::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
					#[doc = "  of `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
					#[doc = "- DbWrites: `NextKeys`, `origin account`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo { call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall> },
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- The weight of this call is defined by the caller."]
					#[doc = "# </weight>"]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB change."]
					#[doc = "# </weight>"]
					set_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo_as {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the Sudo pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
					KeyChanged { old_sudoer: ::core::option::Option<::subxt::utils::AccountId32> },
					#[codec(index = 2)]
					#[doc = "A sudo just took place. \\[result\\]"]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "`MinimumPeriod`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Inherent`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					#[doc = "# </weight>"]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
					#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
					#[doc = "proposal is awarded."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `ProposalCount`, `origin account`"]
					#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
					#[doc = "# </weight>"]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Reject a proposed spend. The original deposit will be slashed."]
					#[doc = ""]
					#[doc = "May only be called from `T::RejectOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
					#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
					#[doc = "# </weight>"]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
					#[doc = "and the original deposit will be returned."]
					#[doc = ""]
					#[doc = "May only be called from `T::ApproveOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)."]
					#[doc = "- DbReads: `Proposals`, `Approvals`"]
					#[doc = "- DbWrite: `Approvals`"]
					#[doc = "# </weight>"]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Propose and approve a spend of treasury funds."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`."]
					#[doc = "- `amount`: The amount to be transferred from the treasury to the `beneficiary`."]
					#[doc = "- `beneficiary`: The destination account for the transfer."]
					#[doc = ""]
					#[doc = "NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the"]
					#[doc = "beneficiary."]
					spend {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 4)]
					#[doc = "Force a previously approved proposal to be removed from the approval queue."]
					#[doc = "The original deposit will no longer be returned."]
					#[doc = ""]
					#[doc = "May only be called from `T::RejectOrigin`."]
					#[doc = "- `proposal_id`: The index of a proposal"]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(A) where `A` is the number of approvals"]
					#[doc = "- Db reads and writes: `Approvals`"]
					#[doc = "# </weight>"]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,"]
					#[doc = "i.e., the proposal has not been approved. This could also mean the proposal does not"]
					#[doc = "exist altogether, thus there is no way it would have been approved in the first place."]
					remove_approval {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the treasury pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "Too many approvals in the queue."]
					TooManyApprovals,
					#[codec(index = 3)]
					#[doc = "The spend origin is valid but the amount it is allowed to spend is lower than the"]
					#[doc = "amount to be spent."]
					InsufficientPermission,
					#[codec(index = 4)]
					#[doc = "Proposal has not been approved."]
					ProposalNotApproved,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New proposal."]
					Proposed { proposal_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "We have ended a spend period and will now allocate funds."]
					Spending { budget_remaining: ::core::primitive::u128 },
					#[codec(index = 2)]
					#[doc = "Some funds have been allocated."]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "A proposal was rejected; funds were slashed."]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some of our funds have been burnt."]
					Burnt { burnt_funds: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
					Rollover { rollover_balance: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some funds have been deposited."]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					#[doc = "A new spend proposal has been approved."]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Send a batch of dispatch calls."]
					#[doc = ""]
					#[doc = "May be called from any origin except `None`."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
					#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					#[doc = ""]
					#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
					#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
					#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
					#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
					#[doc = "event is deposited."]
					batch { calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall> },
					#[codec(index = 1)]
					#[doc = "Send a call through an indexed pseudonym of the sender."]
					#[doc = ""]
					#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
					#[doc = "use the same filter as the origin of this call."]
					#[doc = ""]
					#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
					#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
					#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
					#[doc = "in the Multisig pallet instead."]
					#[doc = ""]
					#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					#[doc = "Send a batch of dispatch calls and atomically execute them."]
					#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
					#[doc = ""]
					#[doc = "May be called from any origin except `None`."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
					#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "Dispatches a function call with a provided origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
					#[doc = "# </weight>"]
					dispatch_as {
						as_origin:
							::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					#[doc = "Send a batch of dispatch calls."]
					#[doc = "Unlike `batch`, it allows errors and won't interrupt."]
					#[doc = ""]
					#[doc = "May be called from any origin except `None`."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
					#[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::composable_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "Dispatch a function call with a specified weight."]
					#[doc = ""]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Root origin to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					with_weight {
						call: ::std::boxed::Box<runtime_types::composable_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many calls batched."]
					TooManyCalls,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
					#[doc = "well as the error."]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					#[doc = "Batch of dispatches completed fully with no error."]
					BatchCompleted,
					#[codec(index = 2)]
					#[doc = "Batch of dispatches completed but has errors."]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 4)]
					#[doc = "A single item within a Batch of dispatches has completed with error."]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec(index = 5)]
					#[doc = "A call was dispatched."]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Execute an XCM message from a local, signed, origin."]
					#[doc = ""]
					#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
					#[doc = "partially."]
					#[doc = ""]
					#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
					#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
					#[doc = "attempt will be made."]
					#[doc = ""]
					#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
					#[doc = "to completion; only that *some* of it was executed."]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					#[doc = "Extoll that a particular destination can be communicated with through a particular"]
					#[doc = "version of XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The destination that is being described."]
					#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
					#[doc = "version a destination can accept is unknown)."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
					#[doc = "version changes."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
					#[doc = "  notifications which we no longer desire."]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 9)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
					#[doc = "to it."]
					Unreachable,
					#[codec(index = 1)]
					#[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
					#[doc = "a lack of space for buffering the message."]
					SendFailure,
					#[codec(index = 2)]
					#[doc = "The message execution fails the filter."]
					Filtered,
					#[codec(index = 3)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 4)]
					#[doc = "The destination `MultiLocation` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 5)]
					#[doc = "The assets to be sent are empty."]
					Empty,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Too many assets have been attempted for transfer."]
					TooManyAssets,
					#[codec(index = 8)]
					#[doc = "Origin is invalid for sending."]
					InvalidOrigin,
					#[codec(index = 9)]
					#[doc = "The version of the `Versioned` value used is not able to be interpreted."]
					BadVersion,
					#[codec(index = 10)]
					#[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
					#[doc = "desired version of XCM)."]
					BadLocation,
					#[codec(index = 11)]
					#[doc = "The referenced subscription could not be found."]
					NoSubscription,
					#[codec(index = 12)]
					#[doc = "The location is invalid since it already has a subscription from us."]
					AlreadySubscribed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Execution of an XCM message was attempted."]
					#[doc = ""]
					#[doc = "\\[ outcome \\]"]
					Attempted(runtime_types::xcm::v2::traits::Outcome),
					#[codec(index = 1)]
					#[doc = "A XCM message was sent."]
					#[doc = ""]
					#[doc = "\\[ origin, destination, message \\]"]
					Sent(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v2::Xcm,
					),
					#[codec(index = 2)]
					#[doc = "Query response received which does not match a registered query. This may be because a"]
					#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
					#[doc = "because the query timed out."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					UnexpectedResponse(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
					#[doc = "no registered notification call."]
					#[doc = ""]
					#[doc = "\\[ id, response \\]"]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
					#[codec(index = 4)]
					#[doc = "Query response has been received and query is removed. The registered notification has"]
					#[doc = "been dispatched and executed successfully."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					#[doc = "Query response has been received and query is removed. The registered notification could"]
					#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
					#[doc = "originally budgeted by this runtime for the query result."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 6)]
					#[doc = "Query response has been received and query is removed. There was a general error with"]
					#[doc = "dispatching the notification call."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
					#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
					#[doc = "is not `(origin, QueryId, Response)`."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					#[doc = "Expected query response has been received but the origin location of the response does"]
					#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					#[doc = ""]
					#[doc = "\\[ origin location, id, expected location \\]"]
					InvalidResponder(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v1::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					#[doc = "Expected query response has been received but the expected origin location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					InvalidResponderVersion(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					#[doc = "Received query response has been read and removed."]
					#[doc = ""]
					#[doc = "\\[ id \\]"]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					#[doc = "Some assets have been placed in an asset trap."]
					#[doc = ""]
					#[doc = "\\[ hash, origin, assets \\]"]
					AssetsTrapped(
						::subxt::utils::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					#[doc = "An XCM version change notification message has been attempted to be sent."]
					#[doc = ""]
					#[doc = "\\[ destination, result \\]"]
					VersionChangeNotified(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 13)]
					#[doc = "The supported version of a location has been changed. This might be through an"]
					#[doc = "automatic notification or a manual intervention."]
					#[doc = ""]
					#[doc = "\\[ location, XCM version \\]"]
					SupportedVersionChanged(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "sending the notification to it."]
					#[doc = ""]
					#[doc = "\\[ location, query ID, error \\]"]
					NotifyTargetSendFail(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 15)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "migrating the location to our new XCM format."]
					#[doc = ""]
					#[doc = "\\[ location, query ID \\]"]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 16)]
					#[doc = "Some assets have been claimed from an asset trap"]
					#[doc = ""]
					#[doc = "\\[ hash, origin, assets \\]"]
					AssetsClaimed(
						::subxt::utils::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v1::multilocation::MultiLocation),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedMultiLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum XcmpMessageFormat {
					#[codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod primitives {
			use super::runtime_types;
			pub mod currency {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct CurrencyId(pub ::core::primitive::u128);
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Permill(pub ::core::primitive::u32);
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod bounded {
				use super::runtime_types;
				pub mod bounded_btree_set {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod weak_bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
			}
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Void {}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Header<_0, _1> {
						pub parent_hash: ::subxt::utils::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::utils::H256,
						pub extrinsics_root: ::subxt::utils::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
					);
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_runtime::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum TokenError {
				#[codec(index = 0)]
				NoFunds,
				#[codec(index = 1)]
				WouldDie,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct OldWeight(pub ::core::primitive::u64);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v0 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parent,
						#[codec(index = 1)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 2)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 3)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 4)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 5)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 6)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 7)]
						GeneralKey(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 8)]
						OnlyChild,
						#[codec(index = 9)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum NetworkId {
						#[codec(index = 0)]
						Any,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
					}
				}
				pub mod multi_asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiAsset {
						#[codec(index = 0)]
						None,
						#[codec(index = 1)]
						All,
						#[codec(index = 2)]
						AllFungible,
						#[codec(index = 3)]
						AllNonFungible,
						#[codec(index = 4)]
						AllAbstractFungible { id: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 5)]
						AllAbstractNonFungible { class: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 6)]
						AllConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 7)]
						AllConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 8)]
						AbstractFungible {
							id: ::std::vec::Vec<::core::primitive::u8>,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						AbstractNonFungible {
							class: ::std::vec::Vec<::core::primitive::u8>,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
						#[codec(index = 10)]
						ConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 11)]
						ConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
					}
				}
				pub mod multi_location {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiLocation {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v0::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Order {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						DepositAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							receive:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposit {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 2)]
					TeleportAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v0::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					RelayedFrom {
						who: runtime_types::xcm::v0::multi_location::MultiLocation,
						message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
					},
				}
			}
			pub mod v1 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v1::multiasset::AssetId,
						pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v1::multiasset::AssetId,
							fun: runtime_types::xcm::v1::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v1::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v1::multilocation::Junctions,
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Order {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						DepositAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							receive: runtime_types::xcm::v1::multiasset::MultiAssets,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v1::multiasset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposited {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 2)]
					ReceiveTeleportedAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v1::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					RelayedFrom {
						who: runtime_types::xcm::v1::multilocation::Junctions,
						message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
					},
					#[codec(index = 11)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 12)]
					UnsubscribeVersion,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::core::primitive::u64),
						#[codec(index = 1)]
						Incomplete(::core::primitive::u64, runtime_types::xcm::v2::traits::Error),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v2::traits::Error),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v1::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v1::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiAsset {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_asset::MultiAsset),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAsset),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiAssets {
				#[codec(index = 0)]
				V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAssets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiLocation {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_location::MultiLocation),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multilocation::MultiLocation),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedResponse {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Response),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Response),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedXcm {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Xcm),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Xcm),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
			}
		}
	}
	#[doc = r" The default error type returned when there is a runtime issue,"]
	#[doc = r" exposed here for ease of use."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn indices(&self) -> indices::constants::ConstantsApi {
			indices::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn authorship(&self) -> authorship::constants::ConstantsApi {
			authorship::constants::ConstantsApi
		}
		pub fn treasury(&self) -> treasury::constants::ConstantsApi {
			treasury::constants::ConstantsApi
		}
		pub fn democracy(&self) -> democracy::constants::ConstantsApi {
			democracy::constants::ConstantsApi
		}
		pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
			scheduler::constants::ConstantsApi
		}
		pub fn utility(&self) -> utility::constants::ConstantsApi {
			utility::constants::ConstantsApi
		}
		pub fn proxy(&self) -> proxy::constants::ConstantsApi {
			proxy::constants::ConstantsApi
		}
		pub fn x_tokens(&self) -> x_tokens::constants::ConstantsApi {
			x_tokens::constants::ConstantsApi
		}
		pub fn tokens(&self) -> tokens::constants::ConstantsApi {
			tokens::constants::ConstantsApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::constants::ConstantsApi {
			crowdloan_rewards::constants::ConstantsApi
		}
		pub fn assets(&self) -> assets::constants::ConstantsApi {
			assets::constants::ConstantsApi
		}
		pub fn call_filter(&self) -> call_filter::constants::ConstantsApi {
			call_filter::constants::ConstantsApi
		}
		pub fn ibc(&self) -> ibc::constants::ConstantsApi {
			ibc::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn randomness_collective_flip(
			&self,
		) -> randomness_collective_flip::storage::StorageApi {
			randomness_collective_flip::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn indices(&self) -> indices::storage::StorageApi {
			indices::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
			collator_selection::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn aura(&self) -> aura::storage::StorageApi {
			aura::storage::StorageApi
		}
		pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
			aura_ext::storage::StorageApi
		}
		pub fn council(&self) -> council::storage::StorageApi {
			council::storage::StorageApi
		}
		pub fn council_membership(&self) -> council_membership::storage::StorageApi {
			council_membership::storage::StorageApi
		}
		pub fn treasury(&self) -> treasury::storage::StorageApi {
			treasury::storage::StorageApi
		}
		pub fn democracy(&self) -> democracy::storage::StorageApi {
			democracy::storage::StorageApi
		}
		pub fn technical_committee(&self) -> technical_committee::storage::StorageApi {
			technical_committee::storage::StorageApi
		}
		pub fn technical_committee_membership(
			&self,
		) -> technical_committee_membership::storage::StorageApi {
			technical_committee_membership::storage::StorageApi
		}
		pub fn release_committee(&self) -> release_committee::storage::StorageApi {
			release_committee::storage::StorageApi
		}
		pub fn release_membership(&self) -> release_membership::storage::StorageApi {
			release_membership::storage::StorageApi
		}
		pub fn scheduler(&self) -> scheduler::storage::StorageApi {
			scheduler::storage::StorageApi
		}
		pub fn preimage(&self) -> preimage::storage::StorageApi {
			preimage::storage::StorageApi
		}
		pub fn proxy(&self) -> proxy::storage::StorageApi {
			proxy::storage::StorageApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
			xcmp_queue::storage::StorageApi
		}
		pub fn relayer_xcm(&self) -> relayer_xcm::storage::StorageApi {
			relayer_xcm::storage::StorageApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi {
			dmp_queue::storage::StorageApi
		}
		pub fn unknown_tokens(&self) -> unknown_tokens::storage::StorageApi {
			unknown_tokens::storage::StorageApi
		}
		pub fn tokens(&self) -> tokens::storage::StorageApi {
			tokens::storage::StorageApi
		}
		pub fn currency_factory(&self) -> currency_factory::storage::StorageApi {
			currency_factory::storage::StorageApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::storage::StorageApi {
			crowdloan_rewards::storage::StorageApi
		}
		pub fn governance_registry(&self) -> governance_registry::storage::StorageApi {
			governance_registry::storage::StorageApi
		}
		pub fn call_filter(&self) -> call_filter::storage::StorageApi {
			call_filter::storage::StorageApi
		}
		pub fn ibc(&self) -> ibc::storage::StorageApi {
			ibc::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn indices(&self) -> indices::calls::TransactionApi {
			indices::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn authorship(&self) -> authorship::calls::TransactionApi {
			authorship::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn council(&self) -> council::calls::TransactionApi {
			council::calls::TransactionApi
		}
		pub fn council_membership(&self) -> council_membership::calls::TransactionApi {
			council_membership::calls::TransactionApi
		}
		pub fn treasury(&self) -> treasury::calls::TransactionApi {
			treasury::calls::TransactionApi
		}
		pub fn democracy(&self) -> democracy::calls::TransactionApi {
			democracy::calls::TransactionApi
		}
		pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi {
			technical_committee::calls::TransactionApi
		}
		pub fn technical_committee_membership(
			&self,
		) -> technical_committee_membership::calls::TransactionApi {
			technical_committee_membership::calls::TransactionApi
		}
		pub fn release_committee(&self) -> release_committee::calls::TransactionApi {
			release_committee::calls::TransactionApi
		}
		pub fn release_membership(&self) -> release_membership::calls::TransactionApi {
			release_membership::calls::TransactionApi
		}
		pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
			scheduler::calls::TransactionApi
		}
		pub fn utility(&self) -> utility::calls::TransactionApi {
			utility::calls::TransactionApi
		}
		pub fn preimage(&self) -> preimage::calls::TransactionApi {
			preimage::calls::TransactionApi
		}
		pub fn proxy(&self) -> proxy::calls::TransactionApi {
			proxy::calls::TransactionApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
			xcmp_queue::calls::TransactionApi
		}
		pub fn relayer_xcm(&self) -> relayer_xcm::calls::TransactionApi {
			relayer_xcm::calls::TransactionApi
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
			cumulus_xcm::calls::TransactionApi
		}
		pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi {
			dmp_queue::calls::TransactionApi
		}
		pub fn x_tokens(&self) -> x_tokens::calls::TransactionApi {
			x_tokens::calls::TransactionApi
		}
		pub fn unknown_tokens(&self) -> unknown_tokens::calls::TransactionApi {
			unknown_tokens::calls::TransactionApi
		}
		pub fn tokens(&self) -> tokens::calls::TransactionApi {
			tokens::calls::TransactionApi
		}
		pub fn currency_factory(&self) -> currency_factory::calls::TransactionApi {
			currency_factory::calls::TransactionApi
		}
		pub fn crowdloan_rewards(&self) -> crowdloan_rewards::calls::TransactionApi {
			crowdloan_rewards::calls::TransactionApi
		}
		pub fn assets(&self) -> assets::calls::TransactionApi {
			assets::calls::TransactionApi
		}
		pub fn governance_registry(&self) -> governance_registry::calls::TransactionApi {
			governance_registry::calls::TransactionApi
		}
		pub fn call_filter(&self) -> call_filter::calls::TransactionApi {
			call_filter::calls::TransactionApi
		}
		pub fn ibc(&self) -> ibc::calls::TransactionApi {
			ibc::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash !=
			[
				57u8, 244u8, 125u8, 185u8, 18u8, 39u8, 249u8, 210u8, 33u8, 111u8, 248u8, 237u8,
				38u8, 124u8, 94u8, 196u8, 201u8, 147u8, 13u8, 2u8, 225u8, 135u8, 93u8, 103u8,
				239u8, 157u8, 1u8, 100u8, 214u8, 94u8, 210u8, 192u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
}
